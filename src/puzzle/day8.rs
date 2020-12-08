use crate::puzzle::day8::ExecutionResult::{Finished, InfiniteLoop};
use crate::puzzle::input::read_lines;
use Instruction::{Accumulator, Jump, Noop};

pub fn execute() {
    let instructions = parse_instructions(read_lines("day8").unwrap());
    println!(
        "8:1 — Value of accumulator before looping: {:?}",
        Program::new(&instructions).execute(),
    );
    println!(
        "8:2 — Value of accumulator after the fixed program terminates: {:?}",
        fix_program(&instructions),
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
    let parameter = line[4..].parse().unwrap();
    match &line[..3] {
        "acc" => Accumulator(parameter),
        "jmp" => Jump(parameter),
        _ => Noop(parameter),
    }
}

fn fix_program(instructions: &[Instruction]) -> Option<ExecutionResult> {
    (0..instructions.len())
        .map(|i| permute(&instructions, i))
        .flatten()
        .map(|instructions| Program::new(&instructions))
        .map(|program| program.execute())
        .find(|result| matches!(result, Finished(_)))
}

fn permute(instructions: &[Instruction], index: usize) -> Option<Vec<Instruction>> {
    match &instructions[index] {
        Noop(i) => {
            let mut instructions = instructions.to_owned();
            instructions[index] = Jump(*i);
            Some(instructions)
        }
        Jump(i) => {
            let mut instructions = instructions.to_owned();
            instructions[index] = Noop(*i);
            Some(instructions)
        }
        _ => None,
    }
}

#[derive(Clone)]
enum Instruction {
    Accumulator(i32),
    Jump(i32),
    Noop(i32),
}

#[derive(PartialEq, Debug)]
enum ExecutionResult {
    InfiniteLoop(i32),
    Finished(i32),
}

struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn new(instructions: &[Instruction]) -> Self {
        Self {
            instructions: instructions.to_owned(),
        }
    }

    fn execute(&self) -> ExecutionResult {
        let mut accumulator: i32 = 0;
        let mut current_instruction: i32 = 0;
        let mut visited_instructions: Vec<i32> = Vec::new();
        while !visited_instructions.contains(&current_instruction)
            && (current_instruction as usize) < self.instructions.len()
        {
            match self.instructions[current_instruction as usize] {
                Noop(_) => {
                    visited_instructions.push(current_instruction);
                    current_instruction += 1;
                }
                Accumulator(i) => {
                    visited_instructions.push(current_instruction);
                    current_instruction += 1;
                    accumulator += i;
                }
                Jump(i) => {
                    visited_instructions.push(current_instruction);
                    current_instruction += i;
                }
            }
        }
        if visited_instructions.contains(&current_instruction) {
            InfiniteLoop(accumulator)
        } else {
            Finished(accumulator)
        }
    }
}

#[cfg(test)]
mod program_execution_execute_should {
    use super::*;

    #[test]
    fn return_0_when_current_instruction_is_noop_and_accumulator_is_0() {
        let program = Program::new(&[Noop(0)]);

        let result = program.execute();

        assert_eq!(result, Finished(0))
    }

    #[test]
    fn return_1_when_current_instruction_is_acc_1_and_accumulator_is_0() {
        let program = Program::new(&[Accumulator(1)]);

        let result = program.execute();

        assert_eq!(result, Finished(1))
    }

    #[test]
    fn return_2_when_current_instruction_is_acc_2_and_accumulator_is_0() {
        let program = Program::new(&[Accumulator(2)]);

        let result = program.execute();

        assert_eq!(result, Finished(2))
    }

    #[test]
    fn return_0_when_instructions_are_acc_3_and_acc_minus_3() {
        let program = Program::new(&[Accumulator(3), Accumulator(-3)]);

        let result = program.execute();

        assert_eq!(result, Finished(0))
    }

    #[test]
    fn return_3_when_instructions_are_acc_3_and_noop() {
        let program = Program::new(&[Accumulator(3), Noop(0)]);

        let result = program.execute();

        assert_eq!(result, Finished(3))
    }

    #[test]
    fn return_3_when_instructions_are_noop_and_acc_3() {
        let program = Program::new(&[Noop(0), Accumulator(3)]);

        let result = program.execute();

        assert_eq!(result, Finished(3))
    }

    #[test]
    fn return_7_when_instructions_are_jmp_2_acc_3_acc_7() {
        let program = Program::new(&[Jump(2), Accumulator(3), Accumulator(7)]);

        let result = program.execute();

        assert_eq!(result, Finished(7))
    }

    #[test]
    fn return_infinite_loop_when_current_instruction_has_already_been_executed() {
        let program = Program::new(&[Noop(0), Jump(-1)]);

        let result = program.execute();

        assert_eq!(result, InfiniteLoop(0))
    }
}

#[cfg(test)]
mod program_execution_last_should {
    use super::*;

    #[test]
    fn return_5_for_example_program() {
        let program = Program::new(&parse_instructions(vec![
            "nop +0".into(),
            "acc +1".into(),
            "jmp +4".into(),
            "acc +3".into(),
            "jmp -3".into(),
            "acc -99".into(),
            "acc +1".into(),
            "jmp -4".into(),
            "acc +6".into(),
        ]));

        let result = program.execute();

        assert_eq!(result, InfiniteLoop(5));
    }
}
