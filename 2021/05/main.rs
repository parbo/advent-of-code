use aoc::Point;
use std::collections::HashMap;
use std::iter::*;

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

type Parsed = Vec<Line>;
type Answer = i64;

fn solve<F>(lines: &[Line], p: F) -> Answer
where
    F: Fn(&&Line) -> bool,
{
    let mut counts = HashMap::new();
    lines
        .into_iter()
        .filter(p)
        .map(|line| aoc::plot_line(line.a, line.b))
        .for_each(|points| {
            points
                .into_iter()
                .for_each(|p| *counts.entry(p).or_insert(0) += 1)
        });
    counts.iter().filter(|(_, c)| **c >= 2).count() as Answer
}

fn part1(lines: &Parsed) -> Answer {
    solve(lines, |line| {
        line.a[0] == line.b[0] || line.a[1] == line.b[1]
    })
}

fn part2(lines: &Parsed) -> Answer {
    solve(lines, |_| true)
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
