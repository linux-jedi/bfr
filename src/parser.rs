use std::io;
use std::io::BufRead;

pub struct Program {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug)]
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

pub fn parse <R: BufRead>(r: &mut R) -> Result<Program, io::Error> {
    let mut chars = Vec::new();
    r.read_to_end(&mut chars)?;
    
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

    Ok(Program {instructions})
}
