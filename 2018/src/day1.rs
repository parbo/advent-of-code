use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::*;
use std::collections::HashSet;

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|s| s.parse::<i64>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve_pt1(input: &Vec<i64>) -> i64 {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn solve_pt2(input: &Vec<i64>) -> i64 {
    let mut frequency : i64 = 0;
    let mut frequencies : HashSet<i64> = HashSet::new();
    loop {
        for v in input {
            frequency += v;
            if !frequencies.insert(frequency) {
                return frequency;
            }
        }
    }
}
