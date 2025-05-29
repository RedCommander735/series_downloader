use clap::Parser;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, exit};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The name of the series
    series_name: String,
    /// Whether to save the links to file
    save_links: Option<bool>,
    /// A season number for the title
    #[arg(short, long)]
    season: Option<u32>,
    /// The starting number for episode numbering
    #[arg(short = 'e', long, default_value_t = 1)]
    starting_episode: usize,
    /// A file containing the links on separate lines
    #[arg(short, long)]
    file: Option<PathBuf>, // Optional name to operate on
                           // directory: Option<String>
}

#[derive(Clone)]
struct Link {
    link: String,
    success: bool,
}

impl ToString for Link {
    fn to_string(&self) -> String {
        self.link.clone()
    }
}

fn main() {
    let cli = Cli::parse();

    let mut episodes: Vec<Link> = Vec::new();
    let mut file_mode: bool = cli.save_links.unwrap_or(false);

    match cli.file {
        None => {
            println!("Paste episode links one after the other, start download with \'download\'");
            loop {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => match input.as_str().trim() {
                        "download" => break,
                        _ => episodes.push(Link {
                            link: input,
                            success: false,
                        }),
                    },
                    Err(_) => {}
                }
            }

            if file_mode {
                link_saver(&cli.series_name, &episodes, false);
            }
        }
        Some(path) => {
            match std::fs::read_to_string(path) {
                Ok(file) => {
                    file_mode = true;
                    for line in file.lines() {
                        episodes.push(Link {
                            link: line.to_string(),
                            success: false,
                        });
                    }
                }
                Err(_) => {
                    eprintln!("Invalid file");
                    exit(1)
                }
            };
        }
    }

    for (index, episode) in episodes.clone().iter().enumerate() {
        let title: String;

        if episode.link.trim().is_empty() {
            continue;
        }

        if cli.season.is_some() {
            title = format!(
                "{} S{:0>2}E{:0>2}.mp4",
                cli.series_name,
                cli.season.unwrap_or(1),
                index + cli.starting_episode
            );
        } else {
            title = format!(
                "{} E{:0>2}.mp4",
                cli.series_name,
                index + cli.starting_episode
            );
        }

        // yt-dlp -S "ext" -N 25 -o "<title>.mp4" <link>
        let mut child = Command::new("yt-dlp")
            .args([
                "-S",
                "ext",
                "-N",
                "50",
                "-o",
                &*title,
                "-q",
                "--progress",
                "--no-warnings",
                &episode.link,
            ])
            .spawn()
            .unwrap();

        let result = child.wait();

        match result {
            Ok(exit_status) => {
                if exit_status.success() {
                    println!("Successfully downloaded \'{}\'", title);
                    episodes[index].success = true;
                    if file_mode {
                        link_saver(&cli.series_name, &episodes, true);
                    }
                } else {
                    eprintln!(
                        "An error occured while downloading \'{}\'. Exit code {}",
                        title,
                        exit_status.to_string()
                    )
                }
            }
            Err(_) => eprintln!("An error occured while downloading \'{}\'", title),
        }
    }

    println!("Done!")
}

fn link_saver(series_name: &str, episodes: &Vec<Link>, update: bool) {
    match File::create(format!("{}-links.txt", series_name)) {
        Ok(mut file) => {
            match file.write_all(
                episodes
                    .iter()
                    .fold("".to_string(), |acc, x| if x.success { acc + "\n" } else { acc + &x.link } )
                    .as_ref(),
            ) {
                Ok(_) => if update { println!("Successfully updated links file.") } else { println!("Successfully saved links to file.") },
                Err(_) => if update { eprintln!("Could not update links file.")  } else { eprintln!("Could not create links file.") },
            };
        }
        Err(_) => if update { eprintln!("Could not update links file.")  } else { eprintln!("Could not create links file.") },
    };
}
