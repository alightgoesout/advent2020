use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

use crate::puzzle::input::read_lines;

pub fn execute() {
    let containing_bags = parse_containing_bags(read_lines("day7").unwrap());
    println!(
        "7:1 — Number of bags that an contain a shiny gold bag: {}",
        count_bags_able_to_contain_a_shiny_gold_bag(&containing_bags),
    );
}

lazy_static! {
    static ref CONTAINING_BAG_REGEX: Regex = Regex::new(r"(?P<bag>\w+ \w+) bags contain").unwrap();
    static ref CONTAINED_BAG_REGEX: Regex =
        Regex::new(r"(?P<number>\d) (?P<bag>\w+ \w+) bag").unwrap();
}

fn count_bags_able_to_contain_a_shiny_gold_bag(
    containing_bags: &HashMap<String, Vec<String>>,
) -> usize {
    let mut bags_to_traverse = vec!["shiny gold"];
    let mut traversed_bags = HashSet::new();
    while !bags_to_traverse.is_empty() {
        let bag = bags_to_traverse.remove(0);
        containing_bags
            .get(bag)
            .iter()
            .flat_map(|v| v.iter())
            .for_each(|containing_bag| {
                if !traversed_bags.contains(containing_bag) {
                    bags_to_traverse.push(containing_bag);
                    traversed_bags.insert(containing_bag);
                }
            });
    }
    traversed_bags.len()
}

fn parse_containing_bags(lines: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut containing_bags = HashMap::new();
    for line in lines {
        if let Some(captures) = CONTAINING_BAG_REGEX.captures(line.as_str()) {
            let bag: String = captures.name("bag").unwrap().as_str().into();
            let contained_bags = get_contained_bags(&line);
            insert_containing_bags(&mut containing_bags, bag, contained_bags);
        }
    }
    containing_bags
}

fn get_contained_bags(line: &str) -> Vec<String> {
    CONTAINED_BAG_REGEX
        .captures_iter(line)
        .map(|c| c.name("bag"))
        .flatten()
        .map(|m| m.as_str().into())
        .collect()
}

fn insert_containing_bags(
    containing_bags: &mut HashMap<String, Vec<String>>,
    bag: String,
    contained_bags: Vec<String>,
) {
    contained_bags.into_iter().for_each(|contained_bag| {
        containing_bags
            .entry(contained_bag)
            .or_insert_with(Vec::new)
            .push(bag.clone())
    });
}

#[cfg(test)]
mod count_bags_able_to_contain_a_shiny_gold_bag_should {
    use super::*;

    #[test]
    fn return_4_for_the_example_data() {
        let containing_bags = parse_containing_bags(vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".into(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".into(),
            "bright white bags contain 1 shiny gold bag.".into(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".into(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".into(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".into(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".into(),
            "faded blue bags contain no other bags.".into(),
            "dotted black bags contain no other bags.".into(),
        ]);

        assert_eq!(
            count_bags_able_to_contain_a_shiny_gold_bag(&containing_bags),
            4
        );
    }
}
