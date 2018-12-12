use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;
use std::mem;

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
    let mut next = vec![];
    let gens = 50000000000_i64;
    let mut sums = HashMap::new();
    let mut loop_c = -1;
    for gen in 0..gens {
        initial.insert(0, '.');
        initial.insert(0, '.');
        inserted += 2;
        initial.push('.');
        initial.push('.');
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
        let mut b = 0;
        while next[b] == '.' {
            b += 1;
        }
        let mut a = 0;
        let l = next.len();
        while next[l - a - 1] == '.' {
            a += 1;
        }
        initial = next[b..(l-a)].to_vec();
        let s : Vec<i64> = initial.iter().enumerate().filter(|(n, c)| **c == '#').map(|(n, c)|  n as i64 - inserted).collect();
        if sums.contains_key(&s) {
            loop_c = *sums.get(&s).unwrap();
            break;
        } else {
            sums.insert(s, gen);
        }
        if gen % 1000 == 0 {
            println!("gen: {}, {:?}", gen, initial);
        }
    }
    println!("{:?}", sums);
    let rem = gens - loop_c * (gens / loop_c);
    for (s, g) in sums {
        if g == rem {
            return s.iter().sum();
        }
    }
    return -1;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{}", result);
}
