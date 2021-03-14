use std::error::Error;
use std::cmp;
use std::fmt;
use std::fs;

use regex::Regex;

use advent::Config;

#[derive(Debug, PartialEq)]
enum Operation {
    Acc,
    Jmp,
    Nop,
    Unknown,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operation::{:?}", self)
    }
}

#[derive(Debug)]
struct Instruction {
    pub op: Operation,
    pub arg: i32,
    pub executed: bool,
}

impl cmp::PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op && self.arg == other.arg && self.executed == other.executed
    }
}

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

fn parse_instruction(input: &str) -> Instruction {
    let re = Regex::new(r"^([a-z]{3}) ([0-9+-]+)$").unwrap();

    let re_result = re.captures(input);
    if re_result.is_some() {
        let groups = re_result.unwrap();

        // Parse the operation
        let op_str: &str = &groups[1].to_lowercase();
        let op: Operation = match op_str {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            "nop" => Operation::Nop,
            _ => Operation::Unknown,
        };

        let num_str: &str = &groups[2];
        let arg: i32 = num_str.parse().unwrap();

        return Instruction{ op: op, arg: arg, executed: false};
    }

    return Instruction{ op: Operation::Unknown, arg: 0, executed: false};
}

fn run_instructions(instruction_strings: &Vec<&str>) -> i32{
    let mut instruction_list: Vec<Instruction> = vec![];
    for ins_str in instruction_strings {
        instruction_list.push(parse_instruction(ins_str));
    }

    let mut current_instruction = &mut instruction_list[0];
    let mut accumulator = 0;
    let mut pc: usize = 0;

    loop {
        if current_instruction.executed {
            break;
        }

        // println!("Executing {:?}", current_instruction);
    
        // Inspect the current instruction
        match current_instruction.op {
            Operation::Acc => {
                pc += 1;
                accumulator += current_instruction.arg;
            },
            Operation::Nop => {
                pc += 1;
            },
            Operation::Jmp => {
                pc = add(pc, current_instruction.arg);
            },
            Operation::Unknown => {
                println!("Found unknown instruction! Arg={}", current_instruction.arg);
                pc += 1;
            },
        };

        current_instruction.executed = true;

        // Update the current instruction
        current_instruction = &mut instruction_list[pc];
    }

    return accumulator;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let instruction_list: Vec<&str> = contents.split('\n').collect();

    println!("Result = {}", run_instructions(&instruction_list));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;   

    macro_rules! test_parse_instruction {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let actual = parse_instruction(input);
                assert_eq!(expected.op, actual.op);
                assert_eq!(expected.arg, actual.arg);
            }
        )*
        }
    }
    test_parse_instruction! {
        intruction_0: (
            "nop +0",
            Instruction{op: Operation::Nop, arg: 0, executed: false}
        ),
        intruction_1: (
            "acc +1",
            Instruction{op: Operation::Acc, arg: 1, executed: false}
        ),
        intruction_2: (
            "jmp +4",
            Instruction{op: Operation::Jmp, arg: 4, executed: false}
        ),
        intruction_3: (
            "acc +3",
            Instruction{op: Operation::Acc, arg: 3, executed: false}
        ),
        intruction_4: (
            "jmp -3",
            Instruction{op: Operation::Jmp, arg: -3, executed: false}
        ),
        intruction_5: (
            "acc -99",
            Instruction{op: Operation::Acc, arg: -99, executed: false}
        ),
        intruction_6: (
            "acc +1",
            Instruction{op: Operation::Acc, arg: 1, executed: false}
        ),
        intruction_7: (
            "jmp -4",
            Instruction{op: Operation::Jmp, arg: -4, executed: false}
        ),
        intruction_8: (
            "acc +6",
            Instruction{op: Operation::Acc, arg: 6, executed: false}
        ),
    }

    #[test]
    fn test_run_instructions() {
        let test_data = vec![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4",
            "acc +6",
        ];

        assert_eq!(run_instructions(&test_data), 5);
    }
}
