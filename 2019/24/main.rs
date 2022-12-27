use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::*;

type Parsed = Vec<Vec<char>>;

#[allow(clippy::needless_range_loop)]
fn bd(grid: &Parsed) -> i64 {
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
            bd *= 2;
        }
        println!();
    }
    res
}

fn draw(g: &HashMap<(i64, i64, i64), char>) {
    let lev_min = g.iter().map(|(k, _)| k.0).min().unwrap();
    let lev_max = g.iter().map(|(k, _)| k.0).max().unwrap();
    let mut tot = 0;
    for level in lev_min..=lev_max {
        let buggs = g.iter().filter(|(k, v)| k.0 == level && **v == '#').count();
        if buggs == 0 {
            continue;
        }
        let mut bugs = 0;
        println!("-- level {} --", level);
        for y in -2..=2 {
            for x in -2..=2 {
                if let Some(x) = g.get(&(level, x, y)) {
                    print!("{}", x);
                    if *x == '#' {
                        bugs += 1;
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
        tot += bugs;
    }
    println!("total bugs: {}", tot);
}

fn part2(grid: &Parsed) -> i64 {
    solve(grid, 200)
}

fn add_level(g: &mut HashMap<(i64, i64, i64), char>, level: i64) {
    for yy in -2..=2 {
        for xx in -2..=2 {
            if xx == 0 && yy == 0 {
                g.entry((level, xx, yy)).or_insert('?');
            } else {
                g.entry((level, xx, yy)).or_insert('.');
            }
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn solve(grid: &Parsed, it: i64) -> i64 {
    let mut g = HashMap::new();
    let mut yy = -2;
    for y in 0..5 {
        let mut xx = -2;
        for x in 0..5 {
            if xx == 0 && yy == 0 {
                g.insert((0, xx, yy), '?');
            } else {
                g.insert((0, xx, yy), grid[y][x]);
            }
            xx += 1;
        }
        yy += 1;
    }
    let mut mins = 0;
    add_level(&mut g, 1);
    add_level(&mut g, -1);
    draw(&g);
    loop {
        let mut new_g = g.clone();
        let lev_min = g.keys().map(|k| k.0).min().unwrap();
        let lev_max = g.keys().map(|k| k.0).max().unwrap();
        add_level(&mut new_g, lev_max + 1);
        add_level(&mut new_g, lev_min - 1);
        let mut changes = vec![];
        for ((level, x, y), v) in &g {
            if *x == 0 && *y == 0 {
                continue;
            }
            let mut c = 0;
            for (nx, ny) in &[(*x + 1, *y), (*x - 1, *y), (*x, *y + 1), (*x, *y - 1)] {
                if *nx == 0 && *ny == 0 {
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
                } else if *ny > 2 {
                    let v = new_g.entry((*level - 1, 0, 1)).or_insert('.');
                    if *v == '#' {
                        c += 1;
                    }
                } else if *ny < -2 {
                    let v = new_g.entry((*level - 1, 0, -1)).or_insert('.');
                    if *v == '#' {
                        c += 1;
                    }
                } else if *nx > 2 {
                    let v = new_g.entry((*level - 1, 1, 0)).or_insert('.');
                    if *v == '#' {
                        c += 1;
                    }
                } else if *nx < -2 {
                    let v = new_g.entry((*level - 1, -1, 0)).or_insert('.');
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
            if *v == '.' {
                if c == 1 || c == 2 {
                    changes.push(((*level, *x, *y), '#'));
                }
            } else if c != 1 {
                changes.push(((*level, *x, *y), '.'));
            }
        }
        for (k, v) in changes {
            new_g.insert(k, v);
        }
        mins += 1;
        g = new_g;
        if mins == it {
            println!("mins: {}", mins);
            draw(&g);
            return 0;
        }
    }
}

fn part1(grid: &Parsed) -> i64 {
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

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::{parse, solve};

    #[test]
    fn test_part1() {
        let a = vec![
            "....#".to_string(),
            "#..#.".to_string(),
            "#..##".to_string(),
            "..#..".to_string(),
            "#....".to_string(),
        ];
        let p = parse(&a);
        assert_eq!(solve(&p, 10), 0);
    }
}
