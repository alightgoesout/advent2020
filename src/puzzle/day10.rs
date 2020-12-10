use crate::puzzle::input::read_lines;
use std::collections::HashSet;

pub fn execute() {
    let adapters = get_adapters();
    let (diff1, diff3) = count_jolt_differences(&compute_adapter_chain(&adapters));
    println!(
        "10:1 — Product of 1-jolt differences and 3-jolts differences: {}",
        diff1 * diff3,
    );
}

fn get_adapters() -> HashSet<u32> {
    read_lines("day10")
        .unwrap()
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn count_jolt_differences(chain: &[u32]) -> (usize, usize) {
    let (diff1, diff3) = chain
        .windows(2)
        .map(|window| window[1] - window[0])
        .filter(|d| *d == 1 || *d == 3)
        .partition::<Vec<_>, _>(|d| *d == 1);
    (diff1.len(), diff3.len())
}

fn compute_adapter_chain(adapters: &HashSet<u32>) -> Vec<u32> {
    let adapters = add_device_joltage(adapters);
    let mut chain = vec![0];
    let mut current_joltage = 0;
    while let Some(joltage) = find_adapter(current_joltage, &adapters) {
        current_joltage = joltage;
        chain.push(joltage);
    }
    chain
}

fn add_device_joltage(adapters: &HashSet<u32>) -> HashSet<u32> {
    let max = adapters.iter().max().copied().unwrap();
    let adapters = adapters | &[max + 3].iter().copied().collect::<HashSet<_>>();
    adapters
}

fn find_adapter(joltage: u32, adapters: &HashSet<u32>) -> Option<u32> {
    adapters
        .get(&(joltage + 1))
        .or_else(|| adapters.get(&(joltage + 2)))
        .or_else(|| adapters.get(&(joltage + 3)))
        .copied()
}

#[cfg(test)]
mod compute_adapter_chain_should {
    use super::*;

    #[test]
    fn return_a_chain_of_13_for_the_first_sample() {
        let adapters = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
            .iter()
            .copied()
            .collect::<HashSet<_>>();

        let result = compute_adapter_chain(&adapters);

        assert_eq!(result, vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22]);
    }
}

#[cfg(test)]
mod count_jolt_differences_should {
    use super::*;

    #[test]
    fn return_7_and_5_for_the_first_sample() {
        let adapters = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
            .iter()
            .copied()
            .collect::<HashSet<_>>();

        let result = count_jolt_differences(&compute_adapter_chain(&adapters));

        assert_eq!(result, (7, 5));
    }
}
