use std::iter::*;

#[derive(Debug)]
enum Dir {
    L,
    R,
}

#[derive(Debug)]
struct Rule {
    dir: Dir,
    num: i64,
}

type ParsedItem = Rule;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let mut zeroes = 0;
    let mut current = 50;
    for d in data {
        match d {
            Rule { dir: Dir::L, num } => {
                current -= num;
            }
            Rule { dir: Dir::R, num } => {
                current += num;
            }
        }
        current %= 100;
        if current == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

fn part2(data: &Parsed) -> i64 {
    let mut zeroes = 0;
    let mut current = 50;
    for d in data {
        let (a, num) = match d {
            Rule { dir: Dir::L, num } => (-1, *num),
            Rule { dir: Dir::R, num } => (1, *num),
        };
        for _ in 0..num {
            current += a;
            current %= 100;
            if current == 0 {
                zeroes += 1;
            }
        }
    }
    zeroes
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let dir = if x.starts_with('L') { Dir::L } else { Dir::R };
            let num = x[1..].parse().unwrap();
            Rule { dir, num }
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
