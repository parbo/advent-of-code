use aoc::GridDrawer;
use std::{cmp::Ordering, iter::*};

type ParsedItem = (aoc::Point, aoc::Point);
type Parsed = Vec<ParsedItem>;

fn quadrant(p: aoc::Point, w: i64, h: i64) -> Option<i64> {
    match (p[0].cmp(&(w / 2)), p[1].cmp(&(h / 2))) {
        (Ordering::Less, Ordering::Less) => Some(0),
        (Ordering::Less, Ordering::Greater) => Some(1),
        (Ordering::Greater, Ordering::Less) => Some(2),
        (Ordering::Greater, Ordering::Greater) => Some(3),
        _ => None,
    }
}

fn solve(data: &Parsed, w: i64, h: i64, n: i64) -> i64 {
    let quadrants = data
        .iter()
        .filter_map(|(p, v)| {
            quadrant(
                [
                    (p[0] + n * v[0]).rem_euclid(w),
                    (p[1] + n * v[1]).rem_euclid(h),
                ],
                w,
                h,
            )
        })
        .collect::<aoc::Counter<_>>();
    quadrants.values().product::<usize>() as i64
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 101, 103, 100)
}

fn part2(data: &Parsed) -> i64 {
    let w = 101;
    let h = 103;
    let n = w * h;
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
    let mut gdc = aoc::PrintGridDrawer::new(|x: i64| if x == 0 { ' ' } else { '*' });
    for i in 0..n {
        for (p, v) in &mut robots {
            p[0] = (p[0] + v[0]).rem_euclid(w);
            p[1] = (p[1] + v[1]).rem_euclid(h);
        }
        let mut g = vec![vec![0; w as usize]; h as usize];
        for (p, _v) in &robots {
            g[p[1] as usize][p[0] as usize] = 1;
        }
        // Detect a number of lines that have some streaks of robots
        if g.iter()
            .filter(|w| w.windows(7).any(|w| w.iter().all(|x| *x == 1)))
            .count()
            > 7
        {
            gd.draw(&g);
            gdc.draw(&g);
            return i + 1;
        }
    }
    -1
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
