use std::iter::*;
use std::time::Instant;
use aoc::{Grid, GridDrawer};

type ParsedItem = Vec<char>;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(grid: &[ParsedItem]) -> Answer {
    let mut g = grid.to_owned();
    // let mut gd = aoc::PrintGridDrawer::new(|c| c);
    let mut step = 0;
    let ([minx, miny], [maxx, maxy]) = g.extents();
    loop {
	step += 1;
	let mut new_g = g.clone();
	for (c, d) in [('>', aoc::EAST), ('v', aoc::SOUTH)] {
	    let mut nn_g = new_g.clone();
	    for p in new_g.points() {
		if let Some(v) = new_g.get_value(p) {
		    if v == c {
			let mut dp = aoc::point_add(p, d);
			if dp[0] > maxx {
			    dp[0] = minx;
			}
			if dp[1] > maxy {
			    dp[1] = miny;
			}
			if let Some('.') = new_g.get_value(dp) {
			    nn_g.set_value(p, '.');
			    nn_g.set_value(dp, c);
			}
		    }
		}
	    }
	    new_g = nn_g;
	}
	// gd.draw(&new_g);
	// println!();
	if new_g == g {
	    break;
	}
	g = new_g;
    }
    step
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
