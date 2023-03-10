use aoc::AsciiSet;
use std::iter::*;

type ParsedItem = char;
type Parsed = Vec<ParsedItem>;
type Answer = usize;

fn solve(data: &Parsed, n: usize) -> Answer {
    data.windows(n)
        .map(|w| AsciiSet::from_iter(w.iter()))
        .position(|h| h.len() == n)
        .unwrap()
        + n
}

fn part1(data: &Parsed) -> Answer {
    solve(data, 4)
}

fn part2(data: &Parsed) -> Answer {
    solve(data, 14)
}

fn parse(lines: &[String]) -> Parsed {
    lines[0].chars().collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
