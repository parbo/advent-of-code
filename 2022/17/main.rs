use std::{
    collections::{BTreeMap, HashMap},
    iter::*,
};

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

fn tetris(grid: &mut HashMap<Point, char>, data: &Parsed, mut c: usize, rounds: usize) -> usize {
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

    // let mut gd = aoc::PrintGridDrawer::new(|c| c);
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
    c
}

fn part1(data: &Parsed) -> Answer {
    let mut g = make_grid();
    tetris(&mut g, data, 0, 2022);
    -g.keys().map(|p| p[1]).min().unwrap()
}

fn part2(data: &Parsed) -> Answer {
    // Find the looping pattern
    let mut rounds = 5i64;
    let mut c = 0;
    let mut g = make_grid();
    let mut rems = vec![];
    let mut heights = vec![];
    let c = 'outer: loop {
        c = tetris(&mut g, data, c, 5);
        let y = g.keys().map(|p| p[1]).min().unwrap();
        // println!("{}, {}, {}", rounds, c, c % data.len());
        let rem = c % data.len();
        rems.push(rem);
        heights.push(y);

        for offs in 0..rems.len() {
            let w = (rems.len() - offs) / 2;
            if w == 0 {
                break;
            }
            if rems[offs..(offs + w)] == rems[(offs + w)..(offs + w + w)] {
                println!(
                    "Found loop at {} {}, {:?}, {}, {}",
                    y,
                    rounds,
                    rems,
                    heights[offs],
                    heights[offs + w],
                );
                break 'outer (
                    5 * offs as i64,
                    heights[offs + w] - heights[offs],
                    rems[offs],
                    heights[offs],
                );
            }
        }

        rounds += 5;
    };
    let initial = c.0;
    let loop_length = rounds - initial;
    let loop_height = c.1;
    let init_h = c.3;
    let c = c.2;

    let mut first_loop = (rounds, init_h);
    let mut last = (0..7)
        .map(|x| *g.get(&[x, init_h]).unwrap_or(&' '))
        .collect::<Vec<_>>();
    let lh = loop {
        let nc = tetris(&mut g, data, c, loop_length as usize);
        assert_eq!(nc % data.len(), c);
        rounds += loop_length;
        let y = g.keys().map(|p| p[1]).min().unwrap();
        let top = (0..7)
            .map(|x| *g.get(&[x, y]).unwrap_or(&' '))
            .collect::<Vec<_>>();
        if top == last {
            // loop found!
            println!("found a loop at {}, {}, {:?} ({})", rounds, y, top, g.len());
            break first_loop.1 - y;
        } else {
            println!(
                "not a loop at {}, {}, {:?}, {:?} ({})",
                rounds,
                y,
                top,
                last,
                g.len()
            );
        }
        last = top;
        first_loop = (rounds, y);
    };
    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    gd.draw(&g);
    println!(
        "found loop: {}, {}, {}, {}",
        first_loop.0, first_loop.1, rounds, lh
    );
    let lp = rounds - first_loop.0;
    let left = 1000000000000 - rounds;
    let loops = left / loop_length;
    let rem = left % loop_length;
    println!("loops: {}, rem: {}, lp: {}", loops, rem, lp);
    let yy = g.keys().map(|p| p[1]).min().unwrap();
    tetris(&mut g, data, c, rem as usize);
    let remh = yy - g.keys().map(|p| p[1]).min().unwrap();
    println!(
        "{}, {}, {}, {}",
        first_loop.0 - first_loop.1,
        loops * lh,
        lh,
        remh
    );
    println!("lh: {}, init_h: {}", lh, first_loop.0);
    (first_loop.0 - first_loop.1) + loops * lh + remh
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
