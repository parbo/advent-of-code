use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    iter::*,
};

use aoc::{FxHashSet, Grid, Point};

type Shape = Vec<Vec<char>>;
type SGrid = HashMap<Point, char>;
type Parsed = (Vec<Shape>, Vec<(i64, i64, Vec<usize>)>);

fn possible_grids(w: i64, h: i64, g: &SGrid, shapes: &[Shape]) -> Vec<SGrid> {
    let mut possible = vec![];
    for y in 0..h {
        for x in 0..w {
            // try to place shape at (x, y)
            for shape in shapes {
                let mut gg = g.clone();
                let mut ok = true;
                for p in shape.points() {
                    let gx = x + p[0];
                    let gy = y + p[1];
                    if gx >= w || gy >= h {
                        break;
                    }
                    if let Some('#') = shape.get_value(p) {
                        let gv = gg.get_value([gx, gy]);
                        if gv == Some('#') {
                            ok = false;
                            break;
                        }
                        gg.set_value([gx, gy], '#');
                    }
                }
                if ok {
                    possible.push(gg);
                }
            }
        }
    }
    possible
}

fn possible_shapes(shape: &Shape) -> Vec<Shape> {
    let mut shapes = vec![];
    shapes.push(shape.clone());
    let mut s = shape.clone();
    for _ in 0..4 {
        s.rotate_90_cw();
        shapes.push(s.clone());
    }
    let mut s = shape.clone();
    s.flip_horizontal();
    shapes.push(s.clone());
    for _ in 0..4 {
        s.rotate_90_cw();
        shapes.push(s.clone());
    }
    shapes
}

#[derive(Eq, PartialEq, Clone)]
struct State {
    score: usize,
    grid: SGrid,
    needed_shapes: Vec<usize>,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.score.hash(state);
        for (k, v) in &self.grid {
            k.hash(state);
            v.hash(state);
        }
        for n in &self.needed_shapes {
            n.hash(state);
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn can_fit(w: i64, h: i64, shapes: &[Vec<Shape>], needed_shapes: &[usize]) -> bool {
    let g = HashMap::new();
    let sum = needed_shapes.iter().sum::<usize>();
    let mut todo = BinaryHeap::new();
    todo.push(Reverse(State {
        score: sum,
        grid: g,
        needed_shapes: needed_shapes.to_vec(),
    }));
    let mut seen = FxHashSet::default();
    // let mut gd = aoc::PrintGridDrawer::new(|c| c);
    while let Some(Reverse(s)) = todo.pop() {
        if s.score == 0 {
            // gd.draw(&s.grid);
            return true;
        }
        for (shape_ix, num) in s.needed_shapes.iter().enumerate() {
            if *num == 0 {
                continue;
            }
            let poss_g = possible_grids(w, h, &s.grid, &shapes[shape_ix]);
            for gg in poss_g {
                let mut new_rem = s.needed_shapes.clone();
                new_rem[shape_ix] -= 1;
                let new_s = State {
                    score: new_rem.iter().sum(),
                    grid: gg,
                    needed_shapes: new_rem,
                };
                if seen.insert(new_s.clone()) {
                    todo.push(Reverse(new_s));
                }
            }
        }
    }
    false
}

fn part1(data: &Parsed) -> i64 {
    println!("{:?}", data);
    let mut possible_shapes_vec = vec![];
    for shape in &data.0 {
        possible_shapes_vec.push(possible_shapes(shape));
    }
    let mut num = 0;
    for (w, h, indices) in &data.1 {
        let tot = indices
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let shape = &data.0[i];
                let mut cnt = 0;
                for r in shape {
                    for c in r {
                        if *c == '#' {
                            cnt += 1;
                        }
                    }
                }
                (n * cnt) as i64
            })
            .sum::<i64>();
        if tot > w * h {
            continue;
        }
        let area = indices
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let shape = &data.0[i];
                let ([minx, miny], [maxx, maxy]) = shape.extents();
                let area = (maxx - minx + 1) * (maxy - miny + 1);
                *n as i64 * area
            })
            .sum::<i64>();
        if area <= w * h || can_fit(*w, *h, &possible_shapes_vec, indices) {
            num += 1;
        }
    }
    num
}

fn part2(_: &Parsed) -> i64 {
    0
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    let mut shapes = vec![];
    let mut areas = vec![];
    for p in parts {
        if p[0].ends_with(':') {
            shapes.push(aoc::parse_grid(&p[1..]));
        } else {
            for r in p {
                let pp = aoc::split_ch(r, ':');
                let ppp = aoc::split_ch(pp[0], 'x');
                let indices: Vec<usize> = aoc::things(pp[1]);
                areas.push((
                    ppp[0].parse::<i64>().unwrap(),
                    ppp[1].parse::<i64>().unwrap(),
                    indices,
                ));
            }
        }
    }
    (shapes, areas)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn example() -> Vec<String> {
    //     let s = include_str!("example.txt");
    //     s.lines().map(|x| x.to_string()).collect()
    // }

    // fn example() -> Vec<String> {
    // 	   vec![
    //         "0".into()
    //     ]
    // }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&parse(&example())), 0);
    // }
}
