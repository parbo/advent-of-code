use std::{cmp::Reverse, collections::BinaryHeap, iter::*};

use aoc::{Grid, GridDrawer, PrintGridDrawer};

type Parsed = Vec<Vec<char>>;

fn get_neighbors(data: &Parsed, p: aoc::Point, d: i64) -> Vec<(aoc::Point, i64)> {
    let mut res = vec![];
    let mut todo = vec![(p, d)];
    let mut seen = aoc::FxHashSet::default();
    while let Some((p, d)) = todo.pop() {
        for nb in aoc::neighbors(p) {
            if seen.insert(nb) {
                match data.get_value(nb) {
                    Some('.') | Some('E') => {
                        res.push((nb, d));
                    }
                    Some('#') => {
                        if d > 0 {
                            todo.push((nb, d - 1));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    // dbg!(&res);
    res
}

fn solve(data: &Parsed, threshold: i64, cheats: i64) -> i64 {
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
    let mut visited = aoc::FxHashSet::default();
    // let mut came_from = aoc::FxHashMap::default();
    frontier.push(Reverse((0, s, cheats, vec![])));
    let mut num = 0;
    while let Some(Reverse((score, current, rem, path))) = frontier.pop() {
        if visited.contains(&(current, rem, path.clone())) {
            continue;
        }
        if uncheated_res - score < threshold {
            continue;
        }
        if num >= 31 {
            println!(
                "{:?}, {} ,{}, {}, {:?}",
                current,
                rem,
                path.len(),
                frontier.len(),
                path,
            );
            let mut gd = PrintGridDrawer::new(|c| c);
            let mut g = data.clone();
            for p in &path {
                g.set_value(*p, '*');
            }
            gd.draw(&g);
            println!();
        }
        if current == e {
            // let mut path = vec![e];
            // let mut curr = (e, rem);
            // while curr.0 != s {
            //     curr = came_from[&curr];
            //     path.push(curr.0)
            // }
            let mut gd = PrintGridDrawer::new(|c| c);
            let mut g = data.clone();
            for p in &path {
                g.set_value(*p, '*');
            }
            gd.draw(&g);
            num += 1;
            continue;
        }
        for (nb, r) in get_neighbors(data, current, rem) {
            if path.contains(&nb) {
                continue;
            }
            // if visited.contains(&(nb, r, )) {
            //     continue;
            // }
            assert!(r >= 0);
            let new_score = score + aoc::manhattan(current, nb);
            // came_from.insert((nb, r), (current, rem));
            let mut newp = path.clone();
            newp.push(nb);
            frontier.push(Reverse((new_score, nb, r, newp)));
        }
        visited.insert((current, rem, path.clone()));
    }
    num
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 100, 1)
}

fn part2(data: &Parsed) -> i64 {
    solve(data, 100, 19)
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
        assert_eq!(solve(&parse(&example()), 1, 1), 44);
    }
}
