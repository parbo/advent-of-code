use std::iter::*;

use aoc::{Grid, GridDrawer};

type Parsed = Vec<Vec<char>>;

fn solve(data: &Parsed, steps: i64, extend: bool) -> i64 {
    let p = data
        .points()
        .find(|p| data.get_value(*p) == Some('S'))
        .unwrap();
    let ([min_x, min_y], [max_x, max_y]) = data.extents();
    let h = max_y - min_y + 1;
    let w = max_x - min_x + 1;
    dbg!(w, h);
    let mut reachable = aoc::FxHashSet::default();
    let mut todo = vec![(p, 0)];
    let mut seen = aoc::FxHashSet::default();
    while let Some((p, s)) = todo.pop() {
        if s == steps {
            reachable.insert(p);
            continue;
        }
        // let ppp = if extend {
        //     [p[0].rem_euclid(w), p[1].rem_euclid(h)]
        // } else {
        //     p
        // };
        if !seen.insert((p, s)) {
            continue;
        }
        for pp in aoc::neighbors(p) {
            let ppp = if extend {
                [pp[0].rem_euclid(w), pp[1].rem_euclid(h)]
            } else {
                pp
            };
            dbg!(pp, ppp);
            let c = data.get_value(ppp);
            if c == Some('.') || c == Some('S') {
                todo.push((pp, s + 1));
            }
        }
    }
    reachable.len() as i64
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 64, false)
}

fn part2(data: &Parsed) -> i64 {
    solve(data, 26501365, true)
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid(lines)
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

    #[test]
    fn test_part1() {
        assert_eq!(solve(&parse(&example()), 6, false), 16);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&parse(&example()), 6, true), 16);
        assert_eq!(solve(&parse(&example()), 10, true), 50);
        assert_eq!(solve(&parse(&example()), 50, true), 1594);
        assert_eq!(solve(&parse(&example()), 100, true), 6536);
        assert_eq!(solve(&parse(&example()), 500, true), 167004);
        assert_eq!(solve(&parse(&example()), 1000, true), 668697);
        assert_eq!(solve(&parse(&example()), 5000, true), 16733044);
    }
}
