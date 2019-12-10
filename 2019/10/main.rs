use aoc;
use std::collections::HashMap;
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
    (m.1).0
}

fn part2(things: &Vec<Vec<char>>) -> i64 {
    0
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
    use super::{parse, part1, part2};

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

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&vec![0]), 0);
    // }
}
