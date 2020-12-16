use crate::puzzle::input::read_lines;
use itertools::__std_iter::Map;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

static TICKET: [u32; 20] = [
    103, 79, 61, 97, 109, 67, 89, 83, 59, 53, 139, 131, 101, 113, 149, 127, 71, 73, 107, 137,
];

pub fn execute() {
    let rules = read_lines("day16_rules")
        .unwrap()
        .iter()
        .map(|rule| ColumnRule::from(rule.as_str()))
        .collect::<Vec<_>>();
    let nearby_tickets = read_lines("day16_nearby_tickets")
        .unwrap()
        .iter()
        .map(|ticket| get_ticket(ticket))
        .collect::<Vec<_>>();

    let sum_of_invalid_columns: u32 = nearby_tickets
        .iter()
        .flat_map(|ticket| get_invalid_values(ticket, &rules))
        .sum();
    println!(
        "16:1 — Sum of all invalid columns: {}",
        sum_of_invalid_columns
    );

    let valid_tickets = nearby_tickets
        .into_iter()
        .filter(|ticket| get_invalid_values(ticket, &rules).is_empty())
        .collect::<Vec<_>>();
    let matches = match_columns_to_names(&valid_tickets, &rules);
    let column_names = allocate_columns(matches);
    let departure_indexes: Vec<_> = column_names
        .into_iter()
        .filter(|(i, name)| name.starts_with("departure"))
        .map(|(i, _)| i)
        .collect();
    let product_of_departure_columns = departure_indexes
        .into_iter()
        .map(|i| TICKET[i] as u64)
        .product::<u64>();

    println!(
        "16:1 — Product  of departure columns: {}",
        product_of_departure_columns,
    );
}

fn get_invalid_values(values: &[u32], rules: &[ColumnRule]) -> Vec<u32> {
    values
        .iter()
        .filter(|value| rules.iter().all(|rule| !rule.is_valid(*value)))
        .copied()
        .collect()
}

fn get_ticket(ticket: &str) -> Vec<u32> {
    ticket
        .split(',')
        .map(|value| value.parse().unwrap())
        .collect()
}

fn find_rules(values: &[u32], rules: &[ColumnRule]) -> Vec<String> {
    rules
        .iter()
        .filter(|rule| values.iter().all(|value| rule.is_valid(value)))
        .map(|rule| rule.name.clone())
        .collect()
}

fn match_columns_to_names(
    tickets: &[Vec<u32>],
    rules: &[ColumnRule],
) -> HashMap<usize, Vec<String>> {
    (0..tickets[0].len())
        .map(|i| {
            (
                i,
                tickets.iter().map(|ticket| ticket[i]).collect::<Vec<_>>(),
            )
        })
        .map(|(i, values)| (i, find_rules(&values, &rules)))
        .collect::<HashMap<_, _>>()
}

