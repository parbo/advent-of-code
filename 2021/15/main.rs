#![allow(clippy::ptr_arg)]

use aoc::Grid;
use std::time::Instant;

type ParsedItem = Vec<i64>;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn manhattan(n: aoc::Point, goal: aoc::Point) -> i64 {
    (goal[0] - n[0]).abs() + (goal[1] - n[1]).abs()
}

fn solve(grid: &Parsed) -> i64 {
    let (start, goal) = grid.extents();
    if let Some((cost, _path)) = aoc::astar_grid(
        grid,
        |_p, _c| true,
        |_p1, _c1, _p2, c2| Some(*c2),
        start,
        goal,
    ) {
        return cost;
    }
    -1
}

fn part1(g: &Parsed) -> Answer {
    solve(g)
}

fn part2(g: &Parsed) -> Answer {
    let rows = g.len() as i64;
    let cols = g[0].len() as i64;
    let mut grid = vec![vec![0i64; 5 * cols as usize]; 5 * rows as usize];
    for p in grid.points() {
        let orig = [p[0] % cols, p[1] % rows];
        let new_v = 1
            + (manhattan([0, 0], [p[0] / cols, p[1] / rows]) + g.get_value(orig).unwrap() - 1) % 9;
        grid.set_value(p, new_v);
    }
    solve(&grid)
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid_to(lines, |x: char| x.to_digit(10).unwrap() as i64)
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
        vec![
            "1163751742".into(),
            "1381373672".into(),
            "2136511328".into(),
            "3694931569".into(),
            "7463417111".into(),
            "1319128137".into(),
            "1359912421".into(),
            "3125421639".into(),
            "1293138521".into(),
            "2311944581".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 315);
    }
}
