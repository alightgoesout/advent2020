use std::collections::HashMap;

pub fn execute() {
    let input = vec![2, 0, 1, 9, 5, 19];
    let mut game = MemoryGame::from(&input);
    println!("15:1 — 2020th round: {}", game.play_until_round(2020));
}

struct MemoryGame {
    previous_numbers: HashMap<usize, usize>,
    last_number: usize,
    round: usize,
}

impl MemoryGame {
    fn play_until_round(&mut self, round: usize) -> usize {
        while self.round < round {
            self.next();
        }
        self.last_number
    }
}

impl From<&Vec<usize>> for MemoryGame {
    fn from(numbers: &Vec<usize>) -> Self {
        let previous_numbers = numbers
            .iter()
            .enumerate()
            .take(numbers.len() - 1)
            .map(|(i, n)| (*n, i + 1))
            .collect();
        let last_number = numbers[numbers.len() - 1];
        Self {
            previous_numbers,
            last_number,
            round: numbers.len(),
        }
    }
}

impl Iterator for MemoryGame {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self
            .previous_numbers
            .get(&self.last_number)
            .map(|round| self.round - round)
            .unwrap_or(0);
        self.previous_numbers.insert(self.last_number, self.round);
        self.round += 1;
        self.last_number = result;
        Some(result)
    }
}

#[cfg(test)]
mod memory_game_play_until_round_should {
    use super::*;

    #[test]
    fn return_0_for_round_4_when_input_is_0_3_6() {
        let mut game = MemoryGame::from(&vec![0, 3, 6]);
        assert_eq!(game.play_until_round(4), 0);
    }

    #[test]
    fn return_3_for_round_5_when_input_is_0_3_6() {
        let mut game = MemoryGame::from(&vec![0, 3, 6]);
        assert_eq!(game.play_until_round(5), 3);
    }

    #[test]
    fn should_return_3_for_the_6th_round_when_input_is_0_3_6() {
        let mut game = MemoryGame::from(&vec![0, 3, 6]);
        assert_eq!(game.play_until_round(6), 3);
    }

    #[test]
    fn should_return_436_for_the_2020th_round_when_input_is_0_3_6() {
        let mut game = MemoryGame::from(&vec![0, 3, 6]);
        assert_eq!(game.play_until_round(2020), 436);
    }

    #[test]
    fn should_return_1_for_the_2020th_round_when_input_is_1_3_2() {
        let mut game = MemoryGame::from(&vec![1, 3, 2]);
        assert_eq!(game.play_until_round(2020), 1);
    }

    #[test]
    fn should_return_10_for_the_2020th_round_when_input_is_2_1_3() {
        let mut game = MemoryGame::from(&vec![2, 1, 3]);
        assert_eq!(game.play_until_round(2020), 10);
    }

    #[test]
    fn should_return_27_for_the_2020th_round_when_input_is_1_2_3() {
        let mut game = MemoryGame::from(&vec![1, 2, 3]);
        assert_eq!(game.play_until_round(2020), 27);
    }

    #[test]
    fn should_return_78_for_the_2020th_round_when_input_is_2_3_1() {
        let mut game = MemoryGame::from(&vec![2, 3, 1]);
        assert_eq!(game.play_until_round(2020), 78);
    }

    #[test]
    fn should_return_438_for_the_2020th_round_when_input_is_3_2_1() {
        let mut game = MemoryGame::from(&vec![3, 2, 1]);
        assert_eq!(game.play_until_round(2020), 438);
    }

    #[test]
    fn should_return_1836_for_the_2020th_round_when_input_is_3_1_2() {
        let mut game = MemoryGame::from(&vec![3, 1, 2]);
        assert_eq!(game.play_until_round(2020), 1836);
    }
}
