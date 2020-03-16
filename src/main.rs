use std::env;

use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{self, Write, Read};
use std::path::Path;


struct Program {
	pub instructions: Vec<Instruction>,
}

struct InterpreterState {
    memory: Vec<u8>,
    pc: usize,
    ptr: usize,
}

#[derive(Debug)]
enum Instruction {
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


impl InterpreterState {
    fn new(mem_size: usize) -> InterpreterState {
        InterpreterState {
            memory: vec![0; mem_size],
            pc: 0,
            ptr: 0,
        }
    }
}

fn parse <R: BufRead>(r: &mut R) -> Program {
    let mut chars = Vec::new();
    r.read_to_end(&mut chars).unwrap();
    
    let instructions: Vec<Instruction> = chars.iter()
        .map(|&character| {
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
        .collect();

    Program {instructions}
}

// TODO: have this return Result(OK, err)
fn interpret_program(program: &Program) {
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
                // println!("Write {}", state.memory[state.ptr]);
            },
            Instruction::Read => {
                io::stdin().read_exact(&mut state.memory[state.ptr..state.ptr+1]).unwrap();
                // println!("Got Number {:?}", state.memory[state.ptr]);
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

fn main() {
    // Get Arguments
    let args: Vec<String> = env::args().collect();

    // Parse File
    let file_path = Path::new(&args[1]);
    let file = match File::open(&file_path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", file_path.display(),
                                                   why.description()),
        Ok(file) => file,
    };  

    let mut buf_reader = BufReader::new(file);
    let program = parse(&mut buf_reader);

    // Let's rev it up
    interpret_program(&program);
}
