use crate::puzzle::input::read_lines;
use std::ops::RangeInclusive;

pub fn execute() {
    let boarding_passes: Vec<BoardingPass> = read_lines("day5")
        .unwrap()
        .iter()
        .map(|line| line.as_str().into())
        .collect();
    println!(
        "5:1 — Highest seat id: {}",
        highest_seat_id(&boarding_passes),
    );
}

fn highest_seat_id(boarding_passes: &[BoardingPass]) -> u16 {
    boarding_passes
        .iter()
        .map(BoardingPass::seat_id)
        .max()
        .unwrap()
}

#[derive(Clone)]
struct BoardingPass {
    row: u8,
    column: u8,
}

impl BoardingPass {
    fn seat_id(&self) -> u16 {
        (self.row as u16 * 8) + (self.column as u16)
    }
}

impl From<&str> for BoardingPass {
    fn from(input: &str) -> Self {
        Self {
            row: partition_binary_space(&input[0..7], 0..=127),
            column: partition_binary_space(&input[7..], 0..=7),
        }
    }
}

fn partition_binary_space(input: &str, range: RangeInclusive<u8>) -> u8 {
    *input.chars().fold(range, partition).start()
}

fn partition(range: RangeInclusive<u8>, c: char) -> RangeInclusive<u8> {
    match c {
        'F' | 'L' => {
            let end = (range.start() + range.end()) / 2;
            *range.start()..=end
        }
        _ => {
            let start = (range.start() + range.end() + 1) / 2;
            start..=*range.end()
        }
    }
}

#[cfg(test)]
mod partition_should {
    use super::*;

    #[test]
    fn return_a_range_including_just_44_when_char_is_f_and_range_44_to_45() {
        assert_eq!(partition(44..=45, 'F'), 44..=44)
    }

    #[test]
    fn return_44_to_45_when_char_is_f_and_range_44_to_47() {
        assert_eq!(partition(44..=47, 'F'), 44..=45)
    }

    #[test]
    fn return_44_to_47_when_char_is_b_and_range_40_to_47() {
        assert_eq!(partition(40..=47, 'B'), 44..=47)
    }

    #[test]
    fn return_4_to_7_when_char_is_r_and_range_0_to_7() {
        assert_eq!(partition(0..=7, 'R'), 4..=7)
    }

    #[test]
    fn return_4_to_5_when_char_is_l_and_range_4_to_7() {
        assert_eq!(partition(4..=7, 'L'), 4..=5)
    }
}

#[cfg(test)]
mod partition_binary_space_should {
    use super::*;

    #[test]
    fn return_44_when_input_is_fbfbbff_and_range_0_to_127() {
        assert_eq!(partition_binary_space("FBFBBFF", 0..=127), 44)
    }

    #[test]
    fn return_5_when_input_is_rlr_and_range_0_to_8() {
        assert_eq!(partition_binary_space("RLR", 0..=7), 5)
    }
}
