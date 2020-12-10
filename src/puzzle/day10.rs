use crate::puzzle::input::read_lines;
use itertools::Itertools;
use std::collections::HashSet;

pub fn execute() {
    let adapters = get_adapters();
    let (diff1, diff3) = count_jolt_differences(&compute_adapter_chain(&adapters));
    println!(
        "10:1 — Product of 1-jolt differences and 3-jolts differences: {}",
        diff1 * diff3,
    );
    println!(
        "10:2 — Total number of adapter arrangements: {}",
        total_arrangements(&adapters),
    );
}

fn get_adapters() -> HashSet<u64> {
    let mut adapters = read_lines("day10")
        .unwrap()
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect::<HashSet<_>>();
    let max = adapters.iter().max().copied().unwrap();
    adapters.insert(0);
    adapters.insert(max + 3);
    adapters
}

fn count_jolt_differences(chain: &[u64]) -> (usize, usize) {
    let (diff1, diff3) = chain
        .windows(2)
        .map(|window| window[1] - window[0])
        .filter(|d| *d == 1 || *d == 3)
        .partition::<Vec<_>, _>(|d| *d == 1);
    (diff1.len(), diff3.len())
}

fn compute_adapter_chain(adapters: &HashSet<u64>) -> Vec<u64> {
    let mut chain = vec![0];
    let mut current_joltage = 0;
    while let Some(joltage) = find_adapter(current_joltage, &adapters) {
        current_joltage = joltage;
        chain.push(joltage);
    }
    chain
}

fn find_adapter(joltage: u64, adapters: &HashSet<u64>) -> Option<u64> {
    adapters
        .get(&(joltage + 1))
        .or_else(|| adapters.get(&(joltage + 2)))
        .or_else(|| adapters.get(&(joltage + 3)))
        .copied()
}

fn total_arrangements(adapters: &HashSet<u64>) -> u64 {
    adapters
        .iter()
        .sorted()
        .copied()
        .contiguous()
        .map(|group| nb_combinations(group.len()))
        .product()
}

fn nb_combinations(group_size: usize) -> u64 {
    match group_size {
        0 => 0,
        1 | 2 => 1,
        n => nb_combinations(n - 1) + nb_combinations(n - 2) + nb_combinations(n - 3),
    }
}

struct ContiguousIntegers<I>
where
    I: Iterator<Item = u64>,
{
    iterator: I,
    previous_value: Option<u64>,
}

impl<I> Iterator for ContiguousIntegers<I>
where
    I: Iterator<Item = u64>,
{
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut values = self.previous_value.into_iter().collect::<Vec<_>>();
        while let Some(i) = self.iterator.next() {
            if values.is_empty() || i - values.last().unwrap() == 1 {
                values.push(i)
            } else {
                self.previous_value = Some(i);
                return Some(values);
            }
        }
        None
    }
}

trait IntoContiguousIntegers<I>
where
    I: Iterator<Item = u64>,
{
    fn contiguous(self) -> ContiguousIntegers<I>;
}

impl<I> IntoContiguousIntegers<I> for I
where
    I: Iterator<Item = u64>,
{
    fn contiguous(self) -> ContiguousIntegers<I> {
        ContiguousIntegers {
            iterator: self,
            previous_value: None,
        }
    }
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

#[cfg(test)]
mod nb_combinations_should {
    use super::*;

    #[test]
    fn return_1_for_1() {
        assert_eq!(nb_combinations(1), 1);
    }

    #[test]
    fn return_1_for_2() {
        assert_eq!(nb_combinations(2), 1);
    }

    #[test]
    fn return_2_for_3() {
        assert_eq!(nb_combinations(3), 2);
    }

    #[test]
    fn return_4_for_4() {
        assert_eq!(nb_combinations(4), 4);
    }

    #[test]
    fn return_7_for_5() {
        assert_eq!(nb_combinations(5), 7);
    }
}
