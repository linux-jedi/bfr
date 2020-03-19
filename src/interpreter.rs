use std::convert::TryInto;
use std::collections::HashMap;
use std::io::{self, Write, Read};

use crate::parser::{Instruction, Op, Program};

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

pub fn serialize_ops(ops: &Vec<Op>) -> String {
    let mut string = String::with_capacity(ops.len() * 2);
    for op in ops.iter() {
        string.push_str(&op.to_string());
    }

    string
}

// TODO: have this return Result(OK, err)
pub fn interpret_program(program: &Program) {
    let mut state = InterpreterState::new(30000);
    
    // Setup tracing infra
    let mut curr_trace: Vec<Op> = Vec::new();
    let mut trace_count: HashMap<String, usize> = HashMap::new();

    while state.pc < program.ops.len() {
        // execute instruction
        let op = &program.ops[state.pc];

        match op.op_type {
            Instruction::IncPtr => state.ptr += op.arg as usize,
            Instruction::DecPtr => state.ptr -= op.arg as usize,
            Instruction::IncData => 
                state.memory[state.ptr] = 
                    state.memory[state.ptr].wrapping_add(op.arg.try_into().unwrap()),
            Instruction::DecData =>
                state.memory[state.ptr] = 
                    state.memory[state.ptr].wrapping_sub(op.arg.try_into().unwrap()),
            Instruction::Write => {
                io::stdout().write_all(&[state.memory[state.ptr]]).unwrap();
            },
            Instruction::Read => {
                io::stdin().read_exact(&mut state.memory[state.ptr..state.ptr+1]).unwrap();
            },
            Instruction::JumpNotZero => {
                if state.memory[state.ptr] != 0 {
                    state.pc = op.arg.try_into().unwrap();
                } else {
                    state.pc += 1;
                }
            },
            Instruction::JumpZero => {
                if state.memory[state.ptr] == 0 {
                    state.pc = op.arg.try_into().unwrap();
                } else {
                    state.pc += 1;
                }
            },
            Instruction::LoopSetZero => state.memory[state.ptr] = 0,
            Instruction::LoopMovePtr => {
                while state.memory[state.ptr] != 0 {
                    state.ptr = (state.ptr as i64 + op.arg).try_into().unwrap();
                }
            },
            Instruction::LoopMoveData => {
                if state.memory[state.ptr] != 0 {
                    let move_to_ptr: usize = 
                        (state.ptr as i64 + op.arg).try_into().unwrap();
                    state.memory[move_to_ptr] = 
                        state.memory[move_to_ptr].wrapping_add(state.memory[state.ptr]);
                    state.memory[state.ptr] = 0;
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

        // Trace
        match op.op_type {
            Instruction::JumpZero => curr_trace.clear(),
            Instruction::JumpNotZero if curr_trace.len() > 0 => {
                *trace_count.entry(serialize_ops(&curr_trace)).or_insert(0) += 1;
                curr_trace.clear();
            },
            Instruction::JumpNotZero => (),
            _ => curr_trace.push(*op),
        }
    }

    // print trace
    let mut trace_count_vec: Vec<_> = trace_count.iter().collect();
    trace_count_vec.sort_by(|a, b| b.1.cmp(a.1));

    for trace in trace_count_vec.iter() {
        println!("{:10}\t{}", trace.0, trace.1);
    }
}