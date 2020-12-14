use std::collections::HashMap;

use crate::puzzle::day14::InitializationInstruction::Write;
use crate::puzzle::input::read_lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryFrom;

pub fn execute() {
    let instructions = read_lines("day14")
        .unwrap()
        .iter()
        .map(|l| InitializationInstruction::try_from(l.as_str()))
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let mut program = DockingProgram::new();
    program.execute_all(&instructions);
    println!(
        "14:1 — Sum of memory after initialization: {}",
        program.memory_sum()
    );
}

struct DockingProgram {
    and_mask: usize,
    or_mask: usize,
    memory: HashMap<usize, usize>,
}

impl DockingProgram {
    fn new() -> Self {
        Self {
            and_mask: 1,
            or_mask: 0,
            memory: HashMap::new(),
        }
    }

    fn execute(&mut self, instruction: &InitializationInstruction) {
        match *instruction {
            InitializationInstruction::Masks { and, or } => {
                self.and_mask = and;
                self.or_mask = or;
            }
            Write { value, destination } => {
                self.memory
                    .insert(destination, value & self.and_mask | self.or_mask);
            }
        }
    }

    fn execute_all(&mut self, instructions: &[InitializationInstruction]) {
        instructions
            .iter()
            .for_each(|instruction| self.execute(instruction));
    }

    fn memory_sum(&self) -> usize {
        self.memory.values().sum()
    }
}

enum InitializationInstruction {
    Masks { and: usize, or: usize },
    Write { value: usize, destination: usize },
}

lazy_static! {
    static ref MASKS_REGEX: Regex = Regex::new(r"mask = (?P<mask>[X01]{36})").unwrap();
    static ref WRITE_REGEX: Regex =
        Regex::new(r"mem\[(?P<destination>\d+)\] = (?P<value>\d+)").unwrap();
}

impl TryFrom<&str> for InitializationInstruction {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Some(captures) = MASKS_REGEX.captures(s) {
            let mask = captures.name("mask").unwrap().as_str();
            let and = mask
                .char_indices()
                .map(|(i, c)| match c {
                    '0' => 0,
                    _ => 1 << (36 - i - 1),
                })
                .sum();
            let or = mask
                .char_indices()
                .map(|(i, c)| match c {
                    '1' => 1 << (36 - i - 1),
                    _ => 0,
                })
                .sum();
            Ok(InitializationInstruction::Masks { and, or })
        } else if let Some(captures) = WRITE_REGEX.captures(s) {
            let value = captures.name("value").unwrap().as_str().parse().unwrap();
            let destination = captures
                .name("destination")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            Ok(InitializationInstruction::Write { value, destination })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod docking_program_execute_all {
    use super::*;

    #[test]
    fn return_a_memory_with_sum_165_for_example() {
        let instructions = r"
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
"
        .lines()
        .map(InitializationInstruction::try_from)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

        let mut program = DockingProgram::new();
        program.execute_all(&instructions);

        assert_eq!(program.memory_sum(), 165);
    }
}
