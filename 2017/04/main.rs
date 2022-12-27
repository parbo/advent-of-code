use aoc::*;
use std::collections::HashSet;
use std::iter::*;

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

fn part1(data: &Parsed) -> Answer {
    data.iter().filter(|x| !has_repeats(x)).count() as i64
}

fn part2(data: &Parsed) -> Answer {
    data.iter().filter(|x| !has_anagrams(x)).count() as i64
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| aoc::split_w(x).iter().map(|x| x.to_string()).collect())
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
