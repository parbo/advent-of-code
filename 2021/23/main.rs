use aoc::{Grid, GridDrawer};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashSet};
use std::time::Instant;

type ParsedItem = Vec<char>;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn is_reachable(
    grid: &Parsed,
    pos: &BTreeMap<aoc::Point, char>,
    s: aoc::Point,
    g: aoc::Point,
) -> bool {
    aoc::dijkstra_grid(
        grid,
        &|p: &aoc::Point, c: &char| {
            if let Some(_) = pos.get(p) {
                false
            } else {
                *c != '#'
            }
        },
        |_pa, _va, _pb, _pv| Some(1),
        s,
        g,
    )
    .is_some()
}

fn solve(
    grid: &Parsed,
    goals: &BTreeMap<char, Vec<aoc::Point>>,
    init_pos: &BTreeMap<aoc::Point, char>,
) -> Option<i64> {
    let mut todo = BinaryHeap::new();
    todo.push(Reverse((0, init_pos.clone())));
    let mut visited = HashSet::new();
    while let Some(Reverse((energy, pos))) = todo.pop() {
        println!("{}, {:?}", energy, pos);
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos.clone());
        // Are all in goals?
        let mut ok = true;
        for (p, a) in &pos {
            if goals.get(a).unwrap().contains(p) {
                ok = false;
                break;
            }
        }
        if ok {
            return Some(energy);
        }
        // Nope, find moves
        for (p, a) in &pos {
            let mut moves: Vec<aoc::Point> = vec![];
            if goals.get(a).unwrap().contains(p) {
                // Already in the right spot
                continue;
            }
            // Are we in the hallway?
            if p[0] > 0 && p[0] < 12 && p[1] == 1 {
                // Find paths to goal
                if let Some(agoals) = goals.get(a) {
                    for g in agoals {
                        let ok = if g[1] == 3 {
                            // goal empty and same underneath
                            *pos.get(g).unwrap_or(&'.') == '.'
                                && *pos.get(&[g[0], g[1] + 1]).unwrap_or(&'.') == *a
                        } else {
                            // goal empty
                            *pos.get(g).unwrap_or(&'.') == '.'
                        };
                        if ok && is_reachable(grid, &pos, *p, *g) {
                            // goal reached, move there
                            // println!("move to goal");
                            moves.push(*g);
                        }
                    }
                }
            } else {
                // Try all possible moves out
                for x in [1, 2, 4, 6, 8, 10, 11] {
                    let hp = [x, 1];
                    if is_reachable(grid, &pos, *p, hp) {
                        // hallway reached, move there
                        // println!("move to hallway");
                        moves.push(hp);
                    }
                }
            }
            // if moves.is_empty() {
            // 	println!("NO MOVES!");
            // }
            for mv in moves {
                let mut new_pos = pos.clone();
                new_pos.remove(p);
                new_pos.insert(*p, *a);
                // println!("old-pos: {:?}", pos);
                // println!("new-pos: {:?}", new_pos);
                let e = match a {
                    'A' => 1,
                    'B' => 10,
                    'C' => 100,
                    'D' => 1000,
                    _ => panic!(),
                };
                todo.push(Reverse((energy + e * aoc::manhattan(*p, mv), new_pos)));
            }
        }
    }
    None
}

fn part1(grid: &Parsed) -> Answer {
    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    gd.draw(grid);
    // Hardcoded goals
    let goals = BTreeMap::from([
        ('A', vec![[3, 2], [3, 3]]),
        ('B', vec![[5, 2], [5, 3]]),
        ('C', vec![[7, 2], [7, 3]]),
        ('D', vec![[9, 2], [9, 3]]),
    ]);
    // Starting positions
    let mut start = BTreeMap::new();
    for p in grid.points() {
        if let Some(x) = grid.get_value(p) {
            if x.is_ascii_alphabetic() {
                start.insert(p, x);
            }
        }
    }
    solve(grid, &goals, &start).unwrap()
}

fn part2(_: &[ParsedItem]) -> Answer {
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
    // use super::*;

    // fn example() -> Vec<String> {
    // 	   vec![
    //         "0".into()
    //     ]
    // }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&parse(&example())), 0);
    // }
}
