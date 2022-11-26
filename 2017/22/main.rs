use std::collections::HashMap;

use aoc::{point_add, Grid, Point, DIRECTION_ROTATE_LEFT, DIRECTION_ROTATE_RIGHT, NORTH, point_neg};

type Parsed = (usize, HashMap<Point, char>);
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    let middle = data.0 as i64 / 2;
    let mut pos: Point = [middle, middle];
    let mut dir = NORTH;
    let mut grid = data.1.clone();
    let mut infected = 0;
    for _ in 0..10000 {
        if grid.get_value(pos) == Some('#') {
            dir = *DIRECTION_ROTATE_RIGHT.get(&dir).unwrap();
            grid.remove(&pos);
        } else {
            dir = *DIRECTION_ROTATE_LEFT.get(&dir).unwrap();
            infected += 1;
            grid.set_value(pos, '#');
        };
        pos = point_add(pos, dir);
    }
    infected
}

fn part2(data: &Parsed) -> Answer {
    let middle = data.0 as i64 / 2;
    let mut pos: Point = [middle, middle];
    let mut dir = NORTH;
    let mut grid = data.1.clone();
    let mut infected = 0;
    for _ in 0..10000000 {
        match grid.get_value(pos) {
            Some('#') => {
                dir = *DIRECTION_ROTATE_RIGHT.get(&dir).unwrap();
                grid.set_value(pos, 'f');
            },
            Some('f') => {
                dir = point_neg(dir);
                grid.remove(&pos);
            }
            Some('w') => {
                grid.set_value(pos, '#');
                infected += 1;
            },
            _ => {
                dir = *DIRECTION_ROTATE_LEFT.get(&dir).unwrap();
                grid.set_value(pos, 'w');
            }
        };
        pos = point_add(pos, dir);
    }
    infected
}

fn parse(lines: &[String]) -> Parsed {
    (
        lines.len(),
        aoc::parse_grid_to_sparse(lines, |c| if c == '#' { Some(c) } else { None }),
    )
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn example() -> Vec<String> {
    // 	   vec![
    //         "0".into()
    //     ]
    // }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&parse(&example())), 0);
    // }
}