fn allocate_columns(mut matches: HashMap<usize, Vec<String>>) -> HashMap<usize, String> {
    let mut result = HashMap::new();
    let mut allocated_columns = HashSet::new();
    while !matches.is_empty() {
        let updated_matches = matches
            .into_iter()
            .map(|(i, names)| {
                (
                    i,
                    names
                        .into_iter()
                        .filter(|name| !allocated_columns.contains(name))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        updated_matches
            .iter()
            .filter(|(_, names)| names.len() == 1)
            .for_each(|(i, names)| {
                allocated_columns.insert(names[0].clone());
                result.insert(*i, names[0].clone());
            });
        matches = updated_matches
            .into_iter()
            .filter(|(_, names)| names.len() > 1)
            .collect();
    }
    result
}

#[derive(Debug)]
struct ColumnRule {
    name: String,
    first_range: RangeInclusive<u32>,
    second_range: RangeInclusive<u32>,
}

impl ColumnRule {
    fn is_valid(&self, value: &u32) -> bool {
        self.first_range.contains(value) || self.second_range.contains(value)
    }
}

lazy_static! {
    static ref COLUMN_RULE_REGEX: Regex =
        Regex::new(r"^(?P<column>[a-z ]+): (?P<s1>\d+)-(?P<e1>\d+) or (?P<s2>\d+)-(?P<e2>\d+)$")
            .unwrap();
}

impl From<&str> for ColumnRule {
    fn from(rule: &str) -> Self {
        let captures = COLUMN_RULE_REGEX.captures(rule).unwrap();
        Self {
            name: captures.name("column").unwrap().as_str().to_string(),
            first_range: get_integer(&captures, "s1")..=get_integer(&captures, "e1"),
            second_range: get_integer(&captures, "s2")..=get_integer(&captures, "e2"),
        }
    }
}

fn get_integer(captures: &Captures, name: &str) -> u32 {
    captures.name(name).unwrap().as_str().parse().unwrap()
}

#[cfg(test)]
mod column_rule_is_valid_should {
    use super::*;

    #[test]
    fn return_true_when_value_is_3_and_rule_is_1_to_3_and_5_to_7() {
        let rule = ColumnRule::from("class: 1-3 or 5-7");
        assert!(rule.is_valid(&3))
    }

    #[test]
    fn return_false_when_value_is_4_and_rule_is_1_to_3_and_5_to_7() {
        let rule = ColumnRule::from("class: 1-3 or 5-7");
        assert!(!rule.is_valid(&4))
    }

    #[test]
    fn return_true_when_value_is_5_and_rule_is_1_to_3_and_5_to_7() {
        let rule = ColumnRule::from("class: 1-3 or 5-7");
        assert!(rule.is_valid(&5))
    }
}

#[cfg(test)]
mod get_invalid_values_should {
    use super::*;

    #[test]
    fn return_an_empty_vector_when_all_fields_are_valid_for_at_least_a_rule() {
        let rules = [
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
        ]
        .iter()
        .copied()
        .map(ColumnRule::from)
        .collect::<Vec<_>>();

        assert!(get_invalid_values(&[7, 3, 47], &rules).is_empty());
    }

    #[test]
    fn return_a_vector_of_one_value_when_one_field_is_invalid_for_all_rules() {
        let rules = [
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
        ]
        .iter()
        .copied()
        .map(ColumnRule::from)
        .collect::<Vec<_>>();

        assert_eq!(get_invalid_values(&[40, 4, 50], &rules), vec![4]);
    }

    #[test]
    fn return_a_vector_of_two_values_when_two_fields_are_invalid_for_all_rules() {
        let rules = [
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
        ]
        .iter()
        .copied()
        .map(ColumnRule::from)
        .collect::<Vec<_>>();

        assert_eq!(get_invalid_values(&[40, 4, 51], &rules), vec![4, 51]);
    }
}

#[cfg(test)]
mod find_rule_should {
    use super::*;

    #[test]
    fn return_row_for_first_column_of_example() {
        let values = [3, 15, 5];
        let rules = [
            "class: 0-1 or 4-19",
            "row: 0-5 or 8-19",
            "seat: 0-13 or 16-19",
        ]
        .iter()
        .copied()
        .map(ColumnRule::from)
        .collect::<Vec<_>>();

        assert_eq!(find_rules(&values, &rules), vec![String::from("row")]);
    }

    #[test]
    fn return_class_row_for_second_column_of_example() {
        let values = [9, 1, 14];
        let rules = [
            "class: 0-1 or 4-19",
            "row: 0-5 or 8-19",
            "seat: 0-13 or 16-19",
        ]
        .iter()
        .copied()
        .map(ColumnRule::from)
        .collect::<Vec<_>>();

        assert_eq!(
            find_rules(&values, &rules),
            vec![String::from("class"), String::from("row")]
        );
    }

    #[test]
    fn return_class_row_seat_for_third_column_of_example() {
        let values = [18, 5, 9];
        let rules = [
            "class: 0-1 or 4-19",
            "row: 0-5 or 8-19",
            "seat: 0-13 or 16-19",
        ]
        .iter()
        .copied()
        .map(ColumnRule::from)
        .collect::<Vec<_>>();

        assert_eq!(
            find_rules(&values, &rules),
            vec![
                String::from("class"),
                String::from("row"),
                String::from("seat")
            ]
        );
    }
}

#[cfg(test)]
mod match_columns_to_names_test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn example() {
        let tickets = vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]];
        let rules = [
            "class: 0-1 or 4-19",
            "row: 0-5 or 8-19",
            "seat: 0-13 or 16-19",
        ]
        .iter()
        .copied()
        .map(ColumnRule::from)
        .collect::<Vec<_>>();

        let mut expected = HashMap::new();
        expected.insert(0, vec![String::from("row")]);
        expected.insert(1, vec![String::from("class"), String::from("row")]);
        expected.insert(
            2,
            vec![
                String::from("class"),
                String::from("row"),
                String::from("seat"),
            ],
        );
        assert_eq!(match_columns_to_names(&tickets, &rules), expected);
    }
}

#[cfg(test)]
mod allocate_columns_test {
    use super::*;

    #[test]
    fn example() {
        let tickets = vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]];
        let rules = [
            "class: 0-1 or 4-19",
            "row: 0-5 or 8-19",
            "seat: 0-13 or 16-19",
        ]
        .iter()
        .copied()
        .map(ColumnRule::from)
        .collect::<Vec<_>>();

        let mut expected = HashMap::new();
        expected.insert(0, String::from("row"));
        expected.insert(1, String::from("class"));
        expected.insert(2, String::from("seat"));
        assert_eq!(
            allocate_columns(match_columns_to_names(&tickets, &rules)),
            expected
        );
    }
}
