use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    iter::*,
};

use aoc::{manhattan, point_add, Grid, Point};

type Parsed = (i64, i64, HashMap<Point, char>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
struct State {
    pos: Point,
    minute: i64,
}

fn solve(data: &Parsed, trips: usize) -> i64 {
    let min_x = 1;
    let min_y = 1;
    let max_x = data.0 - 2;
    let max_y = data.1 - 2;
    let mut grids = vec![data.2.clone()];
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
        if g == grids[0] {
            break;
        }
        minute += 1;
        grids.push(g);
    }
    let mut minute = 0;
    let mut path: Vec<State> = vec![];
    for t in 0..trips {
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
                if nb == goal
                    || nb == start
                    || (nb[0] >= min_x
                        && nb[0] <= max_x
                        && nb[1] >= min_y
                        && nb[1] <= max_y
                        && grid.get_value(nb).is_none())
                {
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
    #[cfg(feature = "vis")]
    {
        use aoc::GridDrawer;
        let mut sprites = HashMap::new();
        let b = [140, 140, 255];
        for (c, col) in [
            ('<', b),
            ('>', b),
            ('^', b),
            ('v', b),
            ('2', [150, 50, 255]),
            ('3', [160, 60, 255]),
            ('4', [170, 70, 255]),
            ('5', [180, 80, 255]),
            ('6', [190, 90, 255]),
            ('7', [200, 100, 255]),
            ('8', [210, 110, 255]),
            ('9', [220, 120, 255]),
            ('X', [230, 130, 255]),
            ('E', [0, 255, 0]),
            ('#', [255, 255, 255]),
        ] {
            let mut g = vec![vec![[0, 0, 0]; 8]; 8];
            g.text(&c.to_string(), [0, 0], col);
            let mut s = vec![];
            for y in 0..8 {
                for x in 0..8 {
                    s.push(g[y][x]);
                }
            }
            sprites.insert(c, s);
        }
        let name = if trips == 1 {
            "vis/24/part1"
        } else {
            "vis/24/part2"
        };
        let mut drawer =
            aoc::BitmapSpriteGridDrawer::new((8, 8), |c| sprites.get(&c).unwrap().clone(), name);
        drawer.set_bg([0, 0, 0]);
        for s in &path {
            let mut gg = grids[s.minute as usize % grids.len()].clone();
            for x in min_x - 1..=max_x + 1 {
                gg.insert([x, 0], '#');
                gg.insert([x, max_y + 1], '#');
            }
            for y in min_y - 1..=max_y + 1 {
                gg.insert([0, y], '#');
                gg.insert([max_x + 1, y], '#');
            }
            gg.remove(&[1, 0]);
            gg.remove(&[max_x, max_y + 1]);
            let r = gg.insert(s.pos, 'E');
            assert!(r.is_none());
            drawer.draw(&gg);
        }
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
