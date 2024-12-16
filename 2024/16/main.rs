use std::{cmp::Reverse, collections::BinaryHeap, iter::*};

use aoc::Grid;

#[cfg(feature = "vis")]
use aoc::GridDrawer;

type Parsed = Vec<Vec<char>>;

#[cfg(feature = "vis")]
fn dir_to_char(d: aoc::Point) -> char {
    match d {
        aoc::EAST => '>',
        aoc::WEST => '<',
        aoc::NORTH => '^',
        aoc::SOUTH => 'v',
        _ => '.',
    }
}

fn dijkstra_grid(
    grid: &Parsed,
    start: aoc::Point,
    goal: aoc::Point,
    all: bool,
) -> (i64, aoc::FxHashSet<aoc::Point>) {
    let mut frontier = BinaryHeap::new();
    let mut visited: aoc::FxHashSet<(aoc::Point, aoc::Point)> = aoc::FxHashSet::default();
    let mut came_from: aoc::FxHashMap<
        (aoc::Point, aoc::Point),
        aoc::FxHashMap<(aoc::Point, aoc::Point), i64>,
    > = aoc::FxHashMap::default();
    let mut paths = aoc::FxHashSet::default();
    paths.insert(start);
    frontier.push(Reverse((0, start, aoc::EAST)));
    let mut best = None;
    while let Some(Reverse((score, current, dir))) = frontier.pop() {
        if let Some(b) = best {
            if score > b {
                break;
            }
        }
        if !all && visited.contains(&(current, dir)) {
            continue;
        }
        if current == goal {
            best = Some(score);
            let curr = (goal, dir);
            let mut todo = vec![curr];
            paths.insert(goal);
            while let Some(curr) = todo.pop() {
                if let Some(cf) = came_from.get(&curr) {
                    let b = cf.values().min().unwrap();
                    for ((p, d), c) in cf {
                        if c == b {
                            todo.push((*p, *d));
                            paths.insert(*p);
                        }
                    }
                }
            }
            if all {
                continue;
            } else {
                break;
            }
        }
        for (nb, ndir, cost) in [
            (aoc::point_add(current, dir), dir, 1),
            (
                current,
                *aoc::DIRECTION_ROTATE_LEFT.get(&dir).unwrap(),
                1000,
            ),
            (
                current,
                *aoc::DIRECTION_ROTATE_RIGHT.get(&dir).unwrap(),
                1000,
            ),
        ] {
            if visited.contains(&(nb, ndir)) {
                continue;
            }
            if let Some(value) = grid.get_value(nb) {
                if value != '#' {
                    let new_score = score + cost;
                    let e = came_from.entry((nb, ndir)).or_default();
                    let ee = e.entry((current, dir)).or_default();
                    if cost <= *ee {
                        *ee = cost;
                    }
                    frontier.push(Reverse((new_score, nb, ndir)));
                }
            }
        }
        visited.insert((current, dir));
    }
    (best.unwrap(), paths)
}

fn part1(data: &Parsed) -> i64 {
    let s = data
        .points()
        .find(|p| data.get_value(*p) == Some('S'))
        .unwrap();
    let e = data
        .points()
        .find(|p| data.get_value(*p) == Some('E'))
        .unwrap();
    let p = dijkstra_grid(data, s, e, false);
    p.0
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
    let p = dijkstra_grid(data, s, e, true);
    p.1.len() as i64
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

    fn example2() -> Vec<String> {
        let s = include_str!("example2.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 7036);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&parse(&example2())), 11048);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 45);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&parse(&example2())), 64);
    }
}
