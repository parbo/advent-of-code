use std::iter::*;

use aoc::{Grid, GridDrawer};

type Parsed = Vec<Vec<char>>;

fn solve(data: &Parsed, steps: i64) -> i64 {
    let p = data
        .points()
        .find(|p| data.get_value(*p) == Some('S'))
        .unwrap();
    let ([min_x, min_y], [max_x, max_y]) = data.extents();
    let h = max_y - min_y + 1;
    let w = max_x - min_x + 1;
    // dbg!(w, h);
    let mut reachable: aoc::FxHashMap<aoc::Point, i64> = aoc::FxHashMap::default();
    reachable.insert(p, 1);
    // let mut gd = aoc::PrintGridDrawer::new(|x: i64| {
    //     if let Some(c) = char::from_digit(x as u32, 16) {
    //         c
    //     } else {
    //         '!'
    //     }
    // });
    for s in 0..steps {
        let mut seen = aoc::FxHashSet::default();
        let mut new_reachable = aoc::FxHashMap::default();
        for (p, num) in &reachable {
            for pp in aoc::neighbors(*p) {
                let ppp = [pp[0].rem_euclid(w), pp[1].rem_euclid(h)];
                // let same = ppp == pp;
                if seen.insert(pp) {
                    let c = data.get_value(ppp);
                    if c == Some('.') || c == Some('S') {
                        // let e = new_reachable.entry(ppp).or_default();
                        let e = new_reachable.entry(pp).or_default();
                        *e += num;
                    }
                }
            }
        }
        reachable = new_reachable;
        println!("{}, {}", s, reachable.values().sum::<i64>());
        // println!("{}, {:?}", reachable.values().sum::<i64>(), reachable);
        // gd.draw(&reachable);
    }
    reachable.values().sum::<i64>()
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 64)
}

fn part2(data: &Parsed) -> i64 {
    solve(data, 26501365)
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
