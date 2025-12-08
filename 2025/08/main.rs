use std::{cmp::Reverse, collections::BinaryHeap, iter::*};

use aoc::{FxHashMap, FxHashSet};

#[derive(
    parse_display::Display, parse_display::FromStr, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[display("{x},{y},{z}")]
struct Junction {
    x: i64,
    y: i64,
    z: i64,
}

type ParsedItem = Junction;
type Parsed = Vec<ParsedItem>;

fn dist(n: Junction, goal: Junction) -> i64 {
    (goal.x - n.x).pow(2) + (goal.y - n.y).pow(2) + (goal.z - n.z).pow(2)
}

fn distances(data: &Parsed) -> BinaryHeap<Reverse<(i64, usize, usize)>> {
    let mut heap = BinaryHeap::new();
    for i in 0..data.len() {
        for j in 0..data.len() {
            if i == j {
                continue;
            }
            let d = dist(data[i], data[j]);
            heap.push(Reverse((d, i, j)))
        }
    }
    heap
}

fn solve(data: &Parsed, num: usize) -> i64 {
    let mut connections: FxHashMap<usize, FxHashSet<usize>> = FxHashMap::default();
    let mut dd = distances(data);
    let mut n = 0;
    while n < num {
        if let Some(Reverse((_d, i, j))) = dd.pop() {
            if connections.entry(i).or_default().contains(&j) {
                continue;
            }
            n += 1;
            connections.entry(i).or_default().insert(j);
            connections.entry(j).or_default().insert(i);
        }
    }
    let mut seen: FxHashSet<usize> = FxHashSet::default();
    let mut chains = vec![];
    for i in 0..data.len() {
        let mut todo = vec![i];
        let mut chain = vec![i];
        while let Some(x) = todo.pop() {
            let e = connections.entry(x).or_default();
            seen.insert(x);
            for k in e.iter() {
                if seen.contains(k) {
                    continue;
                }
                todo.push(*k);
                chain.push(*k);
                seen.insert(*k);
            }
        }
        chains.push(chain);
    }
    chains.sort_by_key(|a| Reverse(a.len()));
    chains.iter().take(3).map(|x| x.len()).product::<usize>() as i64
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 1000)
}

fn part2(data: &Parsed) -> i64 {
    let mut connections: FxHashMap<usize, FxHashSet<usize>> = FxHashMap::default();
    let mut dd = distances(data);
    loop {
        if let Some(Reverse((_d, i, j))) = dd.pop() {
            if connections.entry(i).or_default().contains(&j) {
                continue;
            }
            connections.entry(i).or_default().insert(j);
            connections.entry(j).or_default().insert(i);
            let mut seen: FxHashSet<usize> = FxHashSet::default();
            let mut todo = vec![i];
            while let Some(x) = todo.pop() {
                let e = connections.entry(x).or_default();
                seen.insert(x);
                for k in e.iter() {
                    if seen.contains(k) {
                        continue;
                    }
                    todo.push(*k);
                    seen.insert(*k);
                }
            }
            if seen.len() == data.len() {
                return data[i].x * data[j].x;
            }
        }
    }
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
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
        assert_eq!(solve(&parse(&example()), 10), 40);
    }
}
