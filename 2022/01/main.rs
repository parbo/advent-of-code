use std::iter::*;

type ParsedItem = i64;
type Parsed = Vec<Vec<ParsedItem>>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    data.iter().map(|x| x.iter().sum()).max().unwrap()
}

fn part2(data: &Parsed) -> Answer {
    let mut d = data.iter().map(|x| x.iter().sum()).collect::<Vec<i64>>();
    d.sort_by(|a, b| b.cmp(&a));
    d.iter().take(3).sum()
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_by_empty_line(lines)
        .iter()
        .map(|x| x.iter().map(|x| x.parse::<i64>().unwrap()).collect())
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
