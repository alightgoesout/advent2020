use crate::puzzle::input::read_lines;
use itertools::Itertools;

pub fn execute() {
    let numbers = get_numbers();
    println!(
        "9:1 — First invalid number: {}",
        validate(&numbers, 25).unwrap()
    );
}

fn get_numbers() -> Vec<u64> {
    read_lines("day9")
        .unwrap()
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn validate(numbers: &[u64], preamble_size: usize) -> Option<u64> {
    numbers
        .windows(preamble_size + 1)
        .find(|window| !is_valid(&window))
        .map(|window| window[preamble_size])
}

fn is_valid(window: &[u64]) -> bool {
    let last = window[window.len() - 1];
    window[..window.len() - 1]
        .iter()
        .combinations(2)
        .any(|c| c[0] + c[1] == last)
}

#[cfg(test)]
mod validate_should {
    use super::*;

    #[test]
    fn return_1_when_preamble_is_one_to_five_and_next_is_1() {
        let numbers = [1, 2, 3, 4, 5, 1];

        assert_eq!(validate(&numbers, 5), Some(1));
    }

    #[test]
    fn return_none_when_preamble_is_one_to_five_and_there_are_no_more_numbers() {
        let numbers = [1, 2, 3, 4, 5];

        assert_eq!(validate(&numbers, 5), None);
    }

    #[test]
    fn return_2_when_preamble_is_one_to_five_and_next_is_2() {
        let numbers = [1, 2, 3, 4, 5, 2];

        assert_eq!(validate(&numbers, 5), Some(2));
    }

    #[test]
    fn return_none_when_preamble_is_one_to_five_and_next_are_all_valid() {
        let numbers = [1, 2, 3, 4, 5, 5, 5, 8, 9, 10];

        assert_eq!(validate(&numbers, 5), None);
    }

    #[test]
    fn return_127_for_the_example_numbers() {
        let numbers = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(validate(&numbers, 5), Some(127))
    }
}
