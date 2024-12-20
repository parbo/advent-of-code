use std::{cmp::Reverse, collections::BinaryHeap, iter::*};

use aoc::{Grid, GridDrawer};

type Parsed = Vec<Vec<char>>;

fn solve(data: &Parsed, threshold: i64) -> i64 {
    let s = data
        .points()
        .find(|p| data.get_value(*p) == Some('S'))
        .unwrap();
    let e = data
        .points()
        .find(|p| data.get_value(*p) == Some('E'))
        .unwrap();
    let (min, max) = data.extents();

    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    let uncheated_res =
        aoc::dijkstra_grid(data, |_p, c| *c != '#', |_p1, _c1, _p2, _c2| Some(1), s, e).unwrap();
    let mut num = 0;
    for p in data.points() {
        if p[0] == min[0] || p[0] == max[0] || p[1] == min[1] || p[1] == max[1] {
            continue;
        }
        if let Some('#') = data.get_value(p) {
            let mut g = data.clone();
            g.set_value(p, 'c');
            if let Some(res) =
                aoc::dijkstra_grid(&g, |_p, c| *c != '#', |_p1, _c1, _p2, _c2| Some(1), s, e)
            {
                if res.0 < uncheated_res.0 {
                    // dbg!(uncheated_res.0 - res.0);
                    // for pp in res.1 {
                    //     if pp != p {
                    //         g.set_value(pp, '*');
                    //     }
                    // }
                    // gd.draw(&g);
                    if uncheated_res.0 - res.0 >= 100 {
                        num += 1;
                    }
                }
            }
        }
    }
    num
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 100)
}

fn part2(data: &Parsed) -> i64 {
    let s = data
        .points()
        .find(|p| data.get_value(*p) == Some('S'))
        .unwrap();
    let e = data
        .points()
        .find(|p| data.get_value(*p) == Some('E'))
        .unwrap();
    let uncheated_res =
        aoc::dijkstra_grid(data, |_p, c| *c != '#', |_p1, _c1, _p2, _c2| Some(1), s, e)
            .unwrap()
            .0;
    let mut frontier = BinaryHeap::new();
    let mut visited: aoc::FxHashSet<(aoc::Point, i64)> = aoc::FxHashSet::default();
    let mut came_from = aoc::FxHashMap::default();
    frontier.push(Reverse((0, s, 20)));
    let mut num = 0;
    while let Some(Reverse((score, current, rem))) = frontier.pop() {
        if visited.contains(&(current, rem)) {
            continue;
        }
        if current == e && uncheated_res - score >= 100 {
            num += 1;
            continue;
        }
        for nb in aoc::neighbors(current) {
            if let Some(value) = data.get_value(nb) {
                let r = if value == '#' { rem - 1 } else { rem };
                if r >= 0 {
                    if visited.contains(&(nb, r)) {
                        continue;
                    }
                    let new_score = score + 1;
                    came_from.insert(nb, current);
                    frontier.push(Reverse((new_score, nb, r)));
                }
            }
        }
        visited.insert((current, rem));
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
    use super::*;

    fn example() -> Vec<String> {
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&parse(&example()), 0), 44);
    }
}
