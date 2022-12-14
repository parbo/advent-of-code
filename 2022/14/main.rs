use std::{collections::HashMap, iter::*};

use aoc::{Grid, GridDrawer, Point};

// #[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
// #[display("{thing}: {al}-{ah} or {bl}-{bh}")]
// struct Rule {
//     thing: String,
//     al: i64,
//     ah: i64,
//     bl: i64,
//     bh: i64,
// }

type Parsed = Vec<Vec<Point>>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    let maxy = data
        .iter()
        .flat_map(|x| x.iter().map(|p| p[1]))
        .max()
        .unwrap();
    let mut grid = HashMap::new();
    for wall in data {
        for p in wall.windows(2) {
            let a = p[0];
            let b = p[1];
            if a[0] == b[0] {
                let s = a[1].min(b[1]);
                let e = a[1].max(b[1]);
                for y in s..=e {
                    grid.insert([a[0], y], '#');
                }
            } else {
                let s = a[0].min(b[0]);
                let e = a[0].max(b[0]);
                for x in s..=e {
                    grid.insert([x, a[1]], '#');
                }
            }
        }
    }
    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    let mut grains = 0;
    let mut s = [500, 0];
    grid.insert(s, '+');
    'outer: loop {
        for d in [aoc::SOUTH, aoc::SOUTH_WEST, aoc::SOUTH_EAST] {
            let p = aoc::point_add(s, d);
            let c = grid.get_value(p).unwrap_or(' ');
            if c != '#' && c != 'o' {
                s = p;
                if s[1] > maxy {
                    break 'outer;
                } else {
                    continue 'outer;
                }
            }
        }
        grid.set_value(s, 'o');
        grains += 1;
        // gd.draw(&grid);
        // println!();
        s = [500, 0];
    }
    grains
}

fn part2(data: &Parsed) -> Answer {
    let maxy = data
        .iter()
        .flat_map(|x| x.iter().map(|p| p[1]))
        .max()
        .unwrap();
    let mut grid = HashMap::new();
    for wall in data {
        for p in wall.windows(2) {
            let a = p[0];
            let b = p[1];
            if a[0] == b[0] {
                let s = a[1].min(b[1]);
                let e = a[1].max(b[1]);
                for y in s..=e {
                    grid.insert([a[0], y], '#');
                }
            } else {
                let s = a[0].min(b[0]);
                let e = a[0].max(b[0]);
                for x in s..=e {
                    grid.insert([x, a[1]], '#');
                }
            }
        }
    }
    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    let mut grains = 0;
    let mut s = [500, 0];
    grid.insert(s, '+');
    'outer: loop {
        for d in [aoc::SOUTH, aoc::SOUTH_WEST, aoc::SOUTH_EAST] {
            let p = aoc::point_add(s, d);
            let mut c = grid.get_value(p).unwrap_or(' ');
            if p[1] == maxy + 2 {
                c = '#';
            }
            if c != '#' && c != 'o' {
                s = p;
                // if s[1] > maxy {
                //     break 'outer;
                // } else {
                continue 'outer;
                // }
            }
        }
        grid.set_value(s, 'o');
        grains += 1;
        // gd.draw(&grid);
        // println!();
        if s == [500, 0] {
            break;
        }
        s = [500, 0];
    }
    grains
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            aoc::split_str(x, "->")
                .iter()
                .map(|x| {
                    let p = aoc::split_ch(x, ',');
                    [p[0].parse().unwrap(), p[1].parse().unwrap()]
                })
                .collect()
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "498,4 -> 498,6 -> 496,6".into(),
            "503,4 -> 502,4 -> 502,9 -> 494,9".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 24);
    }
}
