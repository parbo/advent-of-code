use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::io::prelude::*;
use std::path::Path;
use std::iter::*;

fn solve(path: &Path) -> Result<i64, Box<Error>> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let s : i64 = buffered.lines().map(|line| line.and_then(|s| s.parse::<i64>().map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e)))).filter_map(Result::ok).sum();
    Ok(s)
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
