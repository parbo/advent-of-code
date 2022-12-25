use std::iter::*;

type ParsedItem = (String, i64);
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(commands: &[ParsedItem]) -> Answer {
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

fn part2(commands: &[ParsedItem]) -> Answer {
    let mut aim = 0;
    let mut pos_x = 0;
    let mut depth = 0;
    for (s, x) in commands {
	match s.as_str() {
	    "forward" => {
		pos_x += x;
		depth += aim * x;
	    },
	    "down" => aim += x,
	    "up" => aim -= x,
	    _ => panic!(),
	}
    }
    pos_x * depth
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| aoc::split_w(x)).map(|x| (x[0].into(), x[1].parse().unwrap())).collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}
