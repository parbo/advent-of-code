use aoc::Point;
use std::collections::HashMap;
use std::iter::*;

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

type ParsedItem = Line;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn solve<F>(lines: &[Line], p: F) -> Answer
where
    F: Fn(&&Line) -> bool,
{
    let mut counts = HashMap::new();
    lines
        .iter()
        .filter(p)
        .map(|line| aoc::plot_line(line.a, line.b))
        .for_each(|points| {
            points
                .into_iter()
                .for_each(|p| *counts.entry(p).or_insert(0) += 1)
        });
    counts.iter().filter(|(_, c)| **c >= 2).count() as Answer
}

fn part1(lines: &[ParsedItem]) -> Answer {
    solve(lines, |line| {
        line.a[0] == line.b[0] || line.a[1] == line.b[1]
    })
}

fn part2(lines: &[ParsedItem]) -> Answer {
    solve(lines, |_| true)
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| aoc::split_w(x))
        .map(|x| Line {
            a: aoc::parse_point(x[0]).unwrap(),
            b: aoc::parse_point(x[2]).unwrap(),
        })
        .collect()
}

fn draw(lines: &[Line]) -> Answer {
    let mut counts = HashMap::new();
    lines
        .iter()
        .map(|line| aoc::plot_line(line.a, line.b))
        .for_each(|points| {
            points
                .into_iter()
                .for_each(|p| *counts.entry(p).or_insert(0) += 1)
        });
    let min_x = counts.keys().map(|x| x[0]).min().unwrap();
    let max_x = counts.keys().map(|x| x[0]).max().unwrap();
    let min_y = counts.keys().map(|x| x[1]).min().unwrap();
    let max_y = counts.keys().map(|x| x[1]).max().unwrap();
    let mut gd = aoc::BitmapGridDrawer::new(
        |x| match x {
            0 => [0, 0, 0],
            1 => [0, 255, 0],
            _ => [255, 0, 0],
        },
        "ppm/day05",
    );
    gd.set_rect(([min_x, min_y], [max_x, max_y]));
    counts.clear();
    lines
        .iter()
        .map(|line| aoc::plot_line(line.a, line.b))
        .for_each(|points| {
            points.into_iter().for_each(|p| {
                *counts.entry(p).or_insert(0) += 1;
            });
            gd.draw_grid(&counts);
            gd.save_image();
        });
    0
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else if part == 2 {
        part2(&parsed)
    } else {
        draw(&parsed)
    };
    println!("{}", result);
}
