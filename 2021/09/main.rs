#![allow(clippy::ptr_arg)]

use aoc::Grid;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::*;

#[cfg(feature = "vis")]
use aoc::GridDrawer;

type Parsed = Vec<Vec<i64>>;
type Answer = i64;

fn get_low_points(map: &Parsed) -> Vec<aoc::Point> {
    let mut low_points = vec![];
    'outer: for p in map.points() {
        let ph = map.get_value(p).unwrap();
        for nb in aoc::neighbors(p) {
            if let Some(h) = map.get_value(nb) {
                if ph >= h {
                    continue 'outer;
                }
            }
        }
        low_points.push(p);
    }
    low_points
}

fn part1(map: &Parsed) -> Answer {
    let low_points = get_low_points(map);
    low_points
        .iter()
        .map(|p| map.get_value(*p).unwrap())
        .map(|h| h + 1)
        .sum::<i64>()
}

fn part2(m: &Parsed) -> Answer {
    let mut map = m.clone();
    let mut basins = vec![];
    let ([min_x, min_y], [max_x, max_y]) = map.extents();
    #[cfg(feature = "vis")]
    let mut gd = aoc::BitmapSpriteGridDrawer::new(
        (3, 3),
        |x| match x {
            -1i64 => vec![[0x79, 0xa2, 0xd8]; 9],
            9 => vec![[0xff, 0xff, 0x66]; 9],
            x => vec![[0, (x * 0x10 + 0x19) as u8, 0]; 9],
        },
        "ppm/day09",
    );
    let low_points = get_low_points(&map);
    for pos in low_points {
        let value = map.get_value(pos).unwrap();
        if value == 9 && value == -1 {
            continue;
        }
        let mut num = 0;
        let mut todo = BinaryHeap::new();
        #[cfg(feature = "vis")]
        let mut last_gen = -1;
        todo.push(Reverse((0, pos)));
        while let Some(Reverse((gen, p))) = todo.pop() {
            if let Some(curr) = map.get_value(p) {
                if curr != 9 && curr != -1 {
                    num += 1;
                    map.set_value(p, -1);
                    if p[0] > min_x {
                        todo.push(Reverse((gen + 1, [p[0] - 1, p[1]])));
                    }
                    if p[0] < max_x {
                        todo.push(Reverse((gen + 1, [p[0] + 1, p[1]])));
                    }
                    if p[1] > min_y {
                        todo.push(Reverse((gen + 1, [p[0], p[1] - 1])));
                    }
                    if p[1] < max_y {
                        todo.push(Reverse((gen + 1, [p[0], p[1] + 1])));
                    }
                    #[cfg(feature = "vis")]
                    {
                        if last_gen != gen {
                            gd.draw(&map);
                            gd.save_image();
                        }
                        last_gen = gen;
                    }
                }
            }
        }
        basins.push(num);
    }
    #[cfg(feature = "vis")]
    {
        let mut d = aoc::PrintGridDrawer::new(|i: i64| {
            if i == -1 {
                '*'
            } else {
                char::from_digit(i as u32, 10).unwrap()
            }
        });
        d.draw(&map);
    }
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
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

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse(&vec![
                "2199943210".into(),
                "3987894921".into(),
                "9856789892".into(),
                "8767896789".into(),
                "9899965678".into()
            ])),
            15
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                &parse(&vec![
                    "2199943210".into(),
                    "3987894921".into(),
                    "9856789892".into(),
                    "8767896789".into(),
                    "9899965678".into()
                ]),
                false
            ),
            1134
        );
    }
}
