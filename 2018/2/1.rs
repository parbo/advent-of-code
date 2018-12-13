use std::collections::HashMap;
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
    let mut with_2 = 0;
    let mut with_3 = 0;
    for line in &lines {
        let mut counts : HashMap<u8, i64> = HashMap::new();
        // Let's assume ascii
        for b in line.bytes() {
            *counts.entry(b).or_insert(0) += 1;
        }
        let mut has_2 = false;
        let mut has_3 = false;
        for (_, c) in counts.iter() {
            if has_2 && has_3 {
                break;
            }
            if *c == 2 {
                has_2 = true;
            }
            if *c == 3 {
                has_3 = true;
            }
        }
        if has_2 {
            with_2 += 1;
        }
        if has_3 {
            with_3 += 1;
        }
    }
    Ok(with_2 * with_3)
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
