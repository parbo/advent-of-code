use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

extern crate intcode;

fn part1(numbers: &Vec<i64>) -> i64 {
    let mut m = intcode::Machine::new(&numbers, 1);
    m.run().unwrap();
    *m.outputs().last().unwrap()
}

fn part2(numbers: &Vec<i64>) -> i64 {
    let mut m = intcode::Machine::new(&numbers, 5);
    m.run().unwrap();
    *m.outputs().last().unwrap()
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
