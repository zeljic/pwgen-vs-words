#[macro_use]
extern crate clap;

use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;
use std::process::Command;
use std::path::Path;

use clap::{App, Arg};

fn read_dictionary(dictionary: &str, length: usize) -> Vec<String> {
    match OpenOptions::new().read(true).open(Path::new(dictionary)) {
        Ok(source) => BufReader::new(source)
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
            .collect::<Vec<String>>(),
        Err(e) => {
            println!("Unable to open dictionary file.");
            eprintln!("ERROR: {}", e);

            vec![]
        }
    }
}

fn exec_pwgen(length: usize, size: usize) -> Vec<String> {
    match Command::new("pwgen")
        .args(&["-A", "-0", &length.to_string(), &size.to_string()])
        .output()
        {
            Ok(result) => String::from_utf8_lossy(&result.stdout)
                .split_whitespace()
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>(),

            Err(e) => {
                println!("Unable to generate pwgen data");
                eprintln!("ERROR: {}", e);

                vec![]
            }
        }
}

fn main() {
    let arg_characters: Arg = Arg::with_name("characters")
        .short("c")
        .long("characters")
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
        .takes_value(true)
        .required(true);

    let args = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(arg_characters)
        .arg(arg_words)
        .arg(arg_gen)
        .get_matches();

    let size: usize = value_t_or_exit!(args.value_of("characters"), usize);
    let gen: usize = value_t_or_exit!(args.value_of("generate"), usize);
    let words_path = args.value_of("words").unwrap();

    let words = read_dictionary(words_path, size);

    exec_pwgen(size, gen)
        .into_iter()
        .filter(|gen| words.contains(gen))
        .for_each(|item| println!("{}", item));
}
