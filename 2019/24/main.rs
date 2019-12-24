use aoc;
// use intcode;
use std::iter::*;
use std::collections::HashSet;
use std::collections::HashMap;

fn bd(grid: &Vec<Vec<char>>) -> i64 {
    let h = grid.len();
    let w = grid[0].len();
    let mut bd = 1;
    let mut res = 0;
    println!("-------------");
    for y in 0..h {
	for x in 0..w {
	    print!("{}", grid[y][x]);
	    if grid[y][x] == '#' {
		res += bd;
	    }
	    bd = bd * 2;
	}
	println!();
    }
    res
}

fn part2(grid: &Vec<Vec<char>>) -> i64 {
    let mut g = HashMap::new();
    let mut yy = -2;
    for y in 0..5 {
        let mut xx = -2;
        for x in 0..5 {
            g.insert((0, xx, yy), grid[y][x]);
            xx += 1;
        }
        yy += 1;
    }
    let mut mins = 0;
    loop {
        let mut new_g = g.clone();
        let mut any_bug = true;

        let mut yy :i64 = 0;
        let mut all_bugs = 0;
        while any_bug {
            let mut xx:i64 = 0;
            while any_bug {

                let mut tot_c = 0;
                for (x, y) in &[(xx, yy), (xx,-yy), (-xx,yy),(-xx,-yy)] {
                    let level : i64 = std::cmp::max((x / 3).abs(), (y /3).abs());
                    let mut c = 0;
  		    for (nx, ny) in &[(*x + 1, *y), (*x - 1, *y), (*x, *y + 1), (*x, *y - 1)] {
                        let nblevel : i64 = std::cmp::max((nx / 3).abs(), (ny /3).abs());
                        if let Some(x) = g.get(&(nblevel, nx % 3, ny % 3)) {
                            if *x == '#' {
                                c += 1;
                            }
                        }
                    }
                    let a = new_g.entry((level, x % 3, y % 3)).or_insert('.');
                    if *a == '.' {
                        if c == 1 || c == 2 {
                            *a = '#';
                        }
                    } else {
                        if c == 1 {
                            *a = '#';
                        }
                    }
                    tot_c += c;
                }
                any_bug = tot_c > 0;
                all_bugs += tot_c;
                xx += 1;
              
            }
            yy += 1;
        }
        mins += 1;
        if mins == 200 {
            return all_bugs;
        }
    }
}

fn part1(grid: &Vec<Vec<char>>) -> i64 {
    let h = grid.len() as i64;
    let w = grid[0].len() as i64;
    let mut g = grid.clone();
    let mut seen = HashSet::new();
    loop {
	let mut new_g = g.clone();
	let bio = bd(&new_g);
	if !seen.insert(bio) {
	    println!("bio: {}", bio);
	    break;
	}
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
		    if c != 1 {
			new_g[y as usize][x as usize] = '.';
		    }
		}
	    }
	}
	g = new_g;
    }
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
