#[macro_use]
extern crate clap;
extern crate rayon;

use clap::{App, Arg};
use rayon::prelude::*;
use std::{fs::OpenOptions, io::{self, BufRead, BufReader, Read}, path::Path, process::Command};

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
            kind: error::GenericErrorKind::DICTIONARY,
        }),
    }
}

fn exec_pwgen(length: usize, size: usize) -> Result<Vec<String>> {
    match Command::new("pwgen")
        .args(&["-A", "-0", &length.to_string(), &size.to_string()])
        .output()
    {
        Ok(result) => Ok(String::from_utf8_lossy(&result.stdout)
            .par_split_whitespace()
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
        Ok(..) => Ok(buf.par_split_whitespace()
            .filter(|item| item.len() == chars)
            .map(String::from)
            .collect::<Vec<String>>()),
        Err(..) => Err(error::GenericError {
            message: "Unable to read stdin".to_string(),
            kind: error::GenericErrorKind::PIPEREAD,
        }),
    }
}

fn main() {
    let arg_letters: Arg = Arg::with_name("letters")
        .short("l")
        .long("letters")
        .takes_value(true)
        .required(true);

    let arg_dictionary: Arg = Arg::with_name("dictionary")
        .short("d")
        .long("dictionary")
        .takes_value(true)
        .required(true);

    let arg_count: Arg = Arg::with_name("count")
        .short("c")
        .long("count")
        .conflicts_with("pipe")
        .takes_value(true);

    let arg_pipe: Arg = Arg::with_name("pipe")
        .short("p")
        .long("pipe")
        .conflicts_with("count")
        .takes_value(false);

    let args = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .args(&[arg_letters, arg_dictionary, arg_count, arg_pipe])
        .get_matches();

    let letters: usize = value_t_or_exit!(args.value_of("letters"), usize);
    let count: usize = value_t!(args.value_of("count"), usize).unwrap_or_else(|_e| 0);
    let words: String = value_t_or_exit!(args.value_of("dictionary"), String);
    let pipe: bool = args.is_present("pipe");

    match read_dictionary(&words, letters) {
        Ok(words) => match if pipe {
            read_pipe(letters)
        } else {
            exec_pwgen(letters, count)
        } {
            Ok(list) => {
                list.par_iter()
                    .filter(|item| words.contains(item))
                    .for_each(|item| println!("{}", item));
            }
            Err(e) => {
                println!("{}", e);
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
