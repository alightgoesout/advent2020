use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Result};

pub fn read_lines(day: &str) -> Result<Vec<String>> {
    let file = File::open(format!("src/puzzle/input/{}", day))?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}

pub fn read_input(day: &str) -> String {
    read_to_string(format!("src/puzzle/input/{}", day))
        .unwrap_or_else(|_| panic!("Could not read input file for {}", day))
}
