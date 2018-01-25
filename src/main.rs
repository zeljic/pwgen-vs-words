use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;
use std::process::Command;
use std::path::Path;

fn read_dictionary(dictionary: &str, length: usize) -> Vec<String> {
    match OpenOptions::new()
        .read(true)
        .open(Path::new(dictionary)) {
        Ok(source) => {
            BufReader::new(source)
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
                .collect::<Vec<String>>()
        }
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
            Ok(result) => {
                String::from_utf8_lossy(&result.stdout)
                    .split_whitespace()
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            }

            Err(e) => {
                println!("Unable to generate pwgen data");
                eprintln!("ERROR: {}", e);

                vec![]
            }
        }
}

fn main() {
    let size = 6;

    let words = read_dictionary("words.txt", size);

    exec_pwgen(size, 4 * 1024)
        .into_iter()
        .filter(|gen| words.contains(gen))
        .for_each(|item| println!("{}", item));
}
