use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::zip;

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

    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
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

fn open(filename: &str) -> MyResult<Box <dyn BufRead>> {

    match filename {

        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
pub fn run(config: Config) -> MyResult<()> {

    
    let mut total_num_lines = 0;
    let mut total_num_words = 0;
    let mut total_num_bytes = 0;
    let mut total_num_chars = 0;
    let mut new_total_str = String::new();

    for filename in &config.files {

        match open(filename)  {

            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(reader) => {

                let result = count(reader)?;
                let mut new_str = String::new();
                
                total_num_lines += result.num_lines;
                total_num_words += result.num_words;
                total_num_bytes += result.num_bytes;
                total_num_chars += result.num_chars;

                zip([config.lines, config.words, config.bytes, config.chars],
                    [result.num_lines, result.num_words, result.num_bytes, result.num_chars]).for_each(

                    |v| {

                        if v.0 {
                            new_str.push_str(format!("{:>8}",v.1 ).as_str());
                        }
                    }
                );

                
                if filename!="-" { 
                    println!("{} {}", new_str, filename);
                }
                else {
                    println!("{}", new_str);
                }
                    
               
            }
        }
    }
    zip([config.lines, config.words, config.bytes, config.chars],
        [total_num_lines, total_num_words, total_num_bytes, total_num_chars]).for_each(

        |v| {

            if v.0 {
                new_total_str.push_str(format!("{:>8}",v.1 ).as_str());
            }
        }
    );
    if (&config.files.len() > &1) {
        println!("{} total", new_total_str);
    }
    Ok(())
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer);
    num_lines += buffer.lines().count();
    num_words += buffer.split_whitespace().count();
    num_bytes += buffer.bytes().count();   
    num_chars += buffer.chars().count();
 


    Ok(
        FileInfo {
            num_lines,
            num_words,
            num_bytes,
            num_chars,
        }
    )
}

#[cfg(test)]
mod tests {

    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count () {

        let text = "I don't want the world. I just want your half.\r\n";

        let info = count(Cursor::new(text));

        assert!(info.is_ok());

        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }
}