#![allow(clippy::identity_op)]

use std::iter::*;

type ParsedItem = (char, char);
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    data.iter()
        .map(|game| match game {
            ('A', 'X') => 1 + 3,
            ('A', 'Y') => 2 + 6,
            ('A', 'Z') => 3 + 0,
            ('B', 'X') => 1 + 0,
            ('B', 'Y') => 2 + 3,
            ('B', 'Z') => 3 + 6,
            ('C', 'X') => 1 + 6,
            ('C', 'Y') => 2 + 0,
            ('C', 'Z') => 3 + 3,
            _ => panic!(),
        })
        .sum()
}

fn part2(data: &Parsed) -> Answer {
    data.iter()
        .map(|game| match game {
            ('A', 'X') => 3 + 0,
            ('A', 'Y') => 1 + 3,
            ('A', 'Z') => 2 + 6,
            ('B', 'X') => 1 + 0,
            ('B', 'Y') => 2 + 3,
            ('B', 'Z') => 3 + 6,
            ('C', 'X') => 2 + 0,
            ('C', 'Y') => 3 + 3,
            ('C', 'Z') => 1 + 6,
            _ => panic!(),
        })
        .sum()
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| (x.chars().next().unwrap(), x.chars().nth(2).unwrap()))
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
