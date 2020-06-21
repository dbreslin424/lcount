extern crate glob;

use glob::glob;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

const VERBOSE: bool = false;

fn main() {
    let mut file = String::new();

    println!("Enter file glob string:");

    io::stdin()
        .read_line(&mut file)
        .expect("Failed to read line");

    println!("Finding files: {}", file);

    let (total, files) = get_total_lines(&file.trim());

    println!("File count: {}", files);
    println!("Total lines: {}", total);
}

fn get_total_lines(file: &str) -> (i32, i32) {
    let mut file_count = 0;
    let mut total_lines = 0;

    for entry in glob(&file).unwrap() {
        match entry {
            Ok(path) => {
                file_count += 1;
                total_lines += get_line_count(&path);
            }
            Err(e) => println!("Error reading file: {}", e),
        }
    }

    (total_lines, file_count)
}

fn get_line_count(path: &PathBuf) -> i32 {
    let mut count = 0;

    let display = path.display();

    let contents = match File::open(&path) {
        Ok(file) => (file),
        Err(why) => panic!("Error reading file {}: {}", display, why),
    };

    let reader = BufReader::new(contents);

    for _line in reader.lines() {
        count += 1;
    }

    if VERBOSE {
        println!("{}: {}", display, count)
    }

    count
}
