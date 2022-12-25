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
    let start_time = Instant::now();
    let (part, lines) = aoc::read_lines();
    let io_time = Instant::now();
    let parsed = parse(&lines);
    let parse_time = Instant::now();
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    let done_time = Instant::now();
    println!(
        "read: {:?}, parse: {:?}, solve: {:?}\n",
        io_time.duration_since(start_time),
        parse_time.duration_since(io_time),
        done_time.duration_since(parse_time)
    );
    println!("{}", result);
}
