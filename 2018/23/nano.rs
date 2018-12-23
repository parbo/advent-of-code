use std::env;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Bot {
    x: i64,
    y: i64,
    z: i64,
    radius: i64
}

impl FromStr for Bot {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(|c| c == '<' || c == ',' || c == '>' || c == '=')
                                 .collect();
        let x = coords[2].parse::<i64>()?;
        let y = coords[3].parse::<i64>()?;
        let z = coords[4].parse::<i64>()?;
        let radius = coords[7].parse::<i64>()?;

        Ok(Bot { x, y, z, radius })
    }
}

fn manhattan(a: &Bot, b: &Bot) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

fn dist(a: &Bot) -> i64 {
    (a.x - 0).abs() + (a.y - 0).abs() + (a.z - 0).abs()
}

fn overlapping(bots: &Vec<Bot>) -> Vec<Bot> {
    let mut overlaps = vec![];
    for i in 0..bots.len() {
        let mut overlap = 0;
        for j in 0..bots.len() {
            if i == j {
                continue;
            }
            let a = bots[i];
            let b = bots[j];
            let mh = manhattan(&a, &b);
            if a.radius + b.radius >= mh {
                overlap += 1;
            }
        }
        overlaps.push(overlap);
    }
    let max_o = overlaps.iter().max().unwrap();
    let candidates : HashSet<usize> = HashSet::from_iter(overlaps.iter().enumerate().filter(|(_, c)| *c == max_o).map(|(n, _)| n));
    let cbots : Vec<Bot> = bots.iter().cloned().enumerate().filter(|(n, _)| candidates.contains(n)).map(|(_, c)| c).collect();
    cbots
}

fn fill(a: &Bot, b: &Bot, pos: (i64, i64, i64), intersections: &mut HashSet<(i64, i64, i64)>) {
    let mut ps = VecDeque::new();
    ps.push_back(pos);
    while let Some(pos) = ps.pop_front() {
        for (dx, dy, dz) in &[(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)] {
            let c = Bot { x: pos.0 + dx, y: pos.1 + dy, z: pos.2 + dz, radius: 1 };
            let p = (c.x, c.y, c.z);
            if intersections.contains(&p) {
                continue;
            }
            let inside_a = manhattan(&c, &a) <= a.radius;
            let inside_b = manhattan(&c, &b) <= b.radius;
            if inside_a && inside_b {
                intersections.insert(p);
                ps.push_back(p);
            }
        }
        if intersections.len() % 1000 == 0 {
            println!("intersections: {}", intersections.len());
        }
    }
}

fn overlap(a: &Bot, b: &Bot) -> HashSet<(i64, i64, i64)> {
    // Walk towards the other
    let mut min_d = manhattan(&a, &b);
    let mut p = *a;
    let mut inside_b = false;
    while !inside_b {
        for (dx, dy, dz) in &[(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)] {
            let c = Bot { x: p.x + dx, y: p.y + dy, z: p.z + dz, radius: 1 };
            let d = manhattan(&c, &b);
            inside_b = d <= b.radius;
            if inside_b {
                break;
            }
            if d < min_d {
                p = c;
                min_d = d;
                continue;
            }
        }
    }
    let mut intersections = HashSet::new();
    fill(a, b, (p.x, p.y, p.z), &mut intersections);
    intersections
}

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let bots : Vec<Bot> = buffered.lines().filter_map(Result::ok).map(|a| a.parse::<Bot>().unwrap()).collect();
    let max_r = bots.iter().max_by(|a, b| a.radius.cmp(&b.radius)).unwrap();
    let in_range = bots.iter().filter(|c| manhattan(c, max_r) <= max_r.radius).count();
    for i in 0..bots.len() {
        for j in 0..bots.len() {
            if i == j {
                continue;
            }
            let a = bots[i];
            let b = bots[j];
            let intersections = overlap(&a, &b);
            println!("intersections: {}", intersections.len());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
