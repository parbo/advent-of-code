use std::{collections::VecDeque, iter::*};

use aoc::{Grid, GridDrawer};

type Parsed = Vec<Vec<char>>;

fn solve(data: &Parsed, steps: i64, extend: bool) -> aoc::FxHashSet<aoc::Point> {
    let p = data
        .points()
        .find(|p| data.get_value(*p) == Some('S'))
        .unwrap();
    let ([min_x, min_y], [max_x, max_y]) = data.extents();
    let h = max_y - min_y + 1;
    let w = max_x - min_x + 1;
    // dbg!(w, h);
    let mut reachable: aoc::FxHashSet<aoc::Point> = aoc::FxHashSet::default();
    let mut last_rr: aoc::FxHashSet<aoc::Point> = aoc::FxHashSet::default();
    let mut last_r_0: aoc::FxHashSet<aoc::Point> = aoc::FxHashSet::default();
    let mut last_r_1: aoc::FxHashSet<aoc::Point> = aoc::FxHashSet::default();
    let mut todo = VecDeque::from([(p, 0)]);
    let mut seen = aoc::FxHashSet::default();
    let mut last = 0;
    let mut last_r = 0;
    let mut last_diff = 0;
    // let mut diffs = vec![];
    // let mut cache = aoc::FxHashMap::default();
    while let Some((p, s)) = todo.pop_back() {
        if s != last {
            if s % 100 == 0 {
                let a = (2 * s) * (2 * s) / 2;
                dbg!(
                    s,
                    reachable.len(),
                    a,
                    a / 2 - reachable.len(),
                    reachable.len() - last_r
                );
            }
            let mut rr: aoc::FxHashSet<aoc::Point> = aoc::FxHashSet::default();
            for &p in &reachable {
                let pp = if extend {
                    [p[0].rem_euclid(w), p[1].rem_euclid(h)]
                } else {
                    p
                };
                rr.insert(pp);
            }
            if rr != last_rr {
                if rr.len() > 15000 {
                    dbg!(&rr);
                }
                last_rr = rr;
            }
            let diff: aoc::FxHashSet<_> = reachable.difference(&last_r_0).cloned().collect();
            // let diff1: aoc::FxHashSet<_> = reachable.difference(&last_r_1).cloned().collect();
            // let diff2: aoc::FxHashSet<_> = last_r_0.difference(&last_r_1).cloned().collect();
            // dbg!(diff.len(), diff.len() as i64 - last_diff);
            // diffs.push(diff.len() as i64 - last_diff);
            // // Look for repeats
            // for offs in 0..diffs.len() {
            //     for len in 1..diffs.len() {
            //         if offs + len + offs + len < diffs.len()
            //             && diffs[offs..(offs + len)]
            //                 == diffs[(offs + len)..(offs + len + offs + len)]
            //         {
            //             println!("repeat found: {} {}", offs, len);
            //         }
            //     }
            // }
            last_diff = diff.len() as i64;
            last = s;
            last_r = reachable.len();

            if s == steps as usize {
                break;
            }

            std::mem::swap(&mut last_r_0, &mut last_r_1);
            std::mem::swap(&mut last_r_1, &mut reachable);
            reachable.clear();
            seen.clear();
        }
        // reachable.insert(p);
        // if !seen.insert((p, s)) {
        //     continue;
        // }
        for pp in aoc::neighbors(p) {
            let ppp = if extend {
                [pp[0].rem_euclid(w), pp[1].rem_euclid(h)]
            } else {
                pp
            };
            // dbg!(pp, ppp);
            let c = data.get_value(ppp);
            if c == Some('.') || c == Some('S') {
                reachable.insert(pp);
                if seen.insert((pp, s + 1)) {
                    todo.push_front((pp, s + 1));
                }
            }
        }
    }
    reachable
}

fn make_g(data: &Parsed, r: &aoc::FxHashSet<aoc::Point>) -> aoc::FxHashMap<aoc::Point, char> {
    let mut g: aoc::FxHashMap<_, _> = r.iter().map(|x| (*x, 'O')).collect();
    let ([min_x, min_y], [max_x, max_y]) = data.extents();
    let h = max_y - min_y + 1;
    let w = max_x - min_x + 1;
    let ([min_xg, min_yg], [max_xg, max_yg]) = g.extents();
    let min_x = min_x.min(min_xg);
    let min_y = min_y.min(min_yg);
    let max_x = max_x.max(max_xg);
    let max_y = max_y.max(max_yg);
    for y in min_y..=max_y {
        for x in min_x..max_x {
            g.entry([x, y]).or_insert_with(|| {
                data.get_value([x.rem_euclid(w), y.rem_euclid(h)])
                    .unwrap_or('.')
            });
        }
    }
    g
}

fn part1(data: &Parsed) -> i64 {
    let r = solve(data, 64, false);
    r.len() as i64
}

fn part2(data: &Parsed) -> i64 {
    let r = solve(data, 26501365, true);
    r.len() as i64
    // //    solve(data, 26501365, true)
    // let mut gd = aoc::BitmapGridDrawer::new(
    //     |c| match c {
    //         '#' => [0, 255, 0],
    //         'O' => [255, 255, 255],
    //         _ => [0, 0, 0],
    //     },
    //     "vis/21/part2",
    // );
    // gd.set_bg([0, 0, 0]);
    // let mut last = 0;
    // for s in 1..300 {
    //     let r = solve1(data, s, true);
    //     let g = make_g(data, &r);
    //     gd.draw(&g);
    //     println!("steps: {}, num: {}, inc: {}", s, r.len(), r.len() - last);
    //     last = r.len();
    // }
    // 0
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
        assert_eq!(solve(&parse(&example()), 6, true).len(), 16);
        assert_eq!(solve(&parse(&example()), 10, true).len(), 50);
        assert_eq!(solve(&parse(&example()), 50, true).len(), 1594);
        assert_eq!(solve(&parse(&example()), 100, true).len(), 6536);
        assert_eq!(solve(&parse(&example()), 500, true).len(), 167004);
        // assert_eq!(solve(&parse(&example()), 1000, true).len(), 668697);
        // assert_eq!(solve(&parse(&example()), 5000, true).len(), 16733044);
    }
}
