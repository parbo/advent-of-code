use std::{collections::HashMap, iter::*};

use aoc::{point_add, Point};

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

fn tetris(
    grid: &mut HashMap<Point, char>,
    data: &Parsed,
    mut c: usize,
    start: usize,
    end: usize,
) -> (usize, i64) {
    let rock1 = ((4, 1), vec![[0, 0], [1, 0], [2, 0], [3, 0]]);
    let rock2 = ((3, 3), vec![[1, 0], [0, 1], [1, 1], [2, 1], [1, 2]]);
    let rock3 = ((3, 3), vec![[2, 0], [2, 1], [0, 2], [1, 2], [2, 2]]);
    let rock4 = ((1, 4), vec![[0, 0], [0, 1], [0, 2], [0, 3]]);
    let rock5 = ((2, 2), vec![[0, 0], [1, 0], [0, 1], [1, 1]]);
    let rocks: Vec<((i64, i64), Vec<Point>)> = vec![rock1, rock2, rock3, rock4, rock5];

    let mut miny = 0;

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

    let blit = |p: Point, r: &[Point], g: &mut HashMap<Point, char>, c: char, miny: &mut i64| {
        for rp in r {
            let pp = point_add(*rp, p);
            g.insert(pp, c);
            let my = *miny;
            *miny = my.min(pp[1]);
        }
    };

    for i in start..end {
        // println!("===== {} ======", i);
        let ((_w, h), rock) = &rocks[i % rocks.len()];
        let y = grid.keys().map(|p| p[1]).min().unwrap();
        miny = miny.min(y);
        let mut p = [2, y - 3 - h];
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
                blit(p, rock, grid, '#', &mut miny);
                break;
            }
        }
    }
    (c, miny)
}

fn part1(data: &Parsed) -> Answer {
    let mut g = make_grid();
    tetris(&mut g, data, 0, 0, 2022);
    -g.keys().map(|p| p[1]).min().unwrap()
}

fn part2(data: &Parsed) -> Answer {
    // Find the looping pattern
    let mut g = make_grid();
    let mut c = 0;
    let mut rounds = 0;
    let mut rows = vec![];
    let mut rounds_vec = vec![];
    let mut last_y = 0;
    let (offs, h) = 'outer: loop {
        // println!("c: {}", c);
        let (cc, miny) = tetris(&mut g, data, c, rounds, rounds + 1);
        c = cc;
        rounds += 1;

        for y in (miny..last_y).rev() {
            let mut v = 0;
            for x in 0..7 {
                if *g.get(&[x, y]).unwrap_or(&' ') == '#' {
                    v |= 1 << x;
                }
            }
            rows.push(v);
            rounds_vec.push(rounds - 1);
        }
        last_y = miny;

        // println!("==== rows ====");
        // gd.draw(&rows);
        // println!("---- grid ----");
        // gd.draw(&g);

        if rows.len() % 1000 == 0 {
            println!("{}", rows.len());
        }

        // Start checking for cycles after data.len() * 5 cycles
        let threshold = data.len() * 5;
        for offs in threshold..(rows.len() / 2) {
            let h = (rows.len() - offs) / 2;
            if h == 0 {
                break;
            }
            if rows[offs..(offs + h)] == rows[(offs + h)..(offs + h + h)] {
                break 'outer (offs as i64, h as i64);
            }
        }
    };
    let left = 1000000000000 - rounds_vec[offs as usize];
    let lp = rounds_vec[(offs + h) as usize] - rounds_vec[offs as usize];
    let loops = left / lp;
    let rem = left % lp;
    let yy = g.keys().map(|p| p[1]).min().unwrap();
    tetris(&mut g, data, c, rounds, rounds + rem);
    let remh = yy - g.keys().map(|p| p[1]).min().unwrap();
    g.keys().map(|p| p[1]).min().unwrap();
    println!(
        "offs: {}, h: {}, loops: {}, lp: {}, rounds: {}, rem: {}, remh: {}",
        offs, h, loops, lp, rounds, rem, remh
    );
    offs + loops as i64 * h + remh
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

    #[test]
    fn test_part2() {
        let res = part2(&parse(&example()));
        let expected = 1514285714288;
        println!("{}", res - expected);
        assert_eq!(res, expected);
    }
}
