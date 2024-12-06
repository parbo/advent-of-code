use std::iter::*;

use aoc::Grid;

// #[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
// #[display("{thing}: {al}-{ah} or {bl}-{bh}")]
// struct Rule {
//     thing: String,
//     al: i64,
//     ah: i64,
//     bl: i64,
//     bh: i64,
// }

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
    let mut num = 0;
    for pp in data.points() {
        let mut g = data.clone();
        g.set_value(pp, '#');
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
                    num += 1;
                    break;
                }
            }
        }
    }
    num
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid(lines)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn example() -> Vec<String> {
    //     let s = include_str!("example.txt");
    //     s.lines().map(|x| x.to_string()).collect()
    // }

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
