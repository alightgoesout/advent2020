use super::input::read_lines;
use im_rc::Vector;

pub fn execute() {
    let entries = get_entries();
    println!(
        "1:1 — Product of two entries that sum to 2020: {}",
        find_product_of_pair_with_sum(2020, 0, &entries).unwrap()
    );
    println!(
        "1:2 — Product of three entries that sum to 2020: {}",
        find_product_of_triplet_with_sum(2020, 0, &entries).unwrap()
    );
}

fn get_entries() -> Vector<u32> {
    read_lines("day1")
        .unwrap()
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn find_product_of_pair_with_sum(sum: u32, start: usize, entries: &Vector<u32>) -> Option<u32> {
    entries
        .iter()
        .skip(start)
        .filter(|v| **v < sum)
        .enumerate()
        .flat_map(|(i, v)| find_value(sum - v, i + 1, &entries).map(|v2| v * v2))
        .next()
}

fn find_value(value: u32, start: usize, entries: &Vector<u32>) -> Option<u32> {
    entries.iter().skip(start).find(|v| **v == value).copied()
}

fn find_product_of_triplet_with_sum(sum: u32, start: usize, entries: &Vector<u32>) -> Option<u32> {
    entries
        .iter()
        .skip(start)
        .filter(|v| **v < sum)
        .enumerate()
        .flat_map(|(i, v)| find_product_of_pair_with_sum(sum - v, i + 1, &entries).map(|v2| v * v2))
        .next()
}
