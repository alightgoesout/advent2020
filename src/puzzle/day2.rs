use im_rc::Vector;
use lazy_static::lazy_static;
use regex::Regex;

use crate::puzzle::input::read_lines;

pub fn execute() {
    let entries = read_lines("day2").unwrap();
    println!(
        "2:1 — Number of valid passwords: {}",
        count_valid_passwords(entries)
    );
}

fn count_valid_passwords(entries: Vector<String>) -> usize {
    entries
        .iter()
        .map(|entry| parse_password_line(entry.as_str()))
        .filter(|(password, policy)| policy.is_valid(password.as_str()))
        .count()
}

fn parse_password_line(password_line: &str) -> (String, PasswordPolicy) {
    let (password, letter, start, end) = split_password_line(password_line);
    (password, PasswordPolicy::new(letter, start, end))
}

fn split_password_line(password_line: &str) -> (String, char, u32, u32) {
    let captures = PASSWORD_LINE_REGEX
        .captures(password_line)
        .unwrap_or_else(|| panic!("Invalid password line: {}", password_line));
    (
        captures.name("password").unwrap().as_str().into(),
        captures
            .name("letter")
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap(),
        captures.name("start").unwrap().as_str().parse().unwrap(),
        captures.name("end").unwrap().as_str().parse().unwrap(),
    )
}

lazy_static! {
    static ref PASSWORD_LINE_REGEX: Regex =
        Regex::new(r"(?P<start>\d+)-(?P<end>\d+) (?P<letter>[[:alpha:]]): (?P<password>.*)")
            .unwrap();
}

#[derive(PartialEq, Debug)]
struct PasswordPolicy {
    letter: char,
    start: u32,
    end: u32,
}

impl PasswordPolicy {
    fn new(letter: char, start: u32, end: u32) -> Self {
        Self { letter, start, end }
    }

    fn is_valid(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| c == &self.letter).count() as u32;
        count >= self.start && count <= self.end
    }
}

#[cfg(test)]
mod password_policy_is_valid_should {
    use super::PasswordPolicy;

    #[test]
    fn return_true_when_policy_is_one_to_three_as_and_there_is_one_a() {
        let policy = PasswordPolicy::new('a', 1, 3);
        assert!(policy.is_valid("abcde"))
    }

    #[test]
    fn return_true_when_policy_is_one_to_three_as_and_there_are_two_as() {
        let policy = PasswordPolicy::new('a', 1, 3);
        assert!(policy.is_valid("abacde"))
    }

    #[test]
    fn return_true_when_policy_is_one_to_three_as_and_there_are_three_as() {
        let policy = PasswordPolicy::new('a', 1, 3);
        assert!(policy.is_valid("abacade"))
    }

    #[test]
    fn return_false_when_policy_is_one_to_three_as_and_there_are_no_as() {
        let policy = PasswordPolicy::new('a', 1, 3);
        assert!(!policy.is_valid("bcde"))
    }

    #[test]
    fn return_false_when_policy_is_one_to_three_as_and_there_are_four_as() {
        let policy = PasswordPolicy::new('a', 1, 3);
        assert!(!policy.is_valid("abacadae"))
    }

    #[test]
    fn return_false_when_policy_is_one_to_three_bs_and_there_are_no_bs() {
        let policy = PasswordPolicy::new('b', 1, 3);
        assert!(!policy.is_valid("cdefg"))
    }

    #[test]
    fn return_true_when_policy_is_two_to_nine_cs_and_there_are_nine_cs() {
        let policy = PasswordPolicy::new('c', 2, 9);
        assert!(policy.is_valid("ccccccccc"))
    }
}

#[cfg(test)]
mod parse_password_line_should {
    use super::{parse_password_line, PasswordPolicy};

    #[test]
    fn parse_first_example() {
        let result = parse_password_line("1-3 a: abcde");

        assert_eq!(result, ("abcde".into(), PasswordPolicy::new('a', 1, 3)));
    }

    #[test]
    fn parse_second_example() {
        let result = parse_password_line("1-3 b: cdefg");

        assert_eq!(result, ("cdefg".into(), PasswordPolicy::new('b', 1, 3)));
    }

    #[test]
    fn parse_third_example() {
        let result = parse_password_line("2-9 c: ccccccccc");

        assert_eq!(result, ("ccccccccc".into(), PasswordPolicy::new('c', 2, 9)));
    }
}
