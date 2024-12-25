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
                    Some('.') | Some('E') | Some('S') => {
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

fn dist_map(data: &Parsed, goal: aoc::Point) -> aoc::FxHashMap<aoc::Point, i64> {
    let mut dists: aoc::FxHashMap<aoc::Point, i64> = aoc::FxHashMap::default();
    // Flood-fill from goal
    let mut todo = vec![(goal, 0)];
    let mut seen: aoc::FxHashSet<aoc::Point> = aoc::FxHashSet::default();
    while let Some((pos, d)) = todo.pop() {
        let e = dists.entry(pos).or_insert(i64::MAX);
        if d < *e {
            *e = d;
        } else {
            // No point in expanding
            continue;
        }
        for (nb, _r) in get_neighbors(data, pos, 0) {
            if seen.insert(nb) {
                todo.push((nb, d + 1));
            }
        }
    }
    dists
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
    let dm = dist_map(data, e);
    // dbg!(&dm);
    let pl = dm.get(dbg!(&s)).unwrap() - 1;
    dbg!(pl);
    let mut frontier = BinaryHeap::new();
    let mut visited = aoc::FxHashSet::default();
    // let mut came_from = aoc::FxHashMap::default();
    frontier.push(Reverse((0, s, cheats, 0, vec![])));
    let mut num = 0;
    let mut gd = aoc::make_bitmap_text_grid_drawer(
        |c| match c {
            '*' => (c, [0x20, 0xff, 0x20]),
            '+' => (c, [0xff, 0x20, 0x20]),
            _ => (c, [0xff, 0xff, 0xff]),
        },
        "vis/20/day20",
    );
    while let Some(Reverse((score, current, rem, d, path))) = frontier.pop() {
        if visited.contains(&(current, rem, d /*path.clone()*/)) {
            continue;
        }
        // if (pl - d) < threshold {
        //     println!("thresholed");
        //     continue;
        // }
        // if uncheated_res - score < threshold {
        //     continue;
        // }
        // let mut g = data.clone();
        // for p in &path {
        //     g.set_value(*p, '*');
        // }
        // gd.draw(&g);
        if current == e {
            // let mut path = vec![e];
            // let mut curr = (e, rem);
            // while curr.0 != s {
            //     curr = came_from[&curr];
            //     path.push(curr.0)
            // }
            // dbg!(path.len(), rem);
            // dbg!(path.len());
            if (pl - d) >= threshold {
                // println!("found");
                dbg!(path.len(), pl, d);
                num += 1;
                let mut g = data.clone();
                let mut last = path[0];
                for p in &path {
                    if aoc::manhattan(*p, last) > 1 {
                        g.line(last, *p, '+');
                    }
                    g.set_value(last, '*');
                    g.set_value(*p, '*');
                    last = *p;
                }
                gd.draw(&g);
            }
            continue;
        }
        // if let Some(Some(x)) = paths.get(&(current, rem)) {
        //     if uncheated_res - (score + *x) < threshold {
        //         continue;
        //     }
        // }
        let ppl = dm.get(&current).unwrap();
        // dbg!(current, rem);
        // let nbs = get_neighbors(data, current, rem);
        // for (nb, r) in nbs
        //     .into_iter()
        //     .filter(|(k, _v)| current != *k && *dm.get(k).unwrap() < *ppl)
        for (nb, r) in dm
            .iter()
            .filter(|(k, v)| current != **k && **v < *ppl)
            .map(|(k, v)| (*k, rem - ((*ppl - *v) - aoc::manhattan(current, *k))))
            .filter(|(k, r)| *r >= 0)
        {
            // dbg!(nb, r);
            if path.contains(&nb) {
                unreachable!();
                continue;
            }
            assert!(r >= 0);
            let npl = dm.get(&nb).unwrap();
            if *npl > *ppl {
                unreachable!();
                continue;
            }
            let mut newp = path.clone();
            newp.push(nb);
            let new_score = newp.len() as i64 + *npl;
            frontier.push(Reverse((new_score, nb, r, d + 1, newp)));
        }
        visited.insert((current, rem, d /*path.clone()*/));
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
