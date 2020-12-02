use im_rc::Vector;
use lazy_static::lazy_static;
use regex::Regex;

use crate::puzzle::input::read_lines;

pub fn execute() {
    let entries = read_lines("day2").unwrap();
    println!(
        "2:1 — Number of valid passwords for sled policy: {}",
        count_valid_passwords_for_sled_policy(&entries)
    );
    println!(
        "2:2 — Number of valid passwords for toboggan policy: {}",
        count_valid_passwords_for_toboggan_policy(&entries)
    );
}

fn count_valid_passwords_for_sled_policy(entries: &Vector<String>) -> usize {
    entries
        .iter()
        .map(|entry| parse_password_line_to_sled_policy(entry.as_str()))
        .filter(|(password, policy)| policy.is_valid(password.as_str()))
        .count()
}

fn parse_password_line_to_sled_policy(password_line: &str) -> (String, SledPasswordPolicy) {
    let (password, letter, start, end) = split_password_line(password_line);
    (
        password,
        SledPasswordPolicy::new(letter, start as u32, end as u32),
    )
}

fn count_valid_passwords_for_toboggan_policy(entries: &Vector<String>) -> usize {
    entries
        .iter()
        .map(|entry| parse_password_line_to_toboggan_policy(entry.as_str()))
        .filter(|(password, policy)| policy.is_valid(password.as_str()))
        .count()
}

fn parse_password_line_to_toboggan_policy(password_line: &str) -> (String, TobogganPasswordPolicy) {
    let (password, letter, first_position, second_position) = split_password_line(password_line);
    (
        password,
        TobogganPasswordPolicy::new(letter, first_position, second_position),
    )
}

fn split_password_line(password_line: &str) -> (String, char, usize, usize) {
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
        captures.name("a").unwrap().as_str().parse().unwrap(),
        captures.name("b").unwrap().as_str().parse().unwrap(),
    )
}

lazy_static! {
    static ref PASSWORD_LINE_REGEX: Regex =
        Regex::new(r"(?P<a>\d+)-(?P<b>\d+) (?P<letter>[[:alpha:]]): (?P<password>.*)").unwrap();
}

#[derive(PartialEq, Debug)]
struct SledPasswordPolicy {
    letter: char,
    start: u32,
    end: u32,
}

impl SledPasswordPolicy {
    fn new(letter: char, start: u32, end: u32) -> Self {
        Self { letter, start, end }
    }

    fn is_valid(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| c == &self.letter).count() as u32;
        count >= self.start && count <= self.end
    }
}

struct TobogganPasswordPolicy {
    letter: char,
    first_position: usize,
    second_position: usize,
}

impl TobogganPasswordPolicy {
    fn new(letter: char, first_position: usize, second_position: usize) -> Self {
        Self {
            letter,
            first_position,
            second_position,
        }
    }

    fn is_valid(&self, password: &str) -> bool {
        let chars = chars_at(password, self.first_position - 1, self.second_position - 1);
        match chars {
            (a, b) if a == b => false,
            (a, b) if a == self.letter || b == self.letter => true,
            _ => false,
        }
    }
}

fn chars_at(str: &str, first_position: usize, second_position: usize) -> (char, char) {
    (
        str.chars().nth(first_position).unwrap(),
        str.chars().nth(second_position).unwrap(),
    )
}

#[cfg(test)]
mod sled_password_policy_is_valid_should {
    use super::SledPasswordPolicy;

    #[test]
    fn return_true_when_policy_is_one_to_three_as_and_there_is_one_a() {
        let policy = SledPasswordPolicy::new('a', 1, 3);
        assert!(policy.is_valid("abcde"))
    }

    #[test]
    fn return_true_when_policy_is_one_to_three_as_and_there_are_two_as() {
        let policy = SledPasswordPolicy::new('a', 1, 3);
        assert!(policy.is_valid("abacde"))
    }

    #[test]
    fn return_true_when_policy_is_one_to_three_as_and_there_are_three_as() {
        let policy = SledPasswordPolicy::new('a', 1, 3);
        assert!(policy.is_valid("abacade"))
    }

    #[test]
    fn return_false_when_policy_is_one_to_three_as_and_there_are_no_as() {
        let policy = SledPasswordPolicy::new('a', 1, 3);
        assert!(!policy.is_valid("bcde"))
    }

    #[test]
    fn return_false_when_policy_is_one_to_three_as_and_there_are_four_as() {
        let policy = SledPasswordPolicy::new('a', 1, 3);
        assert!(!policy.is_valid("abacadae"))
    }

    #[test]
    fn return_false_when_policy_is_one_to_three_bs_and_there_are_no_bs() {
        let policy = SledPasswordPolicy::new('b', 1, 3);
        assert!(!policy.is_valid("cdefg"))
    }

    #[test]
    fn return_true_when_policy_is_two_to_nine_cs_and_there_are_nine_cs() {
        let policy = SledPasswordPolicy::new('c', 2, 9);
        assert!(policy.is_valid("ccccccccc"))
    }
}

#[cfg(test)]
mod parse_password_line_to_sled_policy {
    use super::{parse_password_line_to_sled_policy, SledPasswordPolicy};

    #[test]
    fn parse_first_example() {
        let result = parse_password_line_to_sled_policy("1-3 a: abcde");

        assert_eq!(result, ("abcde".into(), SledPasswordPolicy::new('a', 1, 3)));
    }

    #[test]
    fn parse_second_example() {
        let result = parse_password_line_to_sled_policy("1-3 b: cdefg");

        assert_eq!(result, ("cdefg".into(), SledPasswordPolicy::new('b', 1, 3)));
    }

    #[test]
    fn parse_third_example() {
        let result = parse_password_line_to_sled_policy("2-9 c: ccccccccc");

        assert_eq!(
            result,
            ("ccccccccc".into(), SledPasswordPolicy::new('c', 2, 9))
        );
    }
}

#[cfg(test)]
mod toboggan_password_policy_is_valid_should {
    use super::TobogganPasswordPolicy;

    #[test]
    fn return_true_when_letter_is_exactly_at_one_position() {
        let policy = TobogganPasswordPolicy::new('a', 1, 3);
        assert!(policy.is_valid("abcde"))
    }

    #[test]
    fn return_false_when_letter_is_not_at_any_position() {
        let policy = TobogganPasswordPolicy::new('b', 1, 3);
        assert!(!policy.is_valid("cdefg"))
    }

    #[test]
    fn return_false_when_letter_is_at_both_positions() {
        let policy = TobogganPasswordPolicy::new('c', 2, 9);
        assert!(!policy.is_valid("ccccccccc"))
    }
}
