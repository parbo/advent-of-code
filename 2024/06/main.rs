use aoc::Grid;
use rayon::prelude::*;
use std::iter::*;

type ParsedItem = Vec<char>;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let mut pos = data
        .points()
        .find(|x| data.get_value(*x) == Some('^'))
        .unwrap();
    let mut dir = aoc::NORTH;
    let mut visited = aoc::FxHashSet::default();
    visited.insert(pos);
    while let Some(v) = data.get_value(aoc::point_add(pos, dir)) {
        if v == '#' {
            dir = *aoc::DIRECTION_ROTATE_RIGHT.get(&dir).unwrap();
        } else {
            pos = aoc::point_add(pos, dir);
            visited.insert(pos);
        }
    }
    visited.len() as i64
}

fn part2(data: &Parsed) -> i64 {
    let mut pos = data
        .points()
        .find(|x| data.get_value(*x) == Some('^'))
        .unwrap();
    let mut dir = aoc::NORTH;
    let mut visited = aoc::FxHashSet::default();
    visited.insert(pos);
    while let Some(v) = data.get_value(aoc::point_add(pos, dir)) {
        if v == '#' {
            dir = *aoc::DIRECTION_ROTATE_RIGHT.get(&dir).unwrap();
        } else {
            pos = aoc::point_add(pos, dir);
            visited.insert(pos);
        }
    }
    visited
        .par_iter()
        .map(|pp| {
            let mut g = data.clone();
            g.set_value(*pp, '#');
            let mut pos = data
                .points()
                .find(|x| data.get_value(*x) == Some('^'))
                .unwrap();
            let mut dir = aoc::NORTH;
            let mut visited = aoc::FxHashSet::default();
            visited.insert((pos, dir));
            while let Some(v) = g.get_value(aoc::point_add(pos, dir)) {
                if v == '#' {
                    dir = *aoc::DIRECTION_ROTATE_RIGHT.get(&dir).unwrap();
                } else {
                    pos = aoc::point_add(pos, dir);
                    let inserted = visited.insert((pos, dir));
                    if !inserted {
                        return 1;
                    }
                }
            }
            0
        })
        .sum()
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid(lines)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
