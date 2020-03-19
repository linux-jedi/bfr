use std::io;

use std::collections::LinkedList;
use std::convert::TryInto;
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
    LoopSetZero,
    LoopMovePtr,
    LoopMoveData,
}

#[derive(Clone, Copy)]
pub struct Op {
    pub op_type: Instruction,
    pub arg: i64,
}

impl Op {
    pub fn new(op_type: Instruction, arg: i64) -> Op {
        Op {
            op_type,
            arg
        }
    }

    pub fn to_string(&self) -> String {
        let mut string = String::with_capacity(2);
        
        match self.op_type {
            Instruction::IncPtr => string.push('>'),
            Instruction::DecPtr => string.push('<'),
            Instruction::IncData => string.push('+'),
            Instruction::DecData => string.push('-'),
            Instruction::JumpZero => string.push('['),
            Instruction::JumpNotZero => string.push(']'),
            Instruction::Read => string.push(','),
            Instruction::Write => string.push('.'),
            Instruction::LoopSetZero => string.push('s'),
            Instruction::LoopMovePtr => string.push('p'),
            Instruction::LoopMoveData => string.push('d'),
            _ => string.push('x'),
        }

        string.push_str(&self.arg.to_string());
        string
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

pub fn optimize_loop(ops: &Vec<Op>, start: usize) -> Vec<Op> {
    let mut new_ops: Vec<Op> = Vec::new();

    match ops.len() - start {
        // Get repeated steps
        // ex. [>], [<], [+], etc.
        2 => {
            let repeated_op = ops[start + 1];
            match repeated_op.op_type {
                Instruction::IncData | Instruction::DecData => 
                    new_ops.push(Op::new(Instruction::LoopSetZero, 0)),
                Instruction::IncPtr => 
                    new_ops.push(Op::new(Instruction::LoopMovePtr, repeated_op.arg)),
                Instruction::DecPtr =>
                    new_ops.push(Op::new(Instruction::LoopMovePtr, -repeated_op.arg)),
                _ => (),
            }
        },
        // detect -<+> and ->+<
        5 => {
            let data_op_one = ops[start + 1];
            let data_op_two = ops[start + 3];

            if  matches!(data_op_one.op_type, Instruction::DecData) &&
                matches!(data_op_two.op_type, Instruction::IncData) &&
                data_op_one.arg == 1 &&
                data_op_two.arg == 1 {
                
                let move_op_one = ops[start + 2];
                let move_op_two = ops[start + 4];
                if  matches!(move_op_one.op_type, Instruction::IncPtr) &&
                    matches!(move_op_two.op_type, Instruction::DecPtr) &&
                    move_op_one.arg == move_op_two.arg {
                    
                    new_ops.push(
                        Op::new(Instruction::LoopMoveData, move_op_one.arg)
                    );
                }
                else if matches!(move_op_one.op_type, Instruction::DecPtr) &&
                        matches!(move_op_two.op_type, Instruction::IncPtr) &&
                        move_op_one.arg == move_op_two.arg {

                    new_ops.push(
                        Op::new(Instruction::LoopMoveData, -move_op_one.arg)
                    )
                }
            }
        },
        _ => (),
    }
    new_ops
}

// Merges repeated ops and performs loop optimizations
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
                
                // Try to optimizing the loop. If successful, replace the loop with
                // the returned vector of operations.
                let mut optimized_loop = optimize_loop(&ops, bracket_offset);

                if optimized_loop.is_empty() {
                    ops[bracket_offset].arg = ops.len().try_into().unwrap();
                    ops.push(Op::new(Instruction::JumpNotZero, bracket_offset.try_into().unwrap()));
                } else {
                    ops.drain(bracket_offset..);
                    ops.append(&mut optimized_loop);
                }
                
                pc += 1;
            },
            _ => {
                // Find where the repeat ends
                let start: usize = pc;
                while pc < program_size && instructions[pc] == *instruction {
                    pc += 1;
                }

                // create new op using offset
                let num_repeats: i64 = (pc - start).try_into().unwrap();
                ops.push(Op::new(instruction.clone(), num_repeats));
            },
        }
    }

    Program {
        ops
    }
}
