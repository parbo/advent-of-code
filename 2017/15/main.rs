use std::iter::*;
use std::time::Instant;

type Parsed = (i64, i64);
type Answer = i64;

fn part1(start: &Parsed) -> Answer {
    let mut prev_a = start.0;
    let mut prev_b = start.1;
    let mut matches = 0;
    for _ in 0..40_000_000 {
        let a = (prev_a * 16807i64) % 2147483647i64;
        let b = (prev_b * 48271i64) % 2147483647i64;
        if (a & 0xffff) == (b & 0xffff) {
            matches += 1;
        }
        prev_a = a;
        prev_b = b;
    }
    matches
}

fn part2(start: &Parsed) -> Answer {
    let mut prev_a = start.0;
    let mut prev_b = start.1;
    let mut iter_a = std::iter::from_fn(move || loop {
        let a = (prev_a * 16807i64) % 2147483647i64;
        prev_a = a;
        if a % 4 == 0 {
            return Some(a);
        }
    });
    let mut iter_b = std::iter::from_fn(move || loop {
        let b = (prev_b * 48271i64) % 2147483647i64;
        prev_b = b;
        if b % 8 == 0 {
            return Some(b);
        }
    });
    let mut matches = 0;
    for _ in 0..5_000_000 {
        let a = iter_a.next().unwrap();
        let b = iter_b.next().unwrap();
        if (a & 0xffff) == (b & 0xffff) {
            matches += 1;
        }
    }
    matches
}

fn parse(lines: &[String]) -> Parsed {
    (
        aoc::split_w(&lines[0])[4].parse().unwrap(),
        aoc::split_w(&lines[1])[4].parse().unwrap(),
    )
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
