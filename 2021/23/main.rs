use aoc::{Grid, GridDrawer};
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;
use std::cmp::Reverse;

type ParsedItem = Vec<char>;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn get_path(grid: &Parsed, s: aoc::Point, g: aoc::Point) -> Option<(i64, Vec<aoc::Point>)> {
    aoc::dijkstra_grid(
	grid,
	|_p, c| *c == '.',
	|_pa, _va, _pb, _pv| Some(1),
	s,
	g)
}

fn solve(
    grid: &Parsed,
    goals: &HashMap<char, Vec<aoc::Point>>,
    init_pos: &[(char, aoc::Point)],
) -> Option<i64> {
    let mut todo = BinaryHeap::new();
    todo.push(Reverse((0, init_pos.to_owned())));
//    let mut visited: HashSet<Vec<(char, aoc::Point)>> = HashSet::new();
    while let Some(Reverse((energy, pos))) = todo.pop() {
	println!("{}, {:?}", energy, pos);
        // if visited.contains(&pos) {
        //     continue;
        // }
	// Are all in goals?
	let mut ok = true;
	for (a, p) in &pos {
	    if goals.get(a).unwrap().contains(p) {
		ok = false;
		break;
	    }
	}
	if ok {
	    return Some(energy);
	}
	// Nope, find moves
	for (a, p) in &pos {
	    let mut moves: Vec<aoc::Point> = vec![];
	    if goals.get(a).unwrap().contains(p) {
		// Already in the right spot
		continue
	    }
	    // Are we in the hallway?
	    if p[0] > 0 && p[0] < 12 && p[1] == 1 {
		// Find paths to goal
		if let Some(agoals) = goals.get(a) {
		    for g in agoals {
			if let Some((_, _)) = get_path(grid, *p, *g) {
			    // goal reached, move there
			    // println!("move to goal");
			    moves.push(*g);
			}
		    }
		}
	    } else {
		// Try all possible moves out
		for x in 1..=11 {
		    let hp = [x, 1];
                    if let Some((_, _)) = get_path(grid, *p, hp) {
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
		for (aa, pp) in new_pos.iter_mut() {
		    if *aa == *a && *pp == *p {
			*pp = mv;
			break;
		    }
		}
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
    let goals = HashMap::from([
        ('A', vec![[3, 2], [3, 3]]),
        ('B', vec![[5, 2], [5, 3]]),
        ('C', vec![[7, 2], [7, 3]]),
        ('D', vec![[9, 2], [9, 3]]),
    ]);
    // Starting positions
    let mut start = vec![];
    for p in grid.points() {
        if let Some(x) = grid.get_value(p) {
            if x.is_ascii_alphabetic() {
                start.push((x, p));
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
