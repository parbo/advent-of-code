use aoc::Point;
use std::collections::HashMap;
use std::iter::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Line {
    a: Point,
    b: Point,
}

type Parsed = Vec<Line>;
type Answer = i64;

fn part1(lines: &Parsed) -> Answer {
    let mut counts = HashMap::new();
    for line in lines {
        if line.a[0] == line.b[0] || line.a[1] == line.b[1] {
            for p in aoc::plot_line(line.a, line.b) {
                *counts.entry(p).or_insert(0) += 1;
            }
        }
    }
    counts.iter().filter(|(_, c)| **c >= 2).count() as Answer
}

fn part2(lines: &Parsed) -> Answer {
    let mut counts = HashMap::new();
    for line in lines {
        for p in aoc::plot_line(line.a, line.b) {
            *counts.entry(p).or_insert(0) += 1;
        }
    }
    counts.iter().filter(|(_, c)| **c >= 2).count() as Answer
}

fn parse_point(s: &str) -> Point {
    let x = aoc::split_ch(s, ',');
    [x[0].parse().unwrap(), x[1].parse().unwrap()]
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| aoc::split_w(x))
        .map(|x| Line {
            a: parse_point(x[0]),
            b: parse_point(x[2]),
        })
        .collect()
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

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }
}
