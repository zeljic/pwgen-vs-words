#[macro_use]
extern crate clap;

use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;
use std::process::Command;
use std::path::Path;

use clap::{App, Arg};

mod error;

type Result<T> = std::result::Result<T, error::GeneralError>;

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
        Err(..) => Err(error::GeneralError {
            message: "Unable to read dictionary".to_string(),
            kind: error::GeneralErrorKind::WORDS,
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
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()),

        Err(..) => Err(error::GeneralError {
            message: "Unable to generate pwgen data".to_string(),
            kind: error::GeneralErrorKind::PWGEN,
        }),
    }
}

fn main() {
    let arg_characters: Arg = Arg::with_name("characters")
        .short("c")
        .long("characters")
        .takes_value(true)
        .required(true)
        .help("");

    let arg_words: Arg = Arg::with_name("words")
        .short("w")
        .long("words")
        .takes_value(true)
        .required(true)
        .help("");

    let arg_gen: Arg = Arg::with_name("generate")
        .short("g")
        .long("generate")
        .takes_value(true)
        .required(true)
        .help("");

    let args = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(&arg_characters)
        .arg(&arg_words)
        .arg(&arg_gen)
        .get_matches();

    let characters: usize = value_t_or_exit!(args.value_of("characters"), usize);
    let generate: usize = value_t_or_exit!(args.value_of("generate"), usize);
    let words_path: &str = args.value_of("words").unwrap();

    match read_dictionary(words_path, characters) {
        Ok(words) => match exec_pwgen(characters, generate) {
            Ok(generated_list) => {
                generated_list
                    .iter()
                    .filter(|generated_item| words.contains(generated_item))
                    .for_each(|found_item| println!("{}", found_item));
            }
            Err(e) => {
                println!("{}", e);
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    };
}
