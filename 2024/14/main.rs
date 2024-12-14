use aoc::GridDrawer;
use std::{collections::BTreeMap, iter::*};

type ParsedItem = (aoc::Point, aoc::Point);
type Parsed = Vec<ParsedItem>;

fn solve(data: &Parsed, w: i64, h: i64, n: i64) -> i64 {
    let mut robots = data.clone();
    let mut gd = aoc::BitmapGridDrawer::new(
        |x: i64| {
            if x == 0 {
                [0, 0, 0]
            } else {
                [0, 0xff, 0]
            }
        },
        "vis/14/day14",
    );
    gd.set_bg([0, 0, 0]);
    for i in 0..n {
        for (p, v) in &mut robots {
            p[0] = (p[0] + v[0]).rem_euclid(w);
            p[1] = (p[1] + v[1]).rem_euclid(h);
        }
        let mut g = BTreeMap::<aoc::Point, i64>::new();
        g.insert([0, 0], 0);
        g.insert([0, h - 1], 0);
        g.insert([w - 1, h - 1], 0);
        g.insert([w - 1, 0], 0);
        for (p, _v) in &robots {
            *g.entry(*p).or_default() += 1;
        }
        gd.draw(&g);
    }
    let mut q = [0, 0, 0, 0];
    for (p, _v) in &robots {
        if p[0] < w / 2 {
            if p[1] < h / 2 {
                q[0] += 1;
            } else if p[1] > h / 2 {
                q[1] += 1;
            }
        } else if p[0] > w / 2 {
            if p[1] < h / 2 {
                q[2] += 1;
            } else if p[1] > h / 2 {
                q[3] += 1;
            }
        }
    }
    q[0] * q[1] * q[2] * q[3]
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 101, 103, 100)
}

fn part2(data: &Parsed) -> i64 {
    solve(data, 101, 103, 10000)
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let parts = aoc::split_w(x);
            let p = aoc::split_ch(&parts[0][2..], ',');
            let v = aoc::split_ch(&parts[1][2..], ',');
            (
                [p[0].parse().unwrap(), p[1].parse().unwrap()],
                [v[0].parse().unwrap(), v[1].parse().unwrap()],
            )
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
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    fn example2() -> Vec<String> {
        vec!["p=2,4 v=2,-3".into()]
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&parse(&example()), 11, 7, 100), 12);
    }
}
