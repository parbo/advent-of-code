use aoc::*;
use std::collections::HashSet;
use std::iter::*;
use std::time::Instant;

type ParsedItem = Vec<String>;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn has_repeats(pp: &[String]) -> bool {
    let mut seen = HashSet::new();
    for w in pp {
        if seen.contains(w) {
            return true;
        }
        seen.insert(w.clone());
    }
    false
}

fn has_anagrams(pp: &[String]) -> bool {
    let mut seen = HashSet::new();
    for w in pp {
        let w_s: String = w.chars().sorted().collect();
        if seen.contains(&w_s) {
            return true;
        }
        seen.insert(w_s);
    }
    false
}

fn part1(data: &[ParsedItem]) -> Answer {
    data.iter().filter(|x| !has_repeats(x)).count() as i64
}

fn part2(data: &[ParsedItem]) -> Answer {
    data.iter().filter(|x| !has_anagrams(x)).count() as i64
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| aoc::split_w(x).iter().map(|x| x.to_string()).collect())
        .collect()
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
