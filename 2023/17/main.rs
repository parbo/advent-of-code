use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc::Grid;

type Parsed = Vec<Vec<char>>;

fn part1(grid: &Parsed) -> i64 {
    let (start, goal) = grid.extents();
    let mut frontier = BinaryHeap::new();
    let mut visited = HashSet::new();
    frontier.push(Reverse((0, (aoc::EAST, -1i64, start))));
    frontier.push(Reverse((0, (aoc::SOUTH, -1i64, start))));
    while let Some(Reverse((score, k))) = frontier.pop() {
        if visited.contains(&k) {
            continue;
        }
        let (dir, sm, current) = k;
        let curr = current;
        if curr == goal {
            return score;
        }
        for nb in aoc::neighbors(curr) {
            if let Some(value) = grid.get_value(nb) {
                let new_dir = aoc::point_sub(nb, curr);
                if *aoc::DIRECTION_OPPOSITE.get(&dir).unwrap() == new_dir {
                    continue;
                }
                let new_sm = if new_dir == dir { sm + 1 } else { 0 };
                if new_sm < 3 {
                    let new_score = score + value.to_digit(10).unwrap() as i64;
                    if visited.contains(&(new_dir, new_sm, nb)) {
                        continue;
                    }
                    frontier.push(Reverse((new_score, (new_dir, new_sm, nb))));
                }
            }
        }
        visited.insert((dir, sm, current));
    }
    panic!();
}

fn part2(grid: &Parsed) -> i64 {
    let (start, goal) = grid.extents();
    let mut frontier = BinaryHeap::new();
    let mut visited = HashSet::new();
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
        if curr == goal && sm > 2 {
            return score;
        }
        for nb in aoc::neighbors(curr) {
            if let Some(value) = grid.get_value(nb) {
                let new_dir = aoc::point_sub(nb, curr);
                if *aoc::DIRECTION_OPPOSITE.get(&dir).unwrap() == new_dir {
                    continue;
                }
                // No turning before 4 steps
                if new_dir != dir && sm < 3 {
                    continue;
                }
                let new_sm = if new_dir == dir { sm + 1 } else { 0 };
                // Must turn before ten steps
                if new_sm < 10 {
                    let new_score = score + value.to_digit(10).unwrap() as i64;
                    if visited.contains(&(new_dir, new_sm, nb)) {
                        continue;
                    }
                    came_from.insert((new_dir, new_sm, nb), (dir, sm, current));
                    frontier.push(Reverse((new_score, (new_dir, new_sm, nb))));
                }
            }
        }
        visited.insert((dir, sm, current));
    }
    panic!();
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
