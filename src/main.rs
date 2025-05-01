use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};
use clap::{Parser};

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
    #[arg(short='e', long, default_value_t=1)]
    starting_episode: usize,
    /// A file containing the links on separate lines
    #[arg(short, long)]
    file: Option<PathBuf>
    // Optional name to operate on
    // directory: Option<String>
}

fn main() {
    let cli = Cli::parse();

    let mut episodes:Vec<String> = Vec::new();

    match cli.file {
        None => {
            println!("Paste episode links one after the other, start download with \'download\'");
            loop {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        match input.as_str().trim() {
                            "download" => break,
                            _ => episodes.push(input)
                        }
                    }
                    Err(_) => {}
                }
            }

            if cli.save_links.unwrap_or(false) {
                match File::create(format!("{}-links.txt", cli.series_name)) {
                    Ok(mut file) => {
                        match file.write_all(episodes.join("").as_ref()) {
                            Ok(_) => println!("Successfully saved links to file."),
                            Err(_) => println!("Could not create link file.")
                        };
                    }
                    Err(_) => println!("Could not create link file.")
                };
            }
        }
        Some(path) => {
            match std::fs::read_to_string(path) {
                Ok(file) => {
                    for line in file.lines() {
                        episodes.push(line.to_string());
                    }
                }
                Err(_) => {
                    println!("Invalid file");
                    exit(1)
                }
            };
        }
    }

    for (index, episode) in episodes.iter().enumerate() {
        let title: String;

        if cli.season.is_some() {
            title = format!("{} ~ {}-{}.mp4", cli.series_name, cli.season.unwrap_or(1), index + cli.starting_episode);
        } else {
            title = format!("{} ~ {}.mp4", cli.series_name, index + cli.starting_episode);
        }

        // yt-dlp -S "ext" -N 25 -o "<title>.mp4" <link>
        let mut child = Command::new("yt-dlp").args(["-S", "ext", "-N", "50", "-o", &*title, "-q", "--progress", "--no-warnings", episode]).spawn().unwrap();
        let _ = child.wait();
        println!("Successfully downloaded \'{}\'", title)
    }

    println!("Done!")
}
