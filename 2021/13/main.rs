use aoc::{Grid, GridDrawer, Point};
use std::time::Instant;
use std::{collections::HashMap, iter::*};

struct Manual {
    grid: HashMap<Point, char>,
    folds: Vec<(usize, i64)>,
}

type Parsed = Manual;
type Answer = i64;

fn fold_grid(
    grid: &HashMap<Point, char>,
    ext: (Point, Point),
    fold: (usize, i64),
) -> (HashMap<Point, char>, (Point, Point)) {
    let mut g = HashMap::new();
    let (min, mut max) = ext;
    for p in grid.keys() {
        let mut np = *p;
        np[fold.0] = if np[fold.0] < fold.1 {
            np[fold.0]
        } else {
            max[fold.0] - np[fold.0]
        };
        if let Some(v) = grid.get_value(*p) {
            g.set_value(np, v);
        }
    }
    max[fold.0] = fold.1 - 1;
    (g, (min, max))
}

fn part1(manual: &Parsed) -> Answer {
    let e = manual.grid.extents();
    let (g, _ext) = fold_grid(&manual.grid, e, manual.folds[0]);
    g.len() as Answer
}

fn part2(manual: &Parsed) -> Answer {
    let mut d = aoc::PrintGridDrawer::new(|c| c);
    let mut e = manual.grid.extents();
    let mut g = manual.grid.clone();
    for fold in &manual.folds {
        let (gg, ee) = fold_grid(&g, e, *fold);
        g = gg;
        e = ee;
    }
    d.draw(&g);
    0
}

fn parse(lines: &[String]) -> Parsed {
    let sections = aoc::split_by_empty_line(lines);
    let grid: HashMap<Point, char> = sections[0]
        .iter()
        .map(|x| aoc::split_ch(x, ','))
        .map(|x| ([x[0].parse().unwrap(), x[1].parse().unwrap()], '#'))
        .collect();
    let folds = sections[1]
        .iter()
        .map(|s| aoc::split_ch(&s[11..], '='))
        .map(|x| {
            let c = x[0].chars().next().unwrap();
            let d = x[1].parse().unwrap();
            (if c == 'x' { 0 } else { 1 }, d)
        })
        .collect();
    Manual { grid, folds }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "6,10".into(),
            "0,14".into(),
            "9,10".into(),
            "0,3".into(),
            "10,4".into(),
            "4,11".into(),
            "6,0".into(),
            "6,12".into(),
            "4,1".into(),
            "0,13".into(),
            "10,12".into(),
            "3,4".into(),
            "3,0".into(),
            "8,4".into(),
            "1,10".into(),
            "2,14".into(),
            "8,10".into(),
            "9,0".into(),
            "".into(),
            "fold along y=7".into(),
            "fold along x=5".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 17);
    }

    #[test]
    fn test_part2() {
        let manual = parse(&example());
        let mut d = aoc::PrintGridDrawer::new(|c| c);
        let mut e = manual.grid.extents();
        let mut g = manual.grid.clone();
        for fold in &manual.folds {
            let (gg, ee) = fold_grid(&g, e, *fold);
            g = gg;
            e = ee;
            d.draw(&g);
            println!("ext: {:?}", e);
        }
        assert_eq!(g.len(), 16);
    }
}
