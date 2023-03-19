use clap::{App, Arg};
use std::{error::Error, io::{BufRead, BufReader, self}, fs::File};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    pub num_lines: usize,
    pub num_words: usize,
    pub num_bytes: usize,
    pub num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("files")
                .help("Input file(s)")
                .default_value("-")
                .multiple(true)
                
        )
        .arg(
            Arg::with_name("chars")
                .long("--chars")
                .short("-m")
                .help("Show character count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("bytes")
                .long("--bytes")
                .short("-c")
                .help("Show byte count")
                .conflicts_with("chars")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("lines")
                .long("--lines")
                .short("-l")
                .help("Show line count")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("words")
                .long("--words")
                .short("-w")
                .help("Show word count")
                .takes_value(false)
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config { 
        files: matches.values_of_lossy("files").unwrap(), 
        lines,
        words,
        bytes,
        chars, 
    })
}


pub fn open(filename: &str) -> MyResult<Box< dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    Ok(FileInfo { 
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for file in &config.files {
        match open(file) {
            Err(e) => eprintln!("{}", e),
            Ok(_) => println!("Opened {}", file),
        }
    }
    Ok(())
}