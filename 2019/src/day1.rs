use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|s| s.parse::<i64>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn solve_pt1(input: &Vec<i64>) -> i64 {
    return 0;
}

#[aoc(day1, part2)]
fn solve_pt2(input: &Vec<i64>) -> i64 {
    return 0;
}
