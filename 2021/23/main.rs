use aoc::{Grid, GridDrawer, Itertools};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::time::Instant;

type Parsed = Vec<Vec<char>>;
type Answer = i64;

fn get_path(
    grid: &BTreeMap<aoc::Point, char>,
    s: aoc::Point,
    g: aoc::Point,
) -> (i64, Vec<aoc::Point>) {
    aoc::dijkstra_grid(
        grid,
        |_p: &aoc::Point, c: &char| *c == '.',
        |_pa, _va, _pb, _pv| Some(1),
        s,
        g,
    )
    .unwrap()
}

fn is_reachable(
    paths: &HashMap<(aoc::Point, aoc::Point), (i64, Vec<aoc::Point>)>,
    grid: &BTreeMap<aoc::Point, char>,
    s: aoc::Point,
    g: aoc::Point,
) -> Option<i64> {
    if let Some((e, path)) = paths.get(&(s, g)) {
        if path
            .iter()
            .skip(1)
            .map(|p| grid.get_value(*p).unwrap())
            .all(|c| c == '.')
        {
            return Some(*e);
        }
    }
    None
}

fn is_in_hallway(p: aoc::Point) -> bool {
    p[0] > 0 && p[0] < 12 && p[1] == 1
}

fn is_empty(grid: &BTreeMap<aoc::Point, char>, _num: i64, p: aoc::Point) -> bool {
    p[1] == 3 && *grid.get(&[p[0], 2]).unwrap() == '.'
}

fn is_same(grid: &BTreeMap<aoc::Point, char>, _num: i64, p: aoc::Point, a: char) -> bool {
    p[1] == 2 && *grid.get(&[p[0], 3]).unwrap() == a
}

fn is_blocking(grid: &BTreeMap<aoc::Point, char>, _num: i64, p: aoc::Point, a: char) -> bool {
    p[1] == 2 && *grid.get(&[p[0], 3]).unwrap() != a
}

fn solve(grid: &BTreeMap<aoc::Point, char>, num: i64) -> Option<i64> {
    let mut goals = HashMap::new();
    for (c, x) in [('A', 3), ('B', 5), ('C', 7), ('D', 9)] {
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
    let mut paths = HashMap::new();
    let empty_g = grid
        .iter()
        .map(|(p, c)| {
            if c.is_ascii_alphabetic() {
                (*p, '.')
            } else {
                (*p, *c)
            }
        })
        .collect::<BTreeMap<aoc::Point, char>>();
    for combo in possible_moves.iter().copied().permutations(2) {
        paths.insert((combo[0], combo[1]), get_path(&empty_g, combo[0], combo[1]));
    }
    // println!("possible: {:?}, {}", possible_moves, paths.len());
    let mut todo = BinaryHeap::new();
    todo.push(Reverse((0, grid.clone())));

    let mut gscore = HashMap::new();
    let mut fscore = HashMap::new();

    gscore.insert(grid.clone(), 0);

    let mut ctr = 0;
    while let Some(Reverse((est, pos))) = todo.pop() {
        ctr += 1;
        if ctr % 10000 == 0 {
            println!("{:?}, {:?}, {:?}", est, pos, todo.len());
        }

        // Are all in goals?
        let mut ok = true;
        for (from, a) in &pos {
            if !a.is_ascii_alphabetic() {
                continue;
            }
            if !goals.get(a).unwrap().contains(from) {
                ok = false;
                break;
            }
        }
        if ok {
            return Some(*gscore.get(&pos).unwrap());
        }

        let g = *gscore.entry(pos.clone()).or_insert(i64::MAX);
        let f = fscore.entry(pos.clone()).or_insert(i64::MAX);
        if *f <= est {
            continue;
        }

        // Nope, find moves
        for (from, a) in &pos {
            // We're only moving the amphipods
            if !a.is_ascii_alphabetic() {
                continue;
            }
            // Already in the right place?
            // Need to move if not in goal, or blocking other not in goal
            let should_move = !goals.get(a).unwrap().contains(from) // Not in my goal
                || is_blocking(&pos, num, *from, *a); // Blocking
            if !should_move {
                continue;
            }
            let mut moves = vec![];
            // Try all possible moves
            for to in &possible_moves {
                // Don't move to self or non-empty
                if *to == *from || *pos.get(to).unwrap() != '.' {
                    continue;
                }
                // Don't move from hallway to hallway
                if is_in_hallway(*from) && is_in_hallway(*to) {
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
                if let Some(e) = is_reachable(&paths, &pos, *from, *to) {
                    moves.push((*to, e));
                }
            }
            // if moves.is_empty() {
            //     println!("NO MOVES!");
            // }
            for (mv, n) in moves {
                let mut new_pos = pos.clone();
                new_pos.insert(*from, '.');
                new_pos.insert(mv, *a);
                // println!("old-pos: {:?}", pos);
                // println!("new-pos: {:?}", new_pos);
                let e = match a {
                    'A' => 1,
                    'B' => 10,
                    'C' => 100,
                    'D' => 1000,
                    _ => panic!(),
                };
                // let mut gd = aoc::PrintGridDrawer::new(|c| c);
                // println!("===============================");
                // println!("{:?}, {:?}", pos.extents(), new_pos.extents());
                // gd.draw(&pos);
                // gd.draw(&new_pos);
                let new_g = g + e * n;
                let nb_g = gscore.entry(new_pos.clone()).or_insert(i64::MAX);
                if new_g < *nb_g {
                    *nb_g = new_g;
                    let mut min_dist = 0;
                    for (p, c) in new_pos.iter().filter(|(_p, c)| **c != '.') {
                        let first_goal = goals.get(c).unwrap()[0];
                        if *p != first_goal {
                            min_dist += paths.get(&(*p, first_goal)).unwrap().0;
                        }
                    }
                    let new_f = new_g + min_dist;
                    todo.push(Reverse((new_f, new_pos)));
                }
            }
        }
    }
    None
}

fn part1(grid: &Parsed) -> Answer {
    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    gd.draw(grid);
    // Make a sparse grid with only amphipods and empty positions
    let mut start = BTreeMap::new();
    for p in grid.points() {
        match grid.get_value(p) {
            Some('#') | Some(' ') | None => (),
            Some(x) => {
                start.insert(p, x);
            }
        }
    }
    solve(&start, 2).unwrap()
}

fn part2(_: &Parsed) -> Answer {
    0
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid(lines)
}

fn main() {
    let start_time = Instant::now();
    let (part, lines) = aoc::read_lines();
    let io_time = Instant::now();
    let parsed = parse(&lines);
    let parse_time = Instant::now();
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    let done_time = Instant::now();
    println!(
        "read: {:?}, parse: {:?}, solve: {:?}\n",
        io_time.duration_since(start_time),
        parse_time.duration_since(io_time),
        done_time.duration_since(parse_time)
    );
    println!("{}", result);
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
}
