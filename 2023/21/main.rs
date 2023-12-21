use std::iter::*;

use aoc::{Grid, GridDrawer};

type Parsed = Vec<Vec<char>>;

fn do_solve(
    data: &Parsed,
    steps: i64,
    p: aoc::Point,
    extend: bool,
    cache: &mut aoc::FxHashMap<(aoc::Point, i64), aoc::FxHashSet<aoc::Point>>,
) -> aoc::FxHashSet<aoc::Point> {
    let ([min_x, min_y], [max_x, max_y]) = data.extents();
    let h = max_y - min_y + 1;
    let w = max_x - min_x + 1;
    let mut reachable = aoc::FxHashSet::default();
    let mut todo: Vec<(aoc::Point, i64, aoc::FxHashSet<aoc::Point>)> =
        vec![(p, steps, aoc::FxHashSet::default())];
    while let Some((p, steps, r)) = todo.pop() {
        let pk = if extend {
            [p[0].rem_euclid(w), p[1].rem_euclid(h)]
        } else {
            p
        };
        if let Some(v) = cache.get(&(pk, steps)) {
            // Translate the positions back
            let mut vv = aoc::FxHashSet::default();
            let diff = aoc::point_sub(p, pk);
            for p in v {
                vv.insert(aoc::point_add(*p, diff));
            }
            reachable = reachable.union(&vv).cloned().collect();
        } else if steps == 0 {
            let mut vv = aoc::FxHashSet::default();
            let diff = aoc::point_sub(p, pk);
            for p in r {
                vv.insert(aoc::point_add(p, diff));
            }
            vv.insert(aoc::point_add(p, diff));
            reachable = reachable.union(&vv).cloned().collect();
            cache.insert((pk, steps), vv);
        } else {
            for pp in aoc::neighbors(p) {
                let ppp = if extend {
                    [pp[0].rem_euclid(w), pp[1].rem_euclid(h)]
                } else {
                    pp
                };
                let c = data.get_value(ppp);
                if c == Some('.') || c == Some('S') {
                    todo.push((pp, steps - 1, r.clone()));
                }
            }
        };
    }
    reachable
}

fn solve(data: &Parsed, steps: i64, extend: bool) -> i64 {
    let p = data
        .points()
        .find(|p| data.get_value(*p) == Some('S'))
        .unwrap();
    let mut cache = aoc::FxHashMap::default();
    let r = do_solve(data, steps, p, extend, &mut cache);
    r.len() as i64
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
