use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

#[derive(Debug)]
struct AdventError {
    details: String
}

impl AdventError {
    fn new(msg: &str) -> AdventError {
        AdventError{details: msg.to_string()}
    }
}

impl fmt::Display for AdventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for AdventError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn solve(path: &Path) -> Result<String, Box<Error>> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    for a in 0..lines.len() {
        let line_a = &lines[a];
        for b in 1..lines.len() {
            let line_b = &lines[b];
            // Let's assume ascii
            let different = line_a.bytes().zip(line_b.bytes()).filter(|(x, y)| x != y).count();
            if different == 1 {
                let common : Vec<u8> = line_a.bytes().zip(line_b.bytes()).filter(|(x, y)| x == y).map(|(x, _)| x).collect();
                return String::from_utf8(common).map_err(|e| e.into());
            }
        }
    }
    Err(Box::new(AdventError::new("OH NOES")))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    match result {
        Ok(str) => println!("{}", str),
        Err(err) => println!("{}", err)
    }
}
