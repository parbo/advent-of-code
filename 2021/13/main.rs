use aoc::{Grid, GridDrawer, Point};

use std::{collections::HashMap, iter::*};

struct Manual {
    grid: HashMap<Point, char>,
    folds: Vec<(usize, i64)>,
}

type Parsed = Manual;
type Answer = i64;

fn fold_grid(grid: &HashMap<Point, char>, fold: (usize, i64)) -> HashMap<Point, char> {
    let mut g = HashMap::new();
    for p in grid.keys() {
        let mut np = *p;
        np[fold.0] = fold.1 - (fold.1 - p[fold.0]).abs();
        if let Some(v) = grid.get_value(*p) {
            g.set_value(np, v);
        }
    }
    g
}

fn part1(manual: &Parsed) -> Answer {
    let g = fold_grid(&manual.grid, manual.folds[0]);
    g.len() as Answer
}

fn part2(manual: &Parsed) -> Answer {
    let mut d = aoc::PrintGridDrawer::new(|c| if c == '#' { '\u{2588}' } else { ' ' });
    let mut g = manual.grid.clone();
    for fold in &manual.folds {
        g = fold_grid(&g, *fold);
    }
    d.draw(&g);
    0
}

fn parse(lines: &[String]) -> Parsed {
    let sections = aoc::split_by_empty_line(lines);
    let grid: HashMap<Point, char> = sections[0]
        .iter()
        .map(|x| (aoc::parse_point(x).unwrap(), '#'))
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
    aoc::run_main(parse, part1, part2);
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
        let mut g = manual.grid.clone();
        d.draw(&g);
        println!();
        for fold in &manual.folds {
            g = fold_grid(&g, *fold);
            d.draw(&g);
            println!();
        }
        assert_eq!(g.len(), 16);
    }
}
