use std::iter::*;

type ParsedItem = (String, i64);
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(commands: &Parsed) -> Answer {
    let mut pos_x = 0;
    let mut depth = 0;
    for (s, x) in commands {
        match s.as_str() {
            "forward" => pos_x += x,
            "down" => depth += x,
            "up" => depth -= x,
            _ => panic!(),
        }
    }
    pos_x * depth
}

fn part2(commands: &Parsed) -> Answer {
    let mut aim = 0;
    let mut pos_x = 0;
    let mut depth = 0;
    for (s, x) in commands {
        match s.as_str() {
            "forward" => {
                pos_x += x;
                depth += aim * x;
            }
            "down" => aim += x,
            "up" => aim -= x,
            _ => panic!(),
        }
    }
    pos_x * depth
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| aoc::split_w(x))
        .map(|x| (x[0].into(), x[1].parse().unwrap()))
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
