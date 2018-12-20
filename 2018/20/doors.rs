use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;
use std::error::Error as StdError;
use std::fmt;

fn walk(buffer: &Vec<char>, ix: usize, pos: (i64, i64), steps: usize, depth: usize, places: &mut HashMap<(i64, i64), usize>) -> usize {
    let mut new_ix = ix;
    let mut new_pos = pos;
    let mut new_steps = steps;
    loop {
        if new_ix >= buffer.len() {
            break;
        }
        // for _ in 0..depth {
        //     print!(" ");
        // }
        // println!("{}, {}, {:?}", buffer[new_ix], new_ix, new_pos);
        match buffer[new_ix] {
            '^' => {
                new_ix = walk(buffer, new_ix + 1, new_pos, new_steps, depth + 1, places);
            },
            '$' => {
                return new_ix + 1;
            },
            '(' => {
                new_ix = walk(buffer, new_ix + 1, new_pos, new_steps, depth + 1, places);
            },
            ')' => {
                return new_ix + 1;
            },
            '|' => {
                new_pos = pos;
                new_steps = steps;
                new_ix += 1;
            },
            'E' => {
                new_pos = (new_pos.0 + 1, new_pos.1);
                new_steps += 1;
                let s = places.entry(new_pos).or_insert(new_steps);
                if new_steps < *s {
                    *s = new_steps;
                }
                new_ix += 1;
            },
            'W' => {
                new_pos = (new_pos.0 - 1, new_pos.1);
                new_steps += 1;
                let s = places.entry(new_pos).or_insert(new_steps);
                if new_steps < *s {
                    *s = new_steps;
                }
                new_ix += 1;
            },
            'N' => {
                new_pos = (new_pos.0, new_pos.1 - 1);
                new_steps += 1;
                let s = places.entry(new_pos).or_insert(new_steps);
                if new_steps < *s {
                    *s = new_steps;
                }
                new_ix += 1;
            },
            'S' => {
                new_pos = (new_pos.0, new_pos.1 + 1);
                new_steps += 1;
                let s = places.entry(new_pos).or_insert(new_steps);
                if new_steps < *s {
                    *s = new_steps;
                }
                new_ix += 1;
            },
            x => {
                println!("x: {}", x);
                panic!()
            }
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
    let mut max_d = 0;
    let mut gte_1000 = 0;
    for (p, d) in places {
        if d > max_d {
            max_d = d;
        }
        if d >= 1000 {
            gte_1000 += 1;
        }
    }
    println!("max: {}, >= 1000: {}", max_d, gte_1000);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
