use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    iter::*,
};

use aoc::{manhattan, point_add, Grid, GridDrawer, Point, EAST, NORTH, SOUTH, WEST};

// #[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
// #[display("{thing}: {al}-{ah} or {bl}-{bh}")]
// struct Rule {
//     thing: String,
//     al: i64,
//     ah: i64,
//     bl: i64,
//     bh: i64,
// }

type Parsed = (i64, i64, HashMap<Point, char>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
struct State {
    pos: Point,
    minute: i64,
}

fn solve(data: &Parsed, trips: usize) -> i64 {
    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    dbg!(data.0, data.1);
    let min_x = 1;
    let min_y = 1;
    let max_x = data.0 - 2;
    let max_y = data.1 - 2;
    let mut grids = vec![data.2.clone()];
    let mut gg = grids[0].clone();
    gg.insert([min_x - 1, min_y - 1], '#');
    gg.insert([max_x + 1, max_y + 1], '#');
    gd.draw(&gg);
    println!();
    let mut minute = 1;
    loop {
        // println!("{}", grids.len());
        let mut g = HashMap::new();
        for (p, d) in &data.2 {
            let mut pp = match d {
                '>' => point_add(*p, [minute, 0]),
                '<' => point_add(*p, [-minute, 0]),
                '^' => point_add(*p, [0, -minute]),
                'v' => point_add(*p, [0, minute]),
                _ => unreachable!(),
            };
            while pp[0] < min_x {
                pp[0] += max_x - min_x + 1;
            }
            while pp[0] > max_x {
                pp[0] -= max_x - min_x + 1;
            }
            while pp[1] < min_y {
                pp[1] += max_y - min_y + 1;
            }
            while pp[1] > max_y {
                pp[1] -= max_y - min_y + 1;
            }
            let e = g.entry(pp).or_insert('.');
            match *e {
                '.' => *e = *d,
                '<' | '>' | '^' | 'v' => *e = '2',
                x => {
                    *e = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'X']
                        [(x.to_digit(10).unwrap_or(9) + 1).min(10) as usize]
                }
            }
        }
        // let mut gg = g.clone();
        // gg.insert([min_x, min_y], '.');
        // gg.insert([max_x, max_y], '.');
        // gd.draw(&gg);
        // println!();
        if g == grids[0] {
            println!("loop at {}", minute);
            break;
        }
        minute += 1;
        grids.push(g);
    }
    let mut minute = 0;
    let mut path: Vec<State> = vec![];
    for t in 0..trips {
        println!("Starting trip {} at minute {}", t, minute);
        let (start, goal) = if t % 2 == 0 {
            ([1, 0], [max_x, max_y + 1])
        } else {
            ([max_x, max_y + 1], [1, 0])
        };
        let mut frontier = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut gscore = HashMap::new();
        let mut fscore = HashMap::new();
        let state = State { pos: start, minute };
        gscore.insert(state, 0);
        fscore.insert(state, manhattan(start, goal));
        frontier.push(Reverse((manhattan(start, goal), state)));
        let mut res = None;
        while let Some(Reverse((_est, current))) = frontier.pop() {
            if current.pos == goal {
                let mut path = vec![current];
                let mut curr = current;
                while curr != state {
                    curr = came_from[&curr];
                    path.push(curr)
                }
                res = Some((gscore.get(&current).unwrap(), path));
                break;
            }
            let g = *gscore.entry(current).or_insert(i64::MAX);
            let minute = current.minute + 1;
            let grid = &grids[minute as usize % grids.len()];
            for nb in aoc::neighbors(current.pos).chain([current.pos]) {
                // dbg!(nb, grid.get_value(nb));
                if nb == goal
                    || nb == start
                    || (nb[0] >= min_x
                        && nb[0] <= max_x
                        && nb[1] >= min_y
                        && nb[1] <= max_y
                        && grid.get_value(nb).is_none())
                {
                    // dbg!(nb);
                    let ns = State { pos: nb, minute };
                    let new_g = g + 1;
                    let nb_g = gscore.entry(ns).or_insert(i64::MAX);
                    if new_g < *nb_g {
                        came_from.insert(ns, current);
                        *nb_g = new_g;
                        let new_f = new_g + manhattan(goal, ns.pos);
                        *fscore.entry(ns).or_insert(i64::MAX) = new_f;
                        frontier.push(Reverse((new_f, ns)));
                    }
                }
            }
        }
        let res = res.unwrap();
        path.extend(res.1.iter().rev());
        minute += *res.0;
    }
    for s in &path {
        println!("minute: {}", s.minute);
        let mut gg = grids[s.minute as usize % grids.len()].clone();
        gg.insert([min_x - 1, min_y - 1], '#');
        gg.insert([max_x + 1, max_y + 1], '#');
        let r = gg.insert(s.pos, 'E');
        assert!(r.is_none());
        gd.draw(&gg);
        println!();
    }
    path.last().unwrap().minute
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 1)
}

fn part2(data: &Parsed) -> i64 {
    solve(data, 3)
}

fn parse(lines: &[String]) -> Parsed {
    (
        lines[0].len() as i64,
        lines.len() as i64,
        aoc::parse_grid_to_sparse(lines, |c| {
            if ['<', '>', '^', 'v'].contains(&c) {
                Some(c)
            } else {
                None
            }
        }),
    )
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let s = include_str!("example2.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 18);
    }
}
