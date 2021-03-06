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
        "3" => puzzle::day3::execute(),
        "4" => puzzle::day4::execute(),
        "5" => puzzle::day5::execute(),
        "6" => puzzle::day6::execute(),
        "7" => puzzle::day7::execute(),
        "8" => puzzle::day8::execute(),
        "9" => puzzle::day9::execute(),
        "10" => puzzle::day10::execute(),
        "11" => puzzle::day11::execute(),
        "12" => puzzle::day12::execute(),
        "13" => puzzle::day13::execute(),
        "14" => puzzle::day14::execute(),
        "15" => puzzle::day15::execute(),
        "16" => puzzle::day16::execute(),
        "17" => puzzle::day17::execute(),
        _ => println!("Unknown day: {}", day),
    }
}
