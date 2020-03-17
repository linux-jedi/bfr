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
pub fn interpret_program(program: &Program, jump_table: &Vec<usize>) {
    let mut state = InterpreterState::new(30000);

    while state.pc < program.instructions.len() {
        // execute instruction
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
                    state.pc = jump_table[state.pc];
                } else {
                    state.pc += 1;
                }
            },
            Instruction::JumpZero => {
                if state.memory[state.ptr] == 0 {
                    state.pc = jump_table[state.pc];
                } else {
                    state.pc += 1;
                }
            },
            Instruction::Invalid => (),
        }

        // Determine if pc should be incremented
        match instruction {
            Instruction::JumpZero => (),
            Instruction::JumpNotZero => (),
            _ => state.pc += 1,
        }
    }
}