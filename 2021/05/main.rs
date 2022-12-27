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

fn part1(lines: &Parsed) -> Answer {
    solve(lines, |line| {
        line.a[0] == line.b[0] || line.a[1] == line.b[1]
    })
}

fn part2(lines: &Parsed) -> Answer {
    #[cfg(feature = "vis")]
    draw(&lines);

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

#[cfg(feature = "vis")]
fn draw(lines: &Parsed) {
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
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
