use aoc::{Grid, GridDrawer};
use std::{
    collections::{BTreeSet, HashSet},
    iter::*,
};

type Parsed = Vec<Vec<i64>>;
type Answer = i64;

fn part1(grid: &Parsed) -> Answer {
    let mut g = grid.clone();
    let mut flashes = 0;
//    let mut d = aoc::PrintGridDrawer::new(|c: i64| c.to_string().chars().next().unwrap());
    for i in 0..100 {
        let mut flash = BTreeSet::new();
        // Increase by 1
        for p in g.points() {
            let v = g.get_value(p).unwrap() + 1;
            g.set_value(p, v);
            if v == 10 {
                flash.insert(p);
            }
        }
        // Do the flash
	let mut flashed = HashSet::new();
        loop {
            let mp = flash.iter().cloned().next();
            if let Some(p) = mp {
		flash.remove(&p);
		flashed.insert(p);
                for d in aoc::DIRECTIONS_INCL_DIAGONALS {
                    let nb = aoc::point_add(p, d);
                    if let Some(v) = g.get_value(nb) {
			g.set_value(nb, v + 1);
			if v + 1 == 10 {
                            flash.insert(nb);
			}
		    }
                }
            } else {
                break;
            }
        }
        flashes += flashed.len();
        // reset to 0
        for p in flashed {
            g.set_value(p, 0);
        }
	// if i <= 10 || i % 10 == 0 {
	//     d.draw(&g);
	// }
    }
    flashes as Answer
}

fn part2(grid: &Parsed) -> Answer {
    let mut g = grid.clone();
    let total = g.len() * g[0].len();
//    let mut d = aoc::PrintGridDrawer::new(|c: i64| c.to_string().chars().next().unwrap());
    let mut i = 0;
    loop {
        let mut flash = BTreeSet::new();
        // Increase by 1
        for p in g.points() {
            let v = g.get_value(p).unwrap() + 1;
            g.set_value(p, v);
            if v == 10 {
                flash.insert(p);
            }
        }
        // Do the flash
	let mut flashed = HashSet::new();
        loop {
            let mp = flash.iter().cloned().next();
            if let Some(p) = mp {
		flash.remove(&p);
		flashed.insert(p);
                for d in aoc::DIRECTIONS_INCL_DIAGONALS {
                    let nb = aoc::point_add(p, d);
                    if let Some(v) = g.get_value(nb) {
			g.set_value(nb, v + 1);
			if v + 1 == 10 {
                            flash.insert(nb);
			}
		    }
                }
            } else {
                break;
            }
        }
	let num = flashed.len();
        // reset to 0
        for p in flashed {
            g.set_value(p, 0);
        }
	// if i <= 10 || i % 10 == 0 {
	//     d.draw(&g);
	// }
	i += 1;
	if num == total {
	    break;
	}
    }
    i as Answer
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid_to(lines, |x| x.to_digit(10).unwrap().into())
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "5483143223".into(),
            "2745854711".into(),
            "5264556173".into(),
            "6141336146".into(),
            "6357385478".into(),
            "4167524645".into(),
            "2176841721".into(),
            "6882881134".into(),
            "4846848554".into(),
            "5283751526".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 195);
    }
}
