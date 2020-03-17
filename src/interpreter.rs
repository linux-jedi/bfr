use std::io::{self, Write, Read};

use crate::parser::Instruction;
use crate::parser::Program;

struct InterpreterState {
    memory: Vec<u8>,
    pc: usize,
    ptr: usize,
}

impl InterpreterState {
    pub fn new(mem_size: usize) -> InterpreterState {
        InterpreterState {
            memory: vec![0; mem_size],
            pc: 0,
            ptr: 0,
        }
    }
}

// TODO: have this return Result(OK, err)
pub fn interpret_program(program: &Program) {
    let mut state = InterpreterState::new(30000);

    while state.pc < program.instructions.len() {
        // run instructions
        let instruction = &program.instructions[state.pc];

        match instruction {
            Instruction::IncPtr => state.ptr += 1,
            Instruction::DecPtr => state.ptr -= 1,
            Instruction::IncData => state.memory[state.ptr] = state.memory[state.ptr].wrapping_add(1),
            Instruction::DecData => state.memory[state.ptr] = state.memory[state.ptr].wrapping_sub(1),
            Instruction::Write => {
                io::stdout().write_all(&[state.memory[state.ptr]]).unwrap();
            },
            Instruction::Read => {
                io::stdin().read_exact(&mut state.memory[state.ptr..state.ptr+1]).unwrap();
            },
            Instruction::JumpNotZero => {
                if state.memory[state.ptr] != 0 {
                    let mut bracket_nesting: i32 = 1;
                    let saved_pc = state.pc;

                    while bracket_nesting > 0 && state.pc > 0 {
                        state.pc -= 1;

                        match program.instructions[state.pc] {
                            Instruction::JumpNotZero => bracket_nesting += 1,
                            Instruction::JumpZero => bracket_nesting -= 1,
                            _ => (),
                        }
                    }
                    state.pc -= 1; // jump back to instruction before '['

                    if bracket_nesting > 0 {
                        panic!("unmatched '[' at {}", saved_pc);
                    }
                }
            },
            Instruction::JumpZero => {
                if state.memory[state.ptr] == 0 {
                    let mut bracket_nesting: i32 = 1;
                    let saved_pc = state.pc;

                    // find the closing bracket
                    state.pc += 1;
                    while bracket_nesting > 0 && state.pc < program.instructions.len() {
                        match program.instructions[state.pc] {
                            Instruction::JumpNotZero => bracket_nesting -= 1,
                            Instruction::JumpZero => bracket_nesting += 1,
                            _ => (),
                        }
                        state.pc += 1;
                    }
                    state.pc -= 1; // move back to closing bracket

                    // TODO: make this return error
                    if bracket_nesting > 0 {
                        panic!("unmatched '[' at {}", saved_pc);
                    }
                }
            },
            Instruction::Invalid => (),
        }

        state.pc += 1;
    }
}