#![allow(clippy::ptr_arg)]

use aoc::{Grid, GridDrawer};
use std::{collections::HashSet, iter::*};

type Parsed = Vec<Vec<i64>>;
type Answer = i64;

fn step(g: &mut Parsed) -> usize {
    let mut flash = vec![];
    // Increase by 1
    for p in g.points() {
        let v = g.get_value(p).unwrap() + 1;
        g.set_value(p, v);
        if v == 10 {
            flash.push(p);
        }
    }
    // Do the flash
    let mut flashed = HashSet::new();
    while !flash.is_empty() {
        for p in flash.split_off(0) {
            flashed.insert(p);
            for nb in aoc::neighbors_incl_diagonals(p) {
                if let Some(v) = g.get_value(nb) {
                    g.set_value(nb, v + 1);
                    if v + 1 == 10 {
                        flash.push(nb);
                    }
                }
            }
        }
    }
    let num = flashed.len();
    // reset to 0
    for p in flashed {
        g.set_value(p, 0);
    }
    num
}

fn part1(grid: &Parsed) -> Answer {
    let mut g = grid.clone();
    (0..100).map(|_| step(&mut g)).sum::<usize>() as Answer
}

fn part2(grid: &Parsed) -> Answer {
    let mut g = grid.clone();
    let total = g.len() * g[0].len();
    let mut gd = aoc::BitmapSpriteGridDrawer::new(
        (8, 8),
        |x| match x {
            0 => vec![[0xff, 0xff, 0xff]; 64],
            x => vec![[0, (x * 0x10 + 0x19) as u8, 0]; 64],
        },
        "ppm/day11",
    );
    let mut i = 0;
    let draw = cfg!(feature = "vis");
    loop {
        let num = step(&mut g);
        if draw {
            gd.draw(&g);
            gd.save_image();
        }
        i += 1;
        if num == total {
            break;
        }
    }
    i as Answer
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid_to(lines, |x| x.to_digit(10).unwrap().into())
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "5483143223".into(),
            "2745854711".into(),
            "5264556173".into(),
            "6141336146".into(),
            "6357385478".into(),
            "4167524645".into(),
            "2176841721".into(),
            "6882881134".into(),
            "4846848554".into(),
            "5283751526".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example()), false), 195);
    }
}
