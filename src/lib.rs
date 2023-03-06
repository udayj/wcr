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
                    .author("udayj")
                    .about("Rust wc")
                    .arg(
                        Arg::with_name("files")
                            .value_name("FILES")
                            .help("Input file(s)")
                            .multiple(true)
                            .default_value("-")
                    )
                    .arg(

                        Arg::with_name("lines")
                            .short("l")
                            .long("lines")
                            .help("Show line count")
                            .takes_value(false)

                    )
                    .arg(
                        Arg::with_name("words")
                            .short("w")
                            .long("words")
                            .help("Show word count")
                            .takes_value(false)
                    )
                    .arg(
                        Arg::with_name("bytes")
                            .short("c")
                            .long("bytes")
                            .help("Show byte count")
                            .takes_value(false)
                    )
                    .arg(

                        Arg::with_name("chars")
                            .short("m")
                            .long("chars")
                            .help("Show character count")
                            .takes_value(false)
                            .conflicts_with("bytes")
                    )
                    .get_matches();
    
    let files = matches.values_of_lossy("files").unwrap();
    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {

        lines = true;
        words = true;
        bytes = true;
    }
    Ok (
        Config {
            files,
            lines,
            words,
            bytes,
            chars,
        }
    )

}