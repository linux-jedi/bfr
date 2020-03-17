mod interpreter;
mod parser;

use std::env;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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
    let program = parser::parse(&mut buf_reader);

    // Let's rev it up
    interpreter::interpret_program(&program);
}
