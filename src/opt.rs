use crate::parser::Program;
use crate::parser::Instruction;

pub fn compute_jumptable(program: &Program) -> Vec<usize> {
    let program_size = program.instructions.len();
    let mut jump_table: Vec<usize> = vec![0; program_size];
    let mut pc: usize = 0;

    while pc < program_size {
        let instruction = &program.instructions[pc];
        if let Instruction::JumpZero = instruction {
            let mut bracket_nesting: u32 = 1;
            let mut seek: usize = pc;

            seek += 1;
            while bracket_nesting > 0 && seek < program_size {
                match program.instructions[seek] {
                    Instruction::JumpNotZero => bracket_nesting -= 1,
                    Instruction::JumpZero => bracket_nesting += 1,
                    _ => (),
                }
                seek += 1;
            }

            if bracket_nesting == 0 {
                jump_table[pc] = seek - 1;
                jump_table[seek - 1] = pc;
            } else {
                panic!("unmatched '[' at {}", pc);
            }
        }
        pc += 1;
    }
    jump_table
}