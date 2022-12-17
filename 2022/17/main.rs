use std::{collections::HashMap, iter::*};

use aoc::{point_add, GridDrawer, Point};

type ParsedItem = char;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn make_grid() -> HashMap<Point, char> {
    // Put in a floor
    let grid: HashMap<Point, char> = [
        ([0, 0], '-'),
        ([1, 0], '-'),
        ([2, 0], '-'),
        ([3, 0], '-'),
        ([4, 0], '-'),
        ([5, 0], '-'),
        ([6, 0], '-'),
    ]
    .into_iter()
    .collect();
    grid
}

fn tetris(grid: &mut HashMap<Point, char>, data: &Parsed, rounds: usize) {
    let rock1 = ((4, 1), vec![[0, 0], [1, 0], [2, 0], [3, 0]]);
    let rock2 = ((3, 3), vec![[1, 0], [0, 1], [1, 1], [2, 1], [1, 2]]);
    let rock3 = ((3, 3), vec![[2, 0], [2, 1], [0, 2], [1, 2], [2, 2]]);
    let rock4 = ((1, 4), vec![[0, 0], [0, 1], [0, 2], [0, 3]]);
    let rock5 = ((2, 2), vec![[0, 0], [1, 0], [0, 1], [1, 1]]);
    let rocks: Vec<((i64, i64), Vec<Point>)> = vec![rock1, rock2, rock3, rock4, rock5];

    let collides = |p: Point, r: &[Point], g: &HashMap<Point, char>| {
        for rp in r {
            let pp = point_add(*rp, p);
            if pp[0] < 0 || pp[0] > 6 {
                return true;
            }
            if g.contains_key(&pp) {
                return true;
            }
        }
        false
    };

    let blit = |p: Point, r: &[Point], g: &mut HashMap<Point, char>, c: char| {
        for rp in r {
            let pp = point_add(*rp, p);
            g.insert(pp, c);
        }
    };

    let mut c = 0;
    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    for i in 0..rounds {
        // println!("===== {} ======", i);
        let ((_w, h), rock) = &rocks[i % rocks.len()];
        let mut p = [2, grid.keys().map(|p| p[1]).min().unwrap() - 3 - h];
        loop {
            // let mut g = grid.clone();
            // blit(p, rock, &mut g, '@');
            // gd.draw(&g);
            // println!();
            // Jet stream
            // println!("{}", data[c % data.len()]);
            let newp = match data[c % data.len()] {
                '>' => [p[0] + 1, p[1]],
                '<' => [p[0] - 1, p[1]],
                _ => unreachable!(),
            };
            c += 1;
            if !collides(newp, rock, grid) {
                p = newp;
            }
            // let mut g = grid.clone();
            // blit(p, rock, &mut g, '@');
            // gd.draw(&g);
            // println!();
            // Drop
            let newp = [p[0], p[1] + 1];
            if !collides(newp, rock, grid) {
                p = newp;
            } else {
                blit(p, rock, grid, '#');
                break;
            }
        }
    }

    // gd.draw(&grid);
}

fn part1(data: &Parsed) -> Answer {
    let mut g = make_grid();
    tetris(&mut g, data, 2022);
    g.keys().map(|p| p[1]).max().unwrap() - g.keys().map(|p| p[1]).min().unwrap()
}

fn part2(data: &Parsed) -> Answer {
    // println!("{}, {}", data.len(), data.len() * 5);
    // let loops = 1000000000000 / (data.len() * 5);
    // let rem = 1000000000000 % (data.len() * 5);
    // let h = tetris(data, data.len() * 5);
    // loops as i64 * h + tetris(data, rem)
    // g.keys().map(|p| p[1]).max().unwrap() - g.keys().map(|p| p[1]).min().unwrap()
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines[0].chars().collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".into()]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 3068);
    }
}
