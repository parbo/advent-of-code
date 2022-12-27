use std::iter::*;

type ParsedItem = u32;
type Parsed = Vec<ParsedItem>;
type Answer = u32;

fn part1(data: &Parsed) -> Answer {
    (0..data.len())
        .map(|i| {
            if data[i] == data[(i + 1) % data.len()] {
                data[i]
            } else {
                0
            }
        })
        .sum()
}

fn part2(data: &Parsed) -> Answer {
    (0..data.len())
        .map(|i| {
            if data[i] == data[(i + data.len() / 2) % data.len()] {
                data[i]
            } else {
                0
            }
        })
        .sum()
}

fn parse(lines: &[String]) -> Parsed {
    lines[0].chars().map(|x| x.to_digit(10).unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
