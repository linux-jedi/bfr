mod interpreter;
mod opt;
mod parser;

use std::env;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
    // Get Arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("not enough arguments provided.");
    }

    // Parse File
    let file_path = Path::new(&args[1]);
    let file = match File::open(&file_path) 
    {
        Err(why) => panic!("couldn't open {}: {}", file_path.display(), why.to_string()),
        Ok(file) => file,
    };  

    let mut buf_reader = BufReader::new(file);
    let program = parser::parse(&mut buf_reader).unwrap();

    // Build jump table
    let jump_table = opt::compute_jumptable(&program);

    // Let's rev it up
    interpreter::interpret_program(&program, &jump_table);
}
