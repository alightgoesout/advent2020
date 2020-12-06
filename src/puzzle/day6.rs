use std::collections::HashSet;

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
    println!(
        "6:2 — Sum of yes answers by all members in each group: {}",
        sum_of_yes_answers_by_all_members_in_each_group(&groups),
    );
}

lazy_static! {
    static ref GROUP_SEPARATOR: Regex = Regex::new(r"\r?\n\s*\r?\n").unwrap();
    static ref MEMBER_SEPARATOR: Regex = Regex::new(r"\r?\n").unwrap();
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

fn sum_of_yes_answers_by_all_members_in_each_group(groups: &[&str]) -> usize {
    groups
        .iter()
        .map(|group| nb_yes_answers_by_all_group_members(group))
        .sum()
}

fn nb_yes_answers_by_all_group_members(group: &str) -> usize {
    MEMBER_SEPARATOR
        .split(group)
        .filter(|member| !member.is_empty())
        .map(|member| {
            member
                .chars()
                .filter(|c| ('a'..='z').contains(c))
                .collect::<HashSet<_>>()
        })
        .fold(None as Option<HashSet<char>>, |acc, member| {
            match (acc, member) {
                (Some(result), yes_answers) => {
                    Some(result.intersection(&yes_answers).cloned().collect())
                }
                (None, yes_answers) => Some(yes_answers),
            }
        })
        .map(|yes_answers| yes_answers.len())
        .unwrap()
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

#[cfg(test)]
mod nb_yes_answers_by_all_group_members_should {
    use super::*;

    #[test]
    fn return_3_when_group_has_one_member_with_three_yes_answers() {
        assert_eq!(nb_yes_answers_by_all_group_members("abc"), 3);
    }

    #[test]
    fn return_0_when_group_has_two_members_with_different_yes_answers() {
        assert_eq!(nb_yes_answers_by_all_group_members("abc\ndef"), 0);
    }

    #[test]
    fn return_1_when_group_has_two_members_with_one_common_yes_answer() {
        assert_eq!(nb_yes_answers_by_all_group_members("ab\nac"), 1);
    }

    #[test]
    fn return_1_when_group_has_four_members_with_a_as_yes_answer() {
        assert_eq!(nb_yes_answers_by_all_group_members("a\na\na\na"), 1);
    }

    #[test]
    fn return_18_for_first_group_in_input() {
        assert_eq!(
            nb_yes_answers_by_all_group_members("edmzkxfoprcnhijtyvl\r\nadxntojykfcvzermplh"),
            18,
        );
    }

    #[test]
    fn return_1_for_last_group_in_input() {
        assert_eq!(
            nb_yes_answers_by_all_group_members("tal\r\ntal\r\na\r\nal\r\ndaevb\r\n"),
            1,
        );
    }
}

#[cfg(test)]
mod sum_of_yes_answers_by_all_members_in_each_group_should {
    use super::*;

    #[test]
    fn return_6_for_the_example() {
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

        assert_eq!(sum_of_yes_answers_by_all_members_in_each_group(&groups), 6);
    }
}
