use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn walk(buffer: &Vec<char>, ix: usize, pos: (i64, i64), steps: usize, depth: usize, places: &mut HashMap<(i64, i64), usize>) -> usize {
    let mut new_ix = ix;
    let mut new_pos = pos;
    let mut new_steps = steps;
    while new_ix < buffer.len() {
        match buffer[new_ix] {
            '^' => new_ix = walk(buffer, new_ix + 1, new_pos, new_steps, depth + 1, places),
            '$' => { return new_ix + 1; }
            '(' => new_ix = walk(buffer, new_ix + 1, new_pos, new_steps, depth + 1, places),
            ')' => { return new_ix + 1; }
            '|' => {
                new_pos = pos;
                new_steps = steps;
                new_ix += 1;
            },
            'E' | 'W' | 'N' | 'S' => {
                match buffer[new_ix] {
                    'E' => new_pos = (new_pos.0 + 1, new_pos.1),
                    'W' => new_pos = (new_pos.0 - 1, new_pos.1),
                    'N' => new_pos = (new_pos.0, new_pos.1 - 1),
                    'S' => new_pos = (new_pos.0, new_pos.1 + 1),
                    _ => panic!()
                }
                new_steps += 1;
                let s = places.entry(new_pos).or_insert(new_steps);
                if new_steps < *s {
                    *s = new_steps;
                }
                new_ix += 1;
            },
            _ => panic!()
        }
    }
    new_ix
}

fn solve(path: &Path) {
    let mut input = File::open(path).unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let mut places = HashMap::new();
    let r = buffer.trim().chars().collect();
    walk(&r, 0, (0, 0), 0, 0, &mut places);
    let max_d = places.iter().map(|(_, v)| v).max().unwrap();
    let gte_1000 = places.iter().filter(|(_, &v)| v >= 1000_usize).count();
    println!("max: {}, >= 1000: {}", max_d, gte_1000);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
