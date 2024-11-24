use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::iter::*;

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|c| c.to_string()).collect()
}

#[aoc(day2, part1)]
fn solve_pt1(lines: &[String]) -> i64 {
    let mut with_2 = 0;
    let mut with_3 = 0;
    for line in lines {
        let mut counts: HashMap<u8, i64> = HashMap::new();
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
    with_2 * with_3
}

#[derive(Debug)]
struct AdventError {
    details: String,
}

impl AdventError {
    fn new(msg: &str) -> AdventError {
        AdventError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for AdventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AdventError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[aoc(day2, part2)]
fn solve_pt2(lines: &[String]) -> Result<String, Box<dyn Error>> {
    for a in 0..lines.len() {
        let line_a = &lines[a];
        for b in 1..lines.len() {
            let line_b = &lines[b];
            // Let's assume ascii
            let different = line_a
                .bytes()
                .zip(line_b.bytes())
                .filter(|(x, y)| x != y)
                .count();
            if different == 1 {
                let common: Vec<u8> = line_a
                    .bytes()
                    .zip(line_b.bytes())
                    .filter(|(x, y)| x == y)
                    .map(|(x, _)| x)
                    .collect();
                return String::from_utf8(common).map_err(|e| e.into());
            }
        }
    }
    Err(Box::new(AdventError::new("OH NOES")))
}
