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
    let gens = 50000000000_i64;
    let mut sums = HashMap::new();
    let mut gen_c = (-1, -1);
    let mut inserted : i64 = 0;
    let mut last_sum = 0;
    for gen in 0..gens {
        initial.insert(0, '.');
        initial.insert(0, '.');
        inserted += 2;
        initial.push('.');
        initial.push('.');
        let mut next = vec![];
        next.resize(initial.len(), '.');
        for i in 0..(initial.len() - 5) {
            for (p, res) in &patterns {
                let mut eq = 0;
                for x in 0..5 {
                    if p[x] == initial[x+i] {
                        eq += 1;
                    }
                }
                if eq == 5 {
                    next[i+2] = *res;
                    break;
                }
            }
        }
        initial = next;
        let first_non_empty = initial.iter().enumerate().find(|(_, c)| **c == '#').unwrap().0;
        let r : Vec<i64> = initial.iter().enumerate().filter(|(_, c)| **c == '#').map(|(n, _)| (n - first_non_empty) as i64).collect();
        last_sum = initial.iter().enumerate().map(|(n, c)| if *c == '#' { n as i64 - inserted } else { 0 }).sum::<i64>();
        if sums.contains_key(&r) {
            gen_c = (gen, inserted);
            break;
        } else {
            sums.insert(r.clone(), (gen, inserted));
        }
    }
    let rem = gens - gen_c.0 - 1;
    let c = initial.iter().filter(|c| **c == '#').count() as i64;
    return last_sum + rem * c;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{}", result);
}
