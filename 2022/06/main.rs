use std::{collections::HashSet, iter::*};

type ParsedItem = char;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    let mut i = 0;
    for w in data.as_slice().windows(4) {
        let mut h = HashSet::new();
        for c in w {
            h.insert(c);
        }
        if h.len() == 4 {
            dbg!(w);
            break;
        }
        i += 1;
    }
    i + 4
}

fn part2(data: &Parsed) -> Answer {
    let mut i = 0;
    for w in data.as_slice().windows(14) {
        let mut h = HashSet::new();
        for c in w {
            h.insert(c);
        }
        if h.len() == 14 {
            dbg!(w);
            break;
        }
        i += 1;
    }
    i + 14
}

fn parse(lines: &[String]) -> Parsed {
    lines[0].chars().collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

