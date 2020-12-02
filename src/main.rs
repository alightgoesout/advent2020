use std::env;
use std::io::Result;

mod puzzle;

fn main() -> Result<()> {
    if let Some(day) = get_day_from_arguments() {
        execute_puzzle(day);
    } else {
        println!("Missing day argument");
    }
    Ok(())
}

fn get_day_from_arguments() -> Option<String> {
    env::args().nth(1)
}

fn execute_puzzle(day: String) {
    match day.as_str() {
        "1" => puzzle::day1::execute(),
        "2" => puzzle::day2::execute(),
        _ => println!("Unknown day: {}", day),
    }
}
