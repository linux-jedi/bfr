mod interpreter;
mod parser;

use std::env;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

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
    let instructions = parser::parse(&mut buf_reader).unwrap();

    // Build program
    let program = parser::merge_ops(&instructions);

    // Let's rev it up
    let now = Instant::now();
    interpreter::interpret_program(&program);

    println!("Time Elapsed: {}", now.elapsed().as_secs_f64());
}
