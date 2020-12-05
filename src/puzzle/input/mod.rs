use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn read_lines(day: &str) -> Result<Vec<String>> {
    let file = File::open(format!("src/puzzle/input/{}", day))?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}
