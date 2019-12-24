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

fn draw(g: &HashMap<(i64, i64, i64), char>) {
    let mut level = g.iter().map(|(k, v)| k.0).min().unwrap();
    let mut tot = 0;
    loop {
        let mut bugs = 0;
  //      println!("-- level {} --", level);
        for y in -2..=2 {
            for x in -2..=2{
                if let Some(x) = g.get(&(level, x, y)) {
    //                print!("{}", x);
                    if *x == '#' {
                        bugs += 1;
                    }
                } else {
   //                 print!(".");
                }
            }
 //           println!();
        }
        if bugs == 0 {
            break;
        }
        tot += bugs;
        level += 1;
    }
    println!("total bugs: {}", tot);
}

fn part2(grid: &Vec<Vec<char>>) -> i64 {
    solve(grid, 200)
}

fn solve(grid: &Vec<Vec<char>>, it: i64) -> i64 {
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
        let mut tot_c = 0;
        for ((level, x, y), v) in &g {
            let mut c = 0;
  	    for (nx, ny) in &[(*x + 1, *y), (*x - 1, *y), (*x, *y + 1), (*x, *y - 1)] {
                if *nx == 0 && *ny  == 0 {
                    if *y > 0 && *x == 0 {
                        for x in -2..=2 {
                            let v = new_g.entry((level + 1, x, 2)).or_insert('.');
                            if *v == '#' {
                                c += 1;
                            }
                        }
                    } else if *y < 0 && *x == 0 {
                        for x in -2..=2 {
                           let v = new_g.entry((level + 1, x, -2)).or_insert('.');
                            if *v == '#' {
                                c += 1;
                            }
                        }
                    } else if *y == 0 && *x < 0 {
                        for y in -2..=2 {
                            let v = new_g.entry((level + 1, -2, y)).or_insert('.');
                            if *v == '#' {
                                c += 1;
                            }
                        }
                    } else if *y == 0 && *x > 0 {
                        for y in -2..=2 {
                             let v = new_g.entry((level + 1, 2, y)).or_insert('.');
                            if *v == '#' {
                                c += 1;
                            }
                        }
                    } else {
                        panic!();
                    }
                } else {
                    if *ny > 2 {
                         let v = new_g.entry((*level-1, 0, 1)).or_insert('.');
                        if *v == '#' {
                            c += 1;
                        }
                    } else if *ny < -2 {
                           let v = new_g.entry((*level-1, 0, -1)).or_insert('.');
                        if *v == '#' {
                            c += 1;
                        }
                      
                    } else if *nx > 2 {
                           let v = new_g.entry((*level-1, 1, 0)).or_insert('.');
                        if *v == '#' {
                            c += 1;
                        }
                      
                    } else if *nx < -2 {
                           let v = new_g.entry((*level-1, -1, 0)).or_insert('.');
                        if *v == '#' {
                            c += 1;
                        }
                      
                    } else {
                        let v = new_g.entry((*level, *nx, *ny)).or_insert('.');
                        if *v == '#' {
                            c += 1;
                        }
                    }
                }
            }
            let a = new_g.entry((*level, x % 3, y % 3)).or_insert('.');
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
        mins += 1;
        g = new_g;
        println!("mins: {}", mins);
        draw(&g);
        if mins == it {
            return 0;
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
