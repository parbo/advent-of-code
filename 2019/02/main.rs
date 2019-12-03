use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

extern crate intcode;

fn run_all(numbers: &Vec<i64>) -> Option<(i64, i64)> {
    for ai in 0..=99 {
        for bi in 0..=99 {
            let mut m = intcode::Machine::new(&numbers);
            // Init
            m.write(1, ai);
            m.write(2, bi);
            let res = m.run();
            if res == Some(19690720) {
                return Some((ai, bi));
            }
        }
    }
    None
}

fn part1(numbers: &Vec<i64>) -> i64 {
    let mut m = intcode::Machine::new(&numbers);
    // Init
    m.write(1, 12);
    m.write(2, 02);
    m.run().unwrap()
}

fn part2(numbers: &Vec<i64>) -> i64 {
    let (noun, verb) = run_all(numbers).unwrap();
    100 * noun + verb
}

fn input(path: &Path) -> Vec<i64> {
    let mut inp = File::open(path).unwrap();
    let mut buffer = String::new();
    inp.read_to_string(&mut buffer).unwrap();

    let result : Vec<i64> = buffer.split(|c| c == ',').map(|s| s.trim()).map(|v| v.parse::<i64>().unwrap()).collect();
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();
    let filename = &args[2];

    let parsed = input(Path::new(&filename));

    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}
