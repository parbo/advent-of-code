use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

extern crate intcode;

fn part1(numbers: &Vec<i128>) -> i128 {
    let mut m = intcode::Machine::new(&numbers, &vec![1]);
    m.run().unwrap();
    *m.outputs().last().unwrap()
}

fn part2(numbers: &Vec<i128>) -> i128 {
    let mut m = intcode::Machine::new(&numbers, &vec![5]);
    m.run().unwrap();
    *m.outputs().last().unwrap()
}

fn input(path: &Path) -> Vec<i128> {
    let mut inp = File::open(path).unwrap();
    let mut buffer = String::new();
    inp.read_to_string(&mut buffer).unwrap();

    let result : Vec<i128> = buffer.split(|c| c == ',').map(|s| s.trim()).map(|v| v.parse::<i128>().unwrap()).collect();
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
