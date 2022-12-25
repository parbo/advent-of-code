#![allow(clippy::ptr_arg)]

use aoc::FxHashMap;
use aoc::{Grid, Itertools};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Parsed = Vec<Vec<u8>>;
type Answer = i64;

fn score(c: u8) -> i64 {
    match c as char {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!(),
    }
}

fn get_path(
    grid: &FxHashMap<aoc::Point, u8>,
    s: aoc::Point,
    g: aoc::Point,
) -> (i64, Vec<aoc::Point>) {
    aoc::dijkstra_grid(
        grid,
        |_p: &aoc::Point, c: &u8| *c == b'.',
        |_pa, _va, _pb, _pv| Some(1),
        s,
        g,
    )
    .unwrap()
}

fn p2ix(p: aoc::Point) -> usize {
    (p[1] * 13 + p[0]) as usize
}

fn ix2p(ix: usize) -> aoc::Point {
    [(ix % 13) as i64, (ix / 13) as i64]
}

fn is_reachable(
    paths: &FxHashMap<(aoc::Point, aoc::Point), (i64, Vec<aoc::Point>)>,
    grid: &[u8],
    s: aoc::Point,
    g: aoc::Point,
) -> Option<i64> {
    if let Some((e, path)) = paths.get(&(s, g)) {
        if path
            .iter()
            .skip(1)
            .map(|p| grid[p2ix(*p)])
            .all(|c| c == b'.')
        {
            return Some(*e);
        }
    }
    None
}

fn is_in_hallway(p: aoc::Point) -> bool {
    p[0] > 0 && p[0] < 12 && p[1] == 1
}

fn is_empty(grid: &[u8], num: i64, p: aoc::Point) -> bool {
    for y in 2..(2 + num) {
        if grid[p2ix([p[0], y])] != b'.' {
            return false;
        }
    }
    true
}

fn is_same(grid: &[u8], num: i64, p: aoc::Point, a: u8) -> bool {
    let start = p[1] + 1;
    for y in start..(2 + num) {
        if grid[p2ix([p[0], y])] != a {
            return false;
        }
    }
    true
}

fn is_blocking(grid: &[u8], num: i64, p: aoc::Point, a: u8) -> bool {
    !is_same(grid, num, p, a)
}

