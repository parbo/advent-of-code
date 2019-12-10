use aoc;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::*;

fn part1(things: &Vec<Vec<char>>) -> i64 {
    let h = things.len() as i64;
    let w = things[0].len() as i64;
    let mut seen = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            if things[y as usize][x as usize] != '#' {
                continue;
            }
            let mut t = things.clone();
            t[y as usize][x as usize] = 'o';
            for dy in -h..=h {
                for dx in -w..=w {
                    let mut s = false;
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    for i in 1..std::i64::MAX {
                        let yy = y + i * dy;
                        let xx = x + i * dx;
                        if xx < 0 || xx >= w || yy < 0 || yy >= h {
                            break;
                        }
                        if t[yy as usize][xx as usize] == '#' {
                            if s {
                                t[yy as usize][xx as usize] = '*';
                            } else {
                                s = true;
                            }
                        }
                    }
                }
            }
            let mut c = 0;
            for yy in 0..h {
                for xx in 0..w {
                    if t[yy as usize][xx as usize] == '#' {
                        c += 1;
                    }
                }
            }
            seen.insert((x, y), (c, t));
        }
    }
    // for m in &seen {
    //     println!("{:?}, {:?}", m.0, (m.1).0);
    //     for tt in &(m.1).1 {
    //         println!("{:?}", tt);
    //     }
    // }
    let m = seen.iter().max_by(|a, b| (a.1).0.cmp(&(b.1).0)).unwrap();
    println!("{:?}", m.0);
    (m.1).0
}

fn ratio(a: i64, b: i64) -> f64 {
    (a as f64) / (b as f64)
}

fn quadrant(dx: i64, dy: i64) -> i64 {
    if dx >= 0 && dy < 0 {
        1
    } else if dx >= 0 && dy >= 0 {
        2
    } else if dx < 0 && dy >= 0 {
        3
    } else {
        4
    }
}

fn compare(dx: i64, dy: i64) -> f64 {
    let q = quadrant(dx, dy);
    match q {
        1 => ratio(dx, -dy),
        2 => ratio(dy, dx),
        3 => ratio(-dx, dy),
        _ => ratio(-dy, -dx),
    }
}

fn seen(things: &Vec<Vec<char>>, x: i64, y: i64) -> HashSet<(i64, i64)> {
    let h = things.len() as i64;
    let w = things[0].len() as i64;
    let mut seen = HashSet::new();
            let mut t = things.clone();
            t[y as usize][x as usize] = 'o';
            for dy in -h..=h {
                for dx in -w..=w {
                    let mut s = false;
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    for i in 1..std::i64::MAX {
                        let yy = y + i * dy;
                        let xx = x + i * dx;
                        if xx < 0 || xx >= w || yy < 0 || yy >= h {
                            break;
                        }
                        if t[yy as usize][xx as usize] == '#' {
                            if s {
                                t[yy as usize][xx as usize] = '*';
                            } else {
                                s = true;
                            }
                        }
                    }
                }
            }
            for yy in 0..h {
                for xx in 0..w {
                    if t[yy as usize][xx as usize] == '#' {
                        seen.insert((xx, yy));
                    }
                }
    }
    seen
}

fn targets(things: &Vec<Vec<char>>, x: i64, y: i64) -> Vec<(i64, i64)> {
    let h = things.len() as i64;
    let w = things[0].len() as i64;
    let mut t = things.clone();
    t[y as usize][x as usize] = 'o';
    let mut asteroids = vec![];
    for yy in 0..h {
        for xx in 0..w {
            if t[yy as usize][xx as usize] != '#' {
                continue;
            }
            let dx = xx - x;
            let dy = yy - y;
            asteroids.push((dx, dy));
        }
    }
    asteroids
}

