#[cfg(feature = "vis")]
use aoc::GridDrawer;

use std::collections::HashMap;

#[derive(Debug)]
struct Step {
    dir: char,
    num: i64,
}

type ParsedItem = Step;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let mut pos = [0, 0];
    let mut dir = aoc::NORTH;
    for d in data {
        match d.dir {
            'L' => dir = *aoc::DIRECTION_ROTATE_LEFT.get(&dir).unwrap(),
            'R' => dir = *aoc::DIRECTION_ROTATE_RIGHT.get(&dir).unwrap(),
            _ => panic!(),
        }
        pos = aoc::point_add(pos, aoc::point_mul(dir, d.num));
    }
    aoc::manhattan(pos, [0, 0])
}

fn part2(data: &Parsed) -> i64 {
    let mut pos = [0, 0];
    let mut dir = aoc::NORTH;
    let mut seen = HashMap::new();
    #[cfg(feature = "vis")]
    let mut gd = aoc::BitmapGridDrawer::new(
        |x| match x {
            '*' => [0, 0, 0],
            _ => [255, 255, 255],
        },
        "day01",
    );
    seen.insert(pos, '*');
    #[cfg(feature = "vis")]
    {
        seen.insert([-100, -100], '.');
        seen.insert([-100, 200], '.');
        seen.insert([100, -100], '.');
        seen.insert([100, 200], '.');
        gd.draw(&seen);
        println!();
    }
    'outer: for d in data {
        match d.dir {
            'L' => dir = *aoc::DIRECTION_ROTATE_LEFT.get(&dir).unwrap(),
            'R' => dir = *aoc::DIRECTION_ROTATE_RIGHT.get(&dir).unwrap(),
            _ => panic!(),
        }
        for _ in 0..d.num {
            pos = aoc::point_add(pos, dir);
            #[cfg(feature = "vis")]
            gd.draw(&seen);
            if !seen.insert(pos, '*').is_none() {
                break 'outer;
            }
        }
    }
    aoc::manhattan(pos, [0, 0])
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_ch(&lines[0], ',')
        .into_iter()
        .map(|c| Step {
            dir: c.chars().next().unwrap(),
            num: c[1..].parse::<i64>().unwrap(),
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
        vec!["R8, R4, R4, R8".into()]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part2(&parse(&vec!["R2, L3".into()])), 5);
        assert_eq!(part2(&parse(&vec!["R2, R2, R2".into()])), 2);
        assert_eq!(part2(&parse(&vec!["R5, L5, R5, R3".into()])), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 4);
    }
}
