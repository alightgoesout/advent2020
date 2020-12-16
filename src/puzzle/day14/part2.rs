use std::collections::HashMap;
use std::convert::TryFrom;

use crate::puzzle::input::read_lines;
use lazy_static::lazy_static;
use regex::Regex;

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
        "14:2 — Sum of memory after initialization: {}",
        program.memory_sum()
    );
}

struct DockingProgram {
    mask: usize,
    floating_masks: Vec<usize>,
    memory: HashMap<usize, usize>,
}

impl DockingProgram {
    fn new() -> Self {
        Self {
            mask: 1,
            floating_masks: Vec::new(),
            memory: HashMap::new(),
        }
    }

    fn execute(&mut self, instruction: &InitializationInstruction) {
        match instruction {
            InitializationInstruction::Masks {
                mask,
                floating_masks,
            } => {
                self.mask = *mask;
                self.floating_masks = floating_masks.clone();
            }
            InitializationInstruction::Write { value, destination } => {
                let address = destination | self.mask;
                for address in apply_floating_masks(address, &self.floating_masks) {
                    self.memory.insert(address, *value);
                }
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

fn apply_floating_masks(address: usize, masks: &[usize]) -> Vec<usize> {
    if masks.is_empty() {
        vec![address]
    } else {
        let mask = 1 << masks[0];
        let masks = &masks[1..];
        let mut addresses = apply_floating_masks(address & !mask, masks);
        addresses.extend(apply_floating_masks(address | mask, masks));
        addresses
    }
}

enum InitializationInstruction {
    Masks {
        mask: usize,
        floating_masks: Vec<usize>,
    },
    Write {
        value: usize,
        destination: usize,
    },
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
            let mask_pattern = captures.name("mask").unwrap().as_str();
            let mut mask = 0;
            let mut floating_masks = Vec::new();
            for (i, c) in mask_pattern.char_indices() {
                mask <<= 1;
                match c {
                    'X' => floating_masks.push(35 - i),
                    '1' => mask += 1,
                    _ => (),
                }
            }
            Ok(InitializationInstruction::Masks {
                mask,
                floating_masks,
            })
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
    fn return_a_memory_with_sum_208_for_example() {
        let instructions = r"
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
"
        .lines()
        .map(InitializationInstruction::try_from)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

        let mut program = DockingProgram::new();
        program.execute_all(&instructions);

        assert_eq!(program.memory_sum(), 208);
    }
}
