use aoc;
use aoc::GridDrawer;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::*;

fn to_char(ch: i128) -> char {
    std::char::from_u32(ch as u32).unwrap()
}

fn is_scaffold(grid: &HashMap<(i128, i128), i128>, pos: (i128, i128)) -> bool {
    if let Some(x) = grid.get(&pos) {
        *x == 35
    } else {
        false
    }
}

fn find_align(grid: &HashMap<(i128, i128), i128>) -> Vec<(i128, i128)> {
    let scaffold: Vec<_> = grid.iter().filter(|x| *x.1 == 35).map(|x| *x.0).collect();
    let mut align = vec![];
    for (x, y) in scaffold {
        // find the +
        if is_scaffold(&grid, (x + 1, y))
            && is_scaffold(&grid, (x - 1, y))
            && is_scaffold(&grid, (x, y + 1))
            && is_scaffold(&grid, (x, y - 1))
        {
            align.push((x, y));
        }
    }
    align
}

#[derive(Copy, Clone, Debug, Hash)]
enum Dir {
    Up,
    Left,
    Right,
    Down,
}

fn to_dir(d: char) -> Dir {
    match d {
	'<' => Dir::Left,
	'>' => Dir::Right,
	'^' => Dir::Up,
	'v' => Dir::Down,
	_ => panic!(),
    }
}

fn get_new_pos(pos: (i128, i128), dir: Dir, ch: char) -> ((i128, i128), Dir) {
    match dir {
	Dir::Up => {
	    match ch {
		'R' => (pos, Dir::Right),
		'L' => (pos, Dir::Left),
		_ => ((pos.0, pos.1 - 1), Dir::Up),
	    }
	},
	Dir::Left => {
	    match ch {
		'R' => (pos, Dir::Up),
		'L' => (pos, Dir::Down),
		_ => ((pos.0 - 1, pos.1), Dir::Left),
	    }
	},
	Dir::Right => {
	    match ch {
		'R' => (pos, Dir::Down),
		'L' => (pos, Dir::Up),
		_ => ((pos.0 + 1, pos.1), Dir::Right),
	    }
	},
	Dir::Down => {
	    match ch {
		'R' => (pos, Dir::Left),
		'L' => (pos, Dir::Right),
		_ => ((pos.0, pos.1 + 1), Dir::Down),
	    }
	},
    }
}

fn walk(
    path: Vec<i128>,
    pos: (i128, i128),
    direction: Dir,
    grid: &HashMap<(i128, i128), i128>,
    seen: &mut HashSet<(i128, i128), Dir>
) -> Option<Vec<i128>> {
    let mut paths = vec![];
    for d in &[82, 76, 1] {
        let new_pos = get_new_pos(pos, direction,  to_char(*d));
        if !seen.insert(new_pos) {
	    continue;
	}
	let pp = if is_scaffold(grid, new_pos.0) {
            // Move ok
            let mut p = path.clone();
	    if let Some(last) = p.last() {
		if *last != 82 && *last != 76 {
		    *last += 1;
		} else {
		    p.push(*d);
		}
	    } else {
		p.push(*d);
	    }
            walk(p, new_pos.0, new_pos.1, grid, seen)
        } else {
	    let scaffold: HashSet<_> = grid.iter().filter(|x| *x.1 == 35).map(|x| *x.0).collect();
	    let visited : HashSet<_> = seen.iter().map(|(p, d)| *p).collect();
	    if scaffold == visited {
		Some(path)
	    } else {
		None
	    }
        };
        if let Some(p) = pp {
            paths.push(p);
        }
    }
    // Return shortest path, if any
    if paths.len() == 0 {
        None
    } else {
        paths.into_iter().min_by(|a, b| a.len().cmp(&b.len()))
    }
}

fn build_grid(program: &Vec<i128>) -> HashMap<(i128, i128), i128> {
    let mut m = intcode::Machine::new(program);
    let mut grid = HashMap::new();
    let mut y = 0;
    let mut x = 0;
    loop {
        if let Some(ch) = m.run_to_next_output() {
            match ch {
                10 => {
                    y += 1;
                    x = 0;
                }
                c => {
                    grid.insert((x, y), c);
                    x += 1;
                }
            }
        } else {
            break;
        }
    }
    grid
}

fn part1(program: &Vec<i128>) -> i128 {
    let mut d = aoc::PrintGridDrawer::new(to_char);
    let grid = build_grid(program);
    d.draw(&grid);
    let align = find_align(&grid);
    let mut g = grid.clone();
    for p in &align {
        g.insert(*p, 'O' as i128);
    }
    d.draw(&g);
    align.iter().map(|(x, y)| x * y).sum()
}

fn part2(program: &Vec<i128>) -> i128 {
    let grid = build_grid(program);
    let robot: Vec<_> = grid.iter().filter(|x| {
	let ch = to_char(*x.1);
	ch == '<' || ch == '>' || ch == '^' || ch == 'v'
    }).map(|x| (*x.0, to_dir(to_char(*x.1)))).collect();
    let mut path = vec![];
    let mut seen = HashSet::new();
    path = walk(path, robot[0].0, robot[0].1, &grid, &mut seen).unwrap();
    println!("path: {:?}", path);
    0
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = aoc::parse_intcode(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    // use super::part1;

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }
}
