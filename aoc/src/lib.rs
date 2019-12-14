use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::*;
use std::path::Path;

pub use num::integer::*;
pub use serde_scan::scan;
pub use serde_scan::from_str;

// TODO: improve and generalize
pub struct Tree {
    things: HashSet<String>,
    children: HashMap<String, Vec<String>>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            things: HashSet::new(),
            children: HashMap::new(),
        }
    }

    pub fn things(&self) -> impl Iterator<Item = &String> {
        self.things.iter()
    }

    pub fn insert(&mut self, parent: &str, child: &str) {
        self.things.insert(parent.to_string());
        self.things.insert(child.to_string());
        self.children
            .entry(parent.to_string())
            .or_insert(Vec::new())
            .push(child.to_string());
    }

    pub fn depth_from_to(&self, from: &str, to: &str) -> Option<i64> {
        self.depth_from_to_recursive(from, to, 0)
    }

    fn depth_from_to_recursive(&self, from: &str, to: &str, depth: i64) -> Option<i64> {
        if from == to {
            return Some(depth);
        }
        if let Some(v) = self.children.get(from) {
            for t in v {
                if let Some(x) = self.depth_from_to_recursive(t, to, depth + 1) {
                    return Some(x);
                }
            }
        }
        return None;
    }
}

pub fn read_lines() -> (i32, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();
    let filename = &args[2];

    let input = File::open(Path::new(filename)).unwrap();
    let buffered = BufReader::new(input);
    (
        part,
        buffered
            .lines()
            .filter_map(Result::ok)
            .map(|x| x.trim().to_string())
            .collect(),
    )
}

pub fn parse_intcode(lines: &Vec<String>) -> Vec<i128> {
    let result: Vec<i128> = lines[0]
        .split(|c| c == ',')
        .map(|s| s.trim())
        .map(|v| v.parse::<i128>().unwrap())
        .collect();
    result
}
