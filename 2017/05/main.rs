use std::iter::*;
use std::time::Instant;

type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(data: &[ParsedItem]) -> Answer {
    let mut data = data.to_vec();
    let mut ix = 0;
    let mut jumps = 0;
    while ix >= 0 && ix < data.len() as i64 {
        let jump = &mut data[ix as usize];
        ix += *jump;
        *jump += 1;
        jumps += 1;
    }
    jumps
}

fn part2(data: &[ParsedItem]) -> Answer {
    let mut data = data.to_vec();
    let mut ix = 0;
    let mut jumps = 0;
    while ix >= 0 && ix < data.len() as i64 {
        let jump = &mut data[ix as usize];
        ix += *jump;
        if *jump >= 3 {
            *jump -= 1;
        } else {
            *jump += 1;
        }
        jumps += 1;
    }
    jumps
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
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
