use std::convert::TryInto;
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

    while state.pc < program.ops.len() {
        // execute instruction
        let op = &program.ops[state.pc];

        match op.op_type {
            Instruction::IncPtr => state.ptr += op.arg,
            Instruction::DecPtr => state.ptr -= op.arg,
            Instruction::IncData => 
                state.memory[state.ptr] = state.memory[state.ptr].wrapping_add(op.arg.try_into().unwrap()),
            Instruction::DecData =>
                state.memory[state.ptr] = state.memory[state.ptr].wrapping_sub(op.arg.try_into().unwrap()),
            Instruction::Write => {
                io::stdout().write_all(&[state.memory[state.ptr]]).unwrap();
            },
            Instruction::Read => {
                io::stdin().read_exact(&mut state.memory[state.ptr..state.ptr+1]).unwrap();
            },
            Instruction::JumpNotZero => {
                if state.memory[state.ptr] != 0 {
                    state.pc = op.arg;
                } else {
                    state.pc += 1;
                }
            },
            Instruction::JumpZero => {
                if state.memory[state.ptr] == 0 {
                    state.pc = op.arg;
                } else {
                    state.pc += 1;
                }
            },
            Instruction::Invalid => (),
        }

        // Determine if pc should be incremented
        match op.op_type {
            Instruction::JumpZero => (),
            Instruction::JumpNotZero => (),
            _ => state.pc += 1,
        }
    }
}