use std::iter::*;

use aoc::point_add;
use aoc::point_neg;
use aoc::Grid;
use aoc::Point;
use aoc::DIRECTIONS;
use aoc::SOUTH;

type Parsed = Vec<Vec<char>>;

fn solve(data: &Parsed) -> (String, usize) {
    let start_x = data[0].iter().position(|c| *c == '|').unwrap() as i64;
    let mut dir = SOUTH;
    let mut pos: Point = [start_x, 0];
    let mut chars = vec![];
    let mut steps = 1;
    'outer: loop {
        let forward = point_add(pos, dir);
        let v = data.get_value(forward).unwrap_or(' ');
        if v != ' ' {
            pos = forward;
            if v.is_ascii_alphabetic() {
                chars.push(v);
            }
            steps += 1;
            continue;
        };
        // Switch direction
        let backtrack = point_neg(dir);
        for d in DIRECTIONS {
            if d == backtrack {
                continue;
            }
            let pos_d = point_add(pos, d);
            if data.get_value(pos_d).unwrap_or(' ') != ' ' {
                dir = d;
                continue 'outer;
            }
        }
        // If we reach here, we can't move further
        break;
    }
    (chars.iter().collect(), steps)
}

fn part1(data: &Parsed) -> String {
    solve(data).0
}

fn part2(data: &Parsed) -> usize {
    solve(data).1
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid(lines)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
