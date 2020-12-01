use super::input::read_lines;
use im_rc::Vector;

pub fn execute() {
    let entries = get_entries();
    println!(
        "1:1 — Product of two entries that sum to 2020: {}",
        get_product_of_entries_that_sum_to_2020(&entries).unwrap()
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

fn get_product_of_entries_that_sum_to_2020(entries: &Vector<u32>) -> Option<u32> {
    entries
        .iter()
        .enumerate()
        .flat_map(|(i, v)| find_pair(i, *v, &entries).map(|v2| v * v2))
        .next()
}

fn find_pair(i: usize, v: u32, entries: &Vector<u32>) -> Option<u32> {
    entries.iter().skip(i).find(|v2| v + **v2 == 2020).copied()
}
