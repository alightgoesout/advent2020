use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::puzzle::input::read_lines;

pub fn execute() {
    let lines = read_lines("day7").unwrap();
    let bags = parse_bags(&lines);
    println!(
        "7:1 — Number of bags that an contain a shiny gold bag: {}",
        count_bags_able_to_contain_a_shiny_gold_bag(&bags),
    );
    println!(
        "7:1 — Number of bags contained by shiny gold bag: {}",
        bags.count_bags_contained_by("shiny gold"),
    );
}

struct Relation<'a> {
    containing_bag: &'a str,
    number: u32,
    contained_bag: &'a str,
}

struct Bags<'a> {
    relations: Vec<Relation<'a>>,
}

impl<'a> Bags<'a> {
    fn bags_containing(&self, bag: &str) -> HashSet<&str> {
        self.relations
            .iter()
            .filter(|r| r.contained_bag == bag)
            .map(|r| r.containing_bag)
            .collect()
    }

    fn count_bags_contained_by(&self, bag: &str) -> u32 {
        self.relations
            .iter()
            .filter(|r| r.containing_bag == bag)
            .map(|r| r.number + r.number * self.count_bags_contained_by(r.contained_bag))
            .sum()
    }
}

impl<'a, T> From<T> for Bags<'a>
where
    T: IntoIterator<Item = Relation<'a>>,
{
    fn from(relations: T) -> Self {
        Self {
            relations: relations.into_iter().collect(),
        }
    }
}

lazy_static! {
    static ref CONTAINING_BAG_REGEX: Regex =
        Regex::new(r"^(?P<bag_color>\w+ \w+) bags contain (?P<rest>.*)$").unwrap();
    static ref CONTAINED_BAG_REGEX: Regex =
        Regex::new(r"(?P<number>\d) (?P<bag_color>\w+ \w+) bag").unwrap();
}

fn count_bags_able_to_contain_a_shiny_gold_bag(bags: &Bags) -> usize {
    let mut bags_to_traverse = vec!["shiny gold"];
    let mut traversed_bags = HashSet::new();
    while !bags_to_traverse.is_empty() {
        let bag = bags_to_traverse.remove(0);
        bags.bags_containing(bag).iter().for_each(|containing_bag| {
            if !traversed_bags.contains(containing_bag) {
                bags_to_traverse.push(*containing_bag);
                traversed_bags.insert(*containing_bag);
            }
        });
    }
    traversed_bags.len()
}

fn parse_bags(lines: &[String]) -> Bags {
    lines
        .iter()
        .map(|line| CONTAINING_BAG_REGEX.captures(&line))
        .flatten()
        .flat_map(get_relations)
        .collect::<Vec<_>>()
        .into()
}

fn get_relations(captures: Captures) -> Vec<Relation> {
    let containing_bag = captures.name("bag_color").unwrap().as_str();
    let rest = captures.name("rest").unwrap().as_str();
    CONTAINED_BAG_REGEX
        .captures_iter(rest)
        .map(|c| Relation {
            containing_bag,
            number: c.name("number").unwrap().as_str().parse().unwrap(),
            contained_bag: c.name("bag_color").unwrap().as_str(),
        })
        .collect()
}

#[cfg(test)]
fn first_example_lines() -> Vec<String> {
    vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".into(),
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".into(),
        "bright white bags contain 1 shiny gold bag.".into(),
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".into(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".into(),
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".into(),
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".into(),
        "faded blue bags contain no other bags.".into(),
        "dotted black bags contain no other bags.".into(),
    ]
}

#[cfg(test)]
mod count_bags_able_to_contain_a_shiny_gold_bag_should {
    use super::*;

    #[test]
    fn return_4_for_the_example_data() {
        let lines = first_example_lines();
        let bags = parse_bags(&lines);

        assert_eq!(count_bags_able_to_contain_a_shiny_gold_bag(&bags), 4);
    }
}

#[cfg(test)]
mod bags_count_bags_contained_by_should {
    use super::*;

    #[test]
    fn return_32_for_the_first_example() {
        let lines = first_example_lines();
        let bags = parse_bags(&lines);

        assert_eq!(bags.count_bags_contained_by("shiny gold"), 32);
    }

    #[test]
    fn return_126_for_the_second_example() {
        let lines = vec![
            "shiny gold bags contain 2 dark red bags.".into(),
            "dark red bags contain 2 dark orange bags.".into(),
            "dark orange bags contain 2 dark yellow bags.".into(),
            "dark yellow bags contain 2 dark green bags.".into(),
            "dark green bags contain 2 dark blue bags.".into(),
            "dark blue bags contain 2 dark violet bags.".into(),
            "dark violet bags contain no other bags.".into(),
        ];
        let bags = parse_bags(&lines);

        assert_eq!(bags.count_bags_contained_by("shiny gold"), 126);
    }
}
