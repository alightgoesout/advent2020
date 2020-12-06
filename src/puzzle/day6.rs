use lazy_static::lazy_static;
use regex::Regex;

use crate::puzzle::input::read_input;

pub fn execute() {
    let input = read_input("day6");
    let groups = get_groups(input.as_str());
    println!(
        "6:1 — Sum of unique yes answers in each group: {}",
        sum_of_unique_yes_answers_in_each_group(&groups),
    );
}

lazy_static! {
    static ref GROUP_SEPARATOR: Regex = Regex::new(r"\r?\n\s*\r?\n").unwrap();
}

fn get_groups(input: &str) -> Vec<&str> {
    GROUP_SEPARATOR.split(input).collect()
}

fn sum_of_unique_yes_answers_in_each_group(groups: &[&str]) -> usize {
    groups
        .iter()
        .map(|group| unique_yes_answers_in_group(group))
        .sum()
}

fn unique_yes_answers_in_group(group: &str) -> usize {
    let mut chars = group
        .chars()
        .filter(|c| ('a'..='z').contains(c))
        .collect::<Vec<_>>();
    chars.sort_unstable();
    chars.dedup();
    chars.len()
}

#[cfg(test)]
mod unique_yes_answers_in_group_should {
    use super::*;

    #[test]
    fn return_0_when_group_is_empty() {
        assert_eq!(unique_yes_answers_in_group(""), 0);
    }

    #[test]
    fn return_1_when_group_contains_one_letter() {
        assert_eq!(unique_yes_answers_in_group("a"), 1);
    }

    #[test]
    fn return_2_when_group_contains_two_different_letters() {
        assert_eq!(unique_yes_answers_in_group("ab"), 2);
    }

    #[test]
    fn return_1_when_group_contains_the_same_two_letters() {
        assert_eq!(unique_yes_answers_in_group("aa"), 1);
    }

    #[test]
    fn return_2_when_group_contains_two_different_letters_separated_by_a_new_line() {
        assert_eq!(unique_yes_answers_in_group("a\nb"), 2);
    }

    #[test]
    fn return_20_for_first_group_in_input() {
        assert_eq!(
            unique_yes_answers_in_group("edmzkxfoprcnhijtyvl\r\nadxntojykfcvzermplh"),
            20
        );
    }
}

#[cfg(test)]
mod sum_of_unique_yes_answers_in_each_group_should {
    use super::*;

    #[test]
    fn return_11_for_the_example() {
        let groups = get_groups(
            r"abc

a
b
c

ab
ac

a
a
a
a

b",
        );

        assert_eq!(sum_of_unique_yes_answers_in_each_group(&groups), 11);
    }
}
