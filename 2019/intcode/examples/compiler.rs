use intcode::{parse, tokenize, ParseError};
use std::env;
use std::fs::read_to_string;
use std::iter::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: compiler filename");
        return;
    }

    let filename = &args[1];
    let s = read_to_string(filename).unwrap();
    let lines: Vec<_> = s.split("\n").collect();
    match parse(&tokenize(&s)) {
        Ok(program) => {
            println!("{:?}", program);
        }
        Err(ParseError::SyntaxError(s, loc)) => {
            println!("{}", s);
            println!("{}", lines[loc.start.line]);
            println!("{:indent$}^", "", indent = loc.start.column);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
