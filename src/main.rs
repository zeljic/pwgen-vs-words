#[macro_use]
extern crate clap;

use std::{fs::OpenOptions, io::{self, BufRead, BufReader, Read}, path::Path, process::Command};

use clap::{App, Arg};

mod error;

type Result<T> = std::result::Result<T, error::GenericError>;

fn read_dictionary(dictionary: &str, length: usize) -> Result<Vec<String>> {
    match OpenOptions::new().read(true).open(Path::new(dictionary)) {
        Ok(source) => Ok(BufReader::new(source)
            .lines()
            .filter_map(|wr| match wr {
                Ok(ref wr_result) => {
                    if wr_result.len() == length {
                        Some(wr_result.to_lowercase())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<String>>()),
        Err(..) => Err(error::GenericError {
            message: "Unable to read dictionary".to_string(),
            kind: error::GenericErrorKind::WORDS,
        }),
    }
}

fn exec_pwgen(length: usize, size: usize) -> Result<Vec<String>> {
    match Command::new("pwgen")
        .args(&["-A", "-0", &length.to_string(), &size.to_string()])
        .output()
    {
        Ok(result) => Ok(String::from_utf8_lossy(&result.stdout)
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>()),

        Err(..) => Err(error::GenericError {
            message: "Unable to generate pwgen data".to_string(),
            kind: error::GenericErrorKind::PWGEN,
        }),
    }
}

fn read_pipe(chars: usize) -> Result<Vec<String>> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    match handle.read_to_string(&mut buf) {
        Ok(..) => Ok(buf.split_whitespace()
            .filter(|item| item.len() == chars)
            .map(String::from)
            .collect::<Vec<String>>()),
        Err(..) => Err(error::GenericError {
            message: "Unable to read stdin".to_string(),
            kind: error::GenericErrorKind::PIPEREAD,
        }),
    }
}

fn operate(list: &[String], words: &[String]) {
    list.iter()
        .filter(|item| words.contains(item))
        .for_each(|item| println!("{}", item));
}

fn main() {
    let arg_chars: Arg = Arg::with_name("chars")
        .short("c")
        .long("chars")
        .takes_value(true)
        .required(true);

    let arg_words: Arg = Arg::with_name("words")
        .short("w")
        .long("words")
        .takes_value(true)
        .required(true);

    let arg_gen: Arg = Arg::with_name("generate")
        .short("g")
        .long("generate")
        .conflicts_with("pipe")
        .takes_value(true);

    let arg_pipe: Arg = Arg::with_name("pipe")
        .short("p")
        .long("pipe")
        .conflicts_with("generate")
        .takes_value(false);

    let args = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .args(&[arg_chars, arg_words, arg_gen, arg_pipe])
        .get_matches();

    let chars: usize = value_t_or_exit!(args.value_of("chars"), usize);
    let generate: usize = value_t!(args.value_of("generate"), usize).unwrap_or_else(|_e| 0);
    let words_path: &str = args.value_of("words").unwrap();
    let pipe: bool = args.is_present("pipe");

    match read_dictionary(words_path, chars) {
        Ok(words) => {
            if pipe {
                match read_pipe(chars) {
                    Ok(list) => {
                        operate(&list, &words);
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            } else {
                match exec_pwgen(chars, generate) {
                    Ok(list) => {
                        operate(&list, &words);
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
