use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn solve(path: &Path) -> i64 {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut initial = vec![];
    let mut patterns = vec![];
    for line in lines {
        if let Some(first) = line.chars().next() {
            if first == 'i' {
                initial = line[15..].chars().collect();
            } else if first == '#' || first == '.' {
                let t : Vec<char> = line[0..5].chars().collect();
                patterns.push((t, line[9..10].chars().next().unwrap()));
            }
        }
    }
    println!("{:?}", patterns);
    let mut inserted = 0;
    for _ in 0..20 {
        initial.insert(0, '.');
        initial.insert(0, '.');
        inserted += 2;
        initial.push('.');
        initial.push('.');
        let mut next = vec![];
        next.resize(initial.len(), '.');
        for i in 0..(initial.len() - 5) {
            let mut matches = 0;
            for (p, res) in &patterns {
                let mut eq = 0;
                for x in 0..5 {
                    if p[x] == initial[x+i] {
                        eq += 1;
                    }
                }
                if eq == 5 {
                    next[i+2] = *res;
                    matches += 1;
                }
            }
            assert!(matches == 1);
        }
        initial = next;
        println!("{:?}", initial);
    }
    return initial.iter().enumerate().map(|(n, c)| if *c == '#' { n as i64 - inserted } else { 0 }).sum();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{}", result);
}
