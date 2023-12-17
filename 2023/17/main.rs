use std::{cmp::Reverse, collections::BinaryHeap};

use aoc::Grid;

#[cfg(feature = "vis")]
use std::collections::HashMap;

#[cfg(feature = "vis")]
use aoc::GridDrawer;

type Parsed = Vec<Vec<char>>;

#[cfg(feature = "vis")]
fn draw(
    grid: &Parsed,
    dir: aoc::Point,
    sm: i64,
    start: aoc::Point,
    goal: aoc::Point,
    came_from: &HashMap<(aoc::Point, i64, aoc::Point), (aoc::Point, i64, aoc::Point)>,
) {
    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    let mut curr = (dir, sm, goal);
    let mut g = grid.clone();
    while curr.2 != start {
        g.set_value(
            curr.2,
            match curr.0 {
                aoc::NORTH => '^',
                aoc::SOUTH => 'v',
                aoc::WEST => '<',
                aoc::EAST => '>',
                _ => panic!(),
            },
        );
        curr = *came_from.get(&curr).unwrap();
        // path.push(curr);
    }
    gd.draw(&g);
    println!();
}

fn solve(grid: &Parsed, min: i64, max: i64) -> i64 {
    let (start, goal) = grid.extents();
    let mut frontier = BinaryHeap::new();
    let mut visited = aoc::FxHashSet::default();
    #[cfg(feature = "vis")]
    let mut came_from: HashMap<(aoc::Point, i64, aoc::Point), (aoc::Point, i64, aoc::Point)> =
        HashMap::new();
    frontier.push(Reverse((0, (aoc::EAST, -1i64, start))));
    frontier.push(Reverse((0, (aoc::SOUTH, -1i64, start))));
    while let Some(Reverse((score, k))) = frontier.pop() {
        if visited.contains(&k) {
            continue;
        }
        let (dir, sm, current) = k;
        let curr = current;
        if curr == goal && sm >= min {
            #[cfg(feature = "vis")]
            draw(grid, dir, sm, start, goal, &came_from);
            return score;
        }
        for nb in aoc::neighbors(curr) {
            if let Some(value) = grid.get_value(nb) {
                let new_dir = aoc::point_sub(nb, curr);
                if *aoc::DIRECTION_OPPOSITE.get(&dir).unwrap() == new_dir {
                    continue;
                }
                // No turning before min steps
                if new_dir != dir && sm < min {
                    continue;
                }
                let new_sm = if new_dir == dir { sm + 1 } else { 0 };
                // Must turn before max steps
                if new_sm < max {
                    let new_score = score + value.to_digit(10).unwrap() as i64;
                    if visited.contains(&(new_dir, new_sm, nb)) {
                        continue;
                    }
                    #[cfg(feature = "vis")]
                    came_from.insert((new_dir, new_sm, nb), (dir, sm, current));
                    frontier.push(Reverse((new_score, (new_dir, new_sm, nb))));
                }
            }
        }
        visited.insert((dir, sm, current));
    }
    panic!();
}

fn part1(grid: &Parsed) -> i64 {
    solve(grid, 0, 3)
}

fn part2(grid: &Parsed) -> i64 {
    solve(grid, 3, 10)
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
        assert_eq!(part1(&parse(&example())), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 94);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&parse(&example2())), 71);
    }
}
