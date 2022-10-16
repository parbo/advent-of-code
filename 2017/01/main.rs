use std::iter::*;
use std::time::Instant;

type ParsedItem = u32;
type Parsed = Vec<ParsedItem>;
type Answer = u32;

fn part1(data: &[ParsedItem]) -> Answer {
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

fn part2(data: &[ParsedItem]) -> Answer {
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
