use std::io;

use std::collections::LinkedList;
use std::io::BufRead;

pub struct Program {
    pub ops: Vec<Op>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    Invalid,
    IncPtr,
    DecPtr,
    IncData,
    DecData,
    Read,
    Write,
    JumpZero,
    JumpNotZero,
}

pub struct Op {
    pub op_type: Instruction,
    pub arg: usize,
}

impl Op {
    pub fn new(op_type: Instruction, arg: usize) -> Op {
        Op {
            op_type,
            arg
        }
    }
}

pub fn parse <R: BufRead>(r: &mut R) -> Result<Vec<Instruction>, io::Error> {
    let mut chars = Vec::new();
    r.read_to_end(&mut chars)?;
    let instructions: Vec<Instruction> = chars.into_iter()
        .map(|character| {
            match character as char {
                '>' => Instruction::IncPtr,
                '<' => Instruction::DecPtr,
                '+' => Instruction::IncData,
                '-' => Instruction::DecData,
                '[' => Instruction::JumpZero,
                ']' => Instruction::JumpNotZero,
                ',' => Instruction::Read,
                '.' => Instruction::Write,
                _ => Instruction::Invalid,
            }
        })
        .filter(|instruction| match instruction {
            Instruction::Invalid => false,
            _ => true,
        })
        .collect();

    Ok(instructions)
}

pub fn merge_ops(instructions: &Vec<Instruction>) -> Program {
    let mut pc: usize = 0;
    let mut ops: Vec<Op> = Vec::new();
    let program_size: usize = instructions.len();

    let mut open_bracket_stack: LinkedList<usize> = LinkedList::new();

    while pc < program_size {
        // match op to determine action
        let instruction = &instructions[pc];

        //   a. Open and close brackets
        match instruction {
            Instruction::JumpZero => {
                open_bracket_stack.push_back(ops.len());
                // update arg w/close bracket when found
                ops.push(Op::new(Instruction::JumpZero, 0)); 
                pc += 1;
            },
            Instruction::JumpNotZero => {
                // check that matching open bracket exists
                if open_bracket_stack.is_empty() {
                    panic!("Unmatched closing at {}", pc);
                }

                // create jump from open to close bracket
                let bracket_offset = open_bracket_stack.pop_back().unwrap();
                ops[bracket_offset].arg = ops.len();

                // create op for close bracket
                ops.push(Op::new(Instruction::JumpNotZero, bracket_offset));
                pc += 1;
            },
            _ => {
                // Find where the repeat ends
                let start: usize = pc;
                while pc < program_size && instructions[pc] == *instruction {
                    pc += 1;
                }

                // create new op using offset
                let num_repeats: usize = pc - start;
                ops.push(Op::new(instruction.clone(), num_repeats));
            },
        }
    }

    Program {
        ops
    }
}
