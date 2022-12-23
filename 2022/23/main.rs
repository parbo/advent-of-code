use aoc::{
    neighbors_incl_diagonals, point_add, Grid, GridDrawer, Point, PrintGridDrawer, EAST, NORTH,
    NORTH_EAST, NORTH_WEST, SOUTH, SOUTH_EAST, SOUTH_WEST, WEST,
};
use std::{collections::HashMap, iter::*};

type Parsed = HashMap<Point, char>;

fn solve(data: &Parsed, max: Option<usize>) -> (i64, usize) {
    let mut rules = vec![
        ([NORTH, NORTH_EAST, NORTH_WEST], NORTH),
        ([SOUTH, SOUTH_EAST, SOUTH_WEST], SOUTH),
        ([WEST, NORTH_WEST, SOUTH_WEST], WEST),
        ([EAST, NORTH_EAST, SOUTH_EAST], EAST),
    ];
    let mut g = data.clone();
    let mut gd = PrintGridDrawer::new(|c| c);
    let mut rounds = 0;
    loop {
        let mut proposed: HashMap<Point, Vec<Point>> = HashMap::new();
        for p in g.keys() {
            if neighbors_incl_diagonals(*p).any(|n| g.get_value(n).is_some()) {
                for (nb, d) in &rules {
                    if nb.iter().all(|n| {
                        let pp = point_add(*p, *n);
                        g.get_value(pp).is_none()
                    }) {
                        let pp = point_add(*p, *d);
                        proposed.entry(pp).or_default().push(*p);
                        break;
                    }
                }
            }
        }
        rounds += 1;
        if proposed.is_empty() {
            break;
        }
        for (to, from) in &proposed {
            if from.len() == 1 {
                let r = g.remove(&from[0]);
                assert!(r.is_some());
                g.insert(*to, '#');
            }
        }
        let r = rules.remove(0);
        rules.push(r);
        // gd.draw(&g);
        // println!();
        if let Some(x) = max {
            if x == rounds {
                break;
            }
        }
    }
    let ([min_x, min_y], [max_x, max_y]) = g.extents();
    let mut empty = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if g.get_value([x, y]).is_none() {
                empty += 1;
            }
        }
    }
    (empty, rounds)
}

fn part1(data: &Parsed) -> i64 {
    solve(data, Some(10)).0
}
fn part2(data: &Parsed) -> usize {
    solve(data, None).1
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid_to_sparse(lines, |c| if c == '#' { Some(c) } else { None })
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
        assert_eq!(part1(&parse(&example())), 110);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 20);
    }
}