fn candidates(t: &Vec<Vec<char>>, x: i64, y: i64) -> Vec<(i64, i64)> {
    let mut seen_set = seen(&t, x, y);
    let mut asteroids : Vec<_> = seen_set.into_iter().map(|(xx, yy)| (xx - x, yy - y)).collect();
    // Sort clockwise
    asteroids.sort_by(|a, b| {
        quadrant(a.0, a.1)
            .cmp(&quadrant(b.0, b.1))
            .then(compare(a.0, a.1).partial_cmp(&compare(b.0, b.1)).unwrap())
    });
   println!("{:?}", asteroids);
    asteroids
}

fn solve_part2(things: &Vec<Vec<char>>, x: i64, y: i64) -> i64 {
    let h = things.len() as i64;
    let w = things[0].len() as i64;
    let mut t = things.clone();
    t[y as usize][x as usize] = 'o';
    let mut cand = candidates(&t, x, y);
    let mut c = 1;
    loop {
//        println!("round {}", i);
        for (dx, dy) in &cand {
            let yy = y + dy;
            let xx = x + dx;
            if t[yy as usize][xx as usize] == '#' {
                println!("vaporizing {}: {}, {} | {}", c, xx, yy, quadrant(*dx, *dy));
                t[yy as usize][xx as usize] = c.to_string().chars().next().unwrap(); //'*';
                //xif c == 2 { return -1;}
                if c == 200 {
                    // for tt in &t {
                    //     println!("{:?}", tt);
                    // }
                    println!("{}, {}", xx, yy);
                    return 100 * xx + yy;
                }
                c = c + 1;
            } else {
		panic!();
	    }
        }
        cand = candidates(&t, x, y);

    }
}

fn part2(things: &Vec<Vec<char>>) -> i64 {
    solve_part2(things, 20, 20)
}

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.into_iter().map(|x| x.chars().collect()).collect()
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
    use super::{parse, part1, solve_part2};

    #[test]
    fn test_part1_1() {
        let m = parse(&vec![
            ".#..#".to_string(),
            ".....".to_string(),
            "#####".to_string(),
            "....#".to_string(),
            "...##".to_string(),
        ]);
        assert_eq!(part1(&m), 8);
    }

    #[test]
    fn test_part1_2() {
        let m = parse(&vec![
            "......#.#.".to_string(),
            "#..#.#....".to_string(),
            "..#######.".to_string(),
            ".#.#.###..".to_string(),
            ".#..#.....".to_string(),
            "..#....#.#".to_string(),
            "#..#....#.".to_string(),
            ".##.#..###".to_string(),
            "##...#..#.".to_string(),
            ".#....####".to_string(),
        ]);
        assert_eq!(part1(&m), 33);
    }

    #[test]
    fn test_part1_3() {
        let m = parse(&vec![
            "#.#...#.#.".to_string(),
            ".###....#.".to_string(),
            ".#....#...".to_string(),
            "##.#.#.#.#".to_string(),
            "....#.#.#.".to_string(),
            ".##..###.#".to_string(),
            "..#...##..".to_string(),
            "..##....##".to_string(),
            "......#...".to_string(),
            ".####.###.".to_string(),
        ]);
        assert_eq!(part1(&m), 35);
    }

    #[test]
    fn test_part2() {
        let m = parse(&vec![
            ".#..##.###...#######".to_string(),
            "##.############..##.".to_string(),
            ".#.######.########.#".to_string(),
            ".###.#######.####.#.".to_string(),
            "#####.##.#.##.###.##".to_string(),
            "..#####..#.#########".to_string(),
            "####################".to_string(),
            "#.####....###.#.#.##".to_string(),
            "##.#################".to_string(),
            "#####.##.###..####..".to_string(),
            "..######..##.#######".to_string(),
            "####.##.####...##..#".to_string(),
            ".#####..#.######.###".to_string(),
            "##...#.##########...".to_string(),
            "#.##########.#######".to_string(),
            ".####.#.###.###.#.##".to_string(),
            "....##.##.###..#####".to_string(),
            ".#.#.###########.###".to_string(),
            "#.#.#.#####.####.###".to_string(),
            "###.##.####.##.#..##".to_string(),
        ]);
        assert_eq!(solve_part2(&m, 11, 13), 8 * 100 + 2);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&vec![0]), 0);
    // }
}
