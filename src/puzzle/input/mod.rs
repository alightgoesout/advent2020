use im_rc::Vector;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Result};

pub fn read_input(day: &str) -> String {
    read_to_string(format!("src/puzzle/input/{}", day))
        .expect(format!("Could not read input file for {}", day).as_ref())
}

pub fn read_lines(day: &str) -> Result<Vector<String>> {
    let file = File::open(format!("src/puzzle/input/{}", day))?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}
