use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn solve(path: &Path) -> Result<i64, Box<Error>> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut frequency : i64 = 0;
    let mut frequencies : HashSet<i64> = HashSet::new();
    loop {
        for line in &lines {
            let v = line.parse::<i64>()?;
            frequency += v;
            if !frequencies.insert(frequency) {
                return Ok(frequency)
            }
        }
    }
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
