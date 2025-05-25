# Single Link Series Downloader

A tool for downloading a series of single link episodes with a series naming scheme (`<series_name> ~ [season-]<episode>`).
This tool uses [yt-dlp](https://github.com/yt-dlp/yt-dlp), so it will assume you have it installed.

Type `series_downloader.exe --help` to see all command line options.

```
Usage: series_downloader.exe [OPTIONS] <SERIES_NAME> [SAVE_LINKS]

Arguments:
  <SERIES_NAME>  The name of the series
  [SAVE_LINKS]   Whether to save the links to file [possible values: true, false]

Options:
  -s, --season <SEASON>                      A season number for the title
  -e, --starting-episode <STARTING_EPISODE>  The starting number for episode numbering [default: 1]
  -f, --file <FILE>                          A file containing the links on separate lines
  -h, --help                                 Print help
  -V, --version                              Print version
```

# TODO
- Update links file if in file mode (only remove successful downloads)