fn solve(parsed_grid: &Vec<Vec<u8>>, num: i64) -> Option<i64> {
    // Make a flat grid with only amphipods and empty positions
    let mut start = vec![];
    start.reserve(parsed_grid.len() * parsed_grid[0].len());
    for p in parsed_grid.points() {
        start.push(parsed_grid.get_value(p).unwrap());
    }
    let mut goals = FxHashMap::default();
    for (c, x) in [(b'A', 3), (b'B', 5), (b'C', 7), (b'D', 9)] {
        let mut v = vec![];
        for y in 2..(2 + num) {
            v.push([x, y]);
        }
        goals.insert(c, v);
    }
    let mut possible_moves = vec![[1i64, 1], [2, 1], [4, 1], [6, 1], [8, 1], [10, 1], [11, 1]];
    for ps in goals.values() {
        possible_moves.extend_from_slice(ps);
    }
    // Pre-compute the shortest paths
    let mut paths = FxHashMap::default();
    paths.reserve(220);
    let mut empty_g = FxHashMap::default();
    for (ix, c) in start.iter().enumerate() {
        let p = ix2p(ix);
        if c.is_ascii_alphabetic() {
            empty_g.insert(p, b'.');
        } else {
            empty_g.insert(p, *c);
        }
    }
    for combo in possible_moves.iter().copied().permutations(2) {
        paths.insert((combo[0], combo[1]), get_path(&empty_g, combo[0], combo[1]));
    }
    let mut gscore = FxHashMap::default();
    gscore.reserve(100000);
    let mut fscore = FxHashMap::default();
    gscore.reserve(100000);

    gscore.insert(start.clone(), 0);

    let mut todo = BinaryHeap::new();
    todo.reserve(100000);
    todo.push(Reverse((0, start)));

    while let Some(Reverse((_est, pos))) = todo.pop() {
        // Are all in goals?
        let mut ok = true;
        for (ix, a) in pos.iter().enumerate() {
            let from = ix2p(ix);
            if !(*a as char).is_ascii_alphabetic() {
                continue;
            }
            if !goals.get(a).unwrap().contains(&from) {
                ok = false;
                break;
            }
        }
        if ok {
            return Some(*gscore.get(&pos).unwrap());
        }

        let g = *gscore.entry(pos.clone()).or_insert(i64::MAX);

        // Nope, find moves
        for (ix, a) in pos.iter().enumerate() {
            let from = ix2p(ix);
            // We're only moving the amphipods
            if !a.is_ascii_alphabetic() {
                continue;
            }
            // Already in the right place?
            // Need to move if not in goal, or blocking other not in goal
            let should_move = !goals.get(a).unwrap().contains(&from) // Not in my goal
                || is_blocking(&pos, num, from, *a); // Blocking
            if !should_move {
                continue;
            }
            let mut moves = vec![];
            // Try all possible moves
            for to in &possible_moves {
                // Don't move to self or non-empty
                if *to == from || pos[p2ix(*to)] != b'.' {
                    continue;
                }
                // Don't move from hallway to hallway
                if is_in_hallway(from) && is_in_hallway(*to) {
                    continue;
                }
                // Move to hallway or my goal
                // "and will only move into the leftmost room if that room is empty or
                // if it only contains other Amber amphipods."
                let ok = is_in_hallway(*to)
                    || (goals.get(a).unwrap().contains(to) && // is goal
			(is_empty(&pos, num, *to)
			|| is_same(&pos, num, *to, *a)));
                if !ok {
                    continue;
                }
                // This is a legit point to move to from this position, is it reachable?
                if let Some(e) = is_reachable(&paths, &pos, from, *to) {
                    moves.push((*to, e));
                }
            }
            for (mv, n) in moves {
                let mut new_pos = pos.clone();
                new_pos[p2ix(from)] = b'.';
                new_pos[p2ix(mv)] = *a;
                let e = score(*a);
                let new_g = g + e * n;
                let nb_g = gscore.entry(new_pos.clone()).or_insert(i64::MAX);
                if new_g < *nb_g {
                    *nb_g = new_g;
                    let mut min_dist = 0;
                    for (ix, c) in new_pos
                        .iter()
                        .enumerate()
                        .filter(|(_p, c)| c.is_ascii_alphabetic())
                    {
                        let first_goal = goals.get(c).unwrap()[0];
                        let p = ix2p(ix);
                        if p != first_goal {
                            min_dist += score(*c) * paths.get(&(p, first_goal)).unwrap().0;
                        }
                    }
                    let new_f = new_g + min_dist;
                    *fscore.entry(new_pos.clone()).or_insert(i64::MAX) = new_f;
                    todo.push(Reverse((new_f, new_pos)));
                }
            }
        }
    }
    None
}

fn part1(grid: &Parsed) -> Answer {
    solve(grid, 2).unwrap()
}

fn part2(grid: &Parsed) -> Answer {
    let mut g = grid.to_owned();
    g.splice(
        3..3,
        vec![
            "  #D#C#B#A#  ".as_bytes().to_owned(),
            "  #D#B#A#C#  ".as_bytes().to_owned(),
        ],
    );
    solve(&g, 4).unwrap()
}

fn parse(lines: &[String]) -> Parsed {
    let mut g = aoc::parse_grid_to(lines, |c| c as u8);
    let w = g.iter().map(|x| x.len()).max().unwrap();
    for line in g.iter_mut() {
        while line.len() < w {
            line.push(b' ')
        }
    }
    g
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        include_str!("sample.txt")
            .lines()
            .map(|x| x.into())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 12521);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 44169);
    }

    #[test]
    fn test_p2ix() {
        for y in 0..7 {
            for x in 0..13 {
                let p = [x, y];
                let ix = p2ix(p);
                assert_eq!(ix2p(ix), p);
            }
        }
    }
}
