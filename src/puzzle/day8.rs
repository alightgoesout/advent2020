use crate::puzzle::input::read_lines;

pub fn execute() {
    let instructions = parse_instructions(read_lines("day8").unwrap());
    println!(
        "8:1 — Value of accumulator before looping: {}",
        ProgramExecution::from(instructions).last().unwrap(),
    );
}

fn parse_instructions(lines: Vec<String>) -> Vec<Instruction> {
    lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(parse_instruction)
        .collect()
}

fn parse_instruction(line: String) -> Instruction {
    match &line[..3] {
        "acc" => Instruction::Accumulator(line[4..].parse().unwrap()),
        "jmp" => Instruction::Jump(line[4..].parse().unwrap()),
        _ => Instruction::Noop,
    }
}

enum Instruction {
    Accumulator(i32),
    Jump(i32),
    Noop,
}

struct ProgramExecution {
    instructions: Vec<Instruction>,
    accumulator: i32,
    current_instruction: i32,
    visited_instructions: Vec<i32>,
}

impl From<Vec<Instruction>> for ProgramExecution {
    fn from(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            accumulator: 0,
            current_instruction: 0,
            visited_instructions: Vec::new(),
        }
    }
}

impl Iterator for ProgramExecution {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.instructions.get(self.current_instruction as usize) {
            _ if self
                .visited_instructions
                .contains(&self.current_instruction) =>
            {
                None
            }
            Some(Instruction::Noop) => {
                self.visited_instructions.push(self.current_instruction);
                self.current_instruction += 1;
                Some(self.accumulator)
            }
            Some(Instruction::Accumulator(i)) => {
                self.visited_instructions.push(self.current_instruction);
                self.current_instruction += 1;
                self.accumulator += i;
                Some(self.accumulator)
            }
            Some(Instruction::Jump(i)) => {
                self.visited_instructions.push(self.current_instruction);
                self.current_instruction += i;
                Some(self.accumulator)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod program_execution_next_should {
    use Instruction::{Accumulator, Jump, Noop};

    use super::*;

    #[test]
    fn return_0_when_current_instruction_is_noop_and_accumulator_is_0() {
        let mut execution: ProgramExecution = vec![Noop].into();

        let result = execution.next();

        assert_eq!(result, Some(0))
    }

    #[test]
    fn return_1_when_current_instruction_is_acc_1_and_accumulator_is_0() {
        let mut execution: ProgramExecution = vec![Accumulator(1)].into();

        let result = execution.next();

        assert_eq!(result, Some(1))
    }

    #[test]
    fn return_2_when_current_instruction_is_acc_2_and_accumulator_is_0() {
        let mut execution: ProgramExecution = vec![Accumulator(2)].into();

        let result = execution.next();

        assert_eq!(result, Some(2))
    }

    #[test]
    fn return_0_when_instructions_are_acc_3_and_acc_minus_3() {
        let mut execution: ProgramExecution = vec![Accumulator(3), Accumulator(-3)].into();

        let result = execution.nth(1);

        assert_eq!(result, Some(0))
    }

    #[test]
    fn return_3_when_instructions_are_acc_3_and_noop() {
        let mut execution: ProgramExecution = vec![Accumulator(3), Noop].into();

        let result = execution.nth(1);

        assert_eq!(result, Some(3))
    }

    #[test]
    fn return_3_when_instructions_are_noop_and_acc_3() {
        let mut execution: ProgramExecution = vec![Noop, Accumulator(3)].into();

        let result = execution.nth(1);

        assert_eq!(result, Some(3))
    }

    #[test]
    fn return_7_when_instructions_are_jmp_2_acc_3_acc_7() {
        let mut execution: ProgramExecution = vec![Jump(2), Accumulator(3), Accumulator(7)].into();

        let result = execution.nth(1);

        assert_eq!(result, Some(7))
    }

    #[test]
    fn return_none_when_current_instruction_has_already_been_executed() {
        let mut execution: ProgramExecution = vec![Noop, Jump(-1)].into();

        let result = execution.nth(2);

        assert!(result.is_none())
    }
}

#[cfg(test)]
mod program_execution_last_should {
    use super::*;

    #[test]
    fn return_5_for_example_program() {
        let mut execution: ProgramExecution = parse_instructions(vec![
            "nop +0".into(),
            "acc +1".into(),
            "jmp +4".into(),
            "acc +3".into(),
            "jmp -3".into(),
            "acc -99".into(),
            "acc +1".into(),
            "jmp -4".into(),
            "acc +6".into(),
        ])
        .into();

        let result = execution.last();

        assert_eq!(result, Some(5));
    }
}
