use std::iter::*;

use aoc::{Grid, GridDrawer};

type Parsed = Vec<Vec<char>>;

fn do_solve(
    data: &Parsed,
    orig_steps: i64,
    steps: i64,
    orig_p: aoc::Point,
    p: aoc::Point,
    extend: bool,
    cache: &mut aoc::FxHashMap<(aoc::Point, i64), (aoc::Point, aoc::FxHashSet<aoc::Point>)>,
) -> aoc::FxHashSet<aoc::Point> {
    let ([min_x, min_y], [max_x, max_y]) = data.extents();
    let h = max_y - min_y + 1;
    let w = max_x - min_x + 1;
    let pk = if extend {
        [p[0].rem_euclid(w), p[1].rem_euclid(h)]
    } else {
        p
    };
    if let Some((op, v)) = cache.get(&(pk, steps)) {
        // Translate the positions back
        let mut vv = aoc::FxHashSet::default();
        let diff = aoc::point_sub(p, *op);
        for vp in v {
            // assert!(aoc::manhattan(*vp, orig_p) <= orig_steps);
            let pp = aoc::point_add(*vp, diff);
            // if aoc::manhattan(pp, orig_p) > orig_steps {
            //     dbg!(diff, p, pk, steps, vp, pp, orig_p, orig_steps);
            // }
            // assert!(aoc::manhattan(pp, orig_p) <= orig_steps);
            vv.insert(pp);
        }
        vv
        // v.clone()
    } else if steps == 0 {
        let mut r = aoc::FxHashSet::default();
        // if aoc::manhattan(p, orig_p) > orig_steps {
        //     dbg!(p, orig_steps);
        // }
        // assert!(aoc::manhattan(p, orig_p) <= orig_steps);
        r.insert(p);
        r
    } else {
        let mut v = aoc::FxHashSet::default();
        for pp in aoc::neighbors(p) {
            let ppp = if extend {
                [pp[0].rem_euclid(w), pp[1].rem_euclid(h)]
            } else {
                pp
            };
            let c = data.get_value(ppp);
            if c == Some('.') || c == Some('S') {
                v = v
                    .union(&do_solve(
                        data,
                        orig_steps,
                        steps - 1,
                        orig_p,
                        pp,
                        extend,
                        cache,
                    ))
                    .cloned()
                    .collect();
            }
        }
        // if pk == [4, 0] {
        //     dbg!(p, pk, &v);
        // }
        let x = cache.insert((pk, steps), (p, v.clone()));
        if cache.len() % 10000 == 0 {
            dbg!(cache.len());
        }
        assert!(x.is_none());
        v
    }
}

fn solve(data: &Parsed, steps: i64, extend: bool) -> i64 {
    let p = data
        .points()
        .find(|p| data.get_value(*p) == Some('S'))
        .unwrap();
    let mut cache = aoc::FxHashMap::default();
    let r = do_solve(data, steps, steps, p, p, extend, &mut cache);
    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    let g: aoc::FxHashMap<_, _> = r.iter().map(|x| (*x, 'O')).collect();
    gd.draw(&g);
    println!();
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

    // #[test]
    // fn test_part1() {
    //     assert_eq!(solve(&parse(&example()), 6, false), 16);
    // }

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
