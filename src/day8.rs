use crate::day;
use crate::file;

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day8 {

}

impl day::Day for Day8 {

    fn puzzle1(&self) {
        println!("Day 8, puzzle 1");

        let instructions = build_instructions(&file::lines("res/day8_1.txt"));
        let (_, accumulated) = run_instructions(instructions);

        println!("{}", accumulated);
    }

    fn puzzle2(&self) {
        println!("Day 8, puzzle 2");

        let instructions = build_instructions(&file::lines("res/day8_1.txt"));
        let (_, accumulated) = fix_instructions(instructions);

        println!("{}", accumulated);
    }

}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Operation {
    ACC,
    JMP,
    NOP
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Instruction {
    operation: Operation,
    argument: i32,
    executed: bool
}

impl Instruction {

    fn new(operation: Operation, argument: i32) -> Instruction {
        Instruction { operation, argument, executed: false }
    }

}

fn run_instructions(mut instructions: Vec<Instruction>) -> (bool, i32) {

    let mut accumulator: i32 = 0;
    let mut idx: i32 = 0;
    let mut cont = true;
    while cont {
        if idx as usize >= instructions.len() {
            cont = false;
        }
        else {
            let instruction: &mut Instruction = instructions.get_mut(idx as usize).unwrap();
            if instruction.executed {
                cont = false;
            } else {
                if instruction.operation == Operation::ACC {
                    accumulator += instruction.argument;
                    idx += 1;
                } else if instruction.operation == Operation::JMP {
                    idx += instruction.argument;
                } else {
                    idx += 1;
                }
            }
            instruction.executed = true;
        }
    }
    (idx as usize == instructions.len(), accumulator)
}

fn fix_instructions(instructions: Vec<Instruction>) -> (bool, i32) {
    let mut idx: usize = 0;
    while (idx) < instructions.len() {
        if instructions[idx].operation != Operation::ACC {
            let mut cloned= instructions.to_vec();
            if cloned[idx].operation == Operation::JMP {
                cloned[idx].operation = Operation::NOP;
            } else {
                cloned[idx].operation = Operation::JMP;
            }
            let (terminated_ok, accumulated) = run_instructions(cloned);
            if terminated_ok {
                return (true, accumulated);
            }
        }
        idx += 1;
    }
    (false, 0)
}

fn build_instructions(instructions: &Vec<String>) -> Vec<Instruction> {
    instructions
        .iter()
        .map(|s| build_instruction(s))
        .collect()
}

lazy_static! {
    static ref RE: Regex = Regex::new("([[:alpha:]]+)\\s+\\+?([-0-9]+)").unwrap();
}

fn build_instruction(instruction: &str) -> Instruction {
    let cap = RE.captures(instruction).unwrap();
    let operation = match &cap[1] {
        "acc" => Operation::ACC,
        "jmp" => Operation::JMP,
        _ => Operation::NOP
    };
    let amount = cap[2].parse::<i32>().expect("Could not parse amount");
    Instruction::new(operation, amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_instructions_1() {
        let instructions = build_instructions(
            &vec![
                "nop +0".to_string(),
                "acc +1".to_string(),
                "jmp +4".to_string(),
                "acc +3".to_string(),
                "jmp -3".to_string(),
                "acc -99".to_string(),
                "acc +1".to_string(),
                "jmp -4".to_string(),
                "acc +6".to_string(),
            ]
        );
        assert_eq!(run_instructions(instructions), (false, 5));
    }

    #[test]
    fn test_run_instructions_2() {
        let instructions = build_instructions(
            &vec![
                "nop +0".to_string(),
                "acc +1".to_string(),
                "jmp +4".to_string(),
                "acc +3".to_string(),
                "jmp -3".to_string(),
                "acc -99".to_string(),
                "acc +1".to_string(),
                "nop -4".to_string(),
                "acc +6".to_string(),
            ]
        );
        assert_eq!(run_instructions(instructions), (true, 8));
    }

    #[test]
    fn test_fix_instructions() {
        let instructions = build_instructions(
            &vec![
                "nop +0".to_string(),
                "acc +1".to_string(),
                "jmp +4".to_string(),
                "acc +3".to_string(),
                "jmp -3".to_string(),
                "acc -99".to_string(),
                "acc +1".to_string(),
                "jmp -4".to_string(),
                "acc +6".to_string(),
            ]
        );
        assert_eq!(fix_instructions(instructions), (true, 8));
    }

}