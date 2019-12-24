use aoc;
// use intcode;
use std::iter::*;
use std::collections::HashSet;

fn bd(grid: &Vec<Vec<char>>) -> i64 {
    let h = grid.len();
    let w = grid[0].len();
    let mut bd = 1;
    let mut res = 0;
    for y in 0..h {
	for x in 0..w {
	    if grid[y][x] == '#' {
		res += bd;
	    }
	    bd = bd * 2;
	}
    }
    res
}

fn part1(grid: &Vec<Vec<char>>) -> i64 {
    let h = grid.len() as i64;
    let w = grid[0].len() as i64;
    let mut g = grid.clone();
    let mut seen = HashSet::new();
    loop {
	let mut new_g = g.clone();
	for y in 0..h {
	    for x in 0..w {
		if g[y as usize][x as usize] == '.' {
		    let mut c = 0;
		    for (nx, ny) in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
			if *nx >= w || *ny >= h || *ny < 0 || *nx < 0 {
			    continue;
			}
			if g[*ny as usize][*nx as usize] == '#' {
			    c += 1;
			}
		    }
		    if c == 1 || c == 2 {
			new_g[y as usize][x as usize] = '#';
		    }
		} else {
		    let mut c = 0;
		    for (nx, ny) in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
			if *nx >= w || *ny >= h || *ny < 0 || *nx < 0 {
			    continue;
			}
			if g[*ny as usize][*nx as usize] == '#' {
			    c += 1;
			}
		    }
		    if c == 1 {
			new_g[y as usize][x as usize] = '.';
		    }
		}
	    }
	}
	let bio = bd(&new_g);
	if !seen.insert(bio) {
	    println!("bio: {}", bio);
	    break;
	}
	g = new_g;
    }
    0
}

fn part2(_: &Vec<Vec<char>>) -> i64 {
    0
}

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    //let parsed = aoc::parse_intcode(&lines);
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
    // use super::part1;

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }
}
