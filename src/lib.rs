use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("files")
                .default_value("-")
                .min_values(1)
                .help("Input file(s)")
                .required(false)
        )
        .arg(
            Arg::with_name("chars")
                .long("--chars")
                .short("-m")
                .help("Show character count")
                .required(false)
                .takes_value(false)
        )
        .arg(
            Arg::with_name("bytes")
                .long("--bytes")
                .short("-c")
                .help("Show byte count")
                .required(false)
                .conflicts_with("chars")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("lines")
                .long("--lines")
                .short("-l")
                .help("Show line count")
                .required(false)
                .takes_value(false)
        )
        .arg(
            Arg::with_name("words")
                .long("--words")
                .short("-w")
                .help("Show word count")
                .required(false)
                .takes_value(false)
        )
        .get_matches();

    Ok(Config { 
        files: matches.values_of_lossy("files").unwrap(), 
        lines: matches.is_present("lines"), 
        words: matches.is_present("words"), 
        bytes: matches.is_present("bytes"), 
        chars: matches.is_present("chars") })
}


pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}