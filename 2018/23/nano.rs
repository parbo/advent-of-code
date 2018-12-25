use std::env;
use std::cmp::Ordering;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Bot {
    p: Point,
    r: i64
}

impl FromStr for Bot {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(|c| c == '<' || c == ',' || c == '>' || c == '=')
                                 .collect();
        let x = coords[2].parse::<i64>()?;
        let y = coords[3].parse::<i64>()?;
        let z = coords[4].parse::<i64>()?;
        let r = coords[7].parse::<i64>()?;
        let p = Point {x, y, z};

        Ok(Bot { p, r })
    }
}

fn manhattan(a: &Point, b: &Point) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

fn overlap(a: &Bot, b: &Bot) -> bool {
    if b.r == 1 {
        manhattan(&a.p, &b.p) <= a.r
    } else {
        manhattan(&a.p, &b.p) <= a.r + 3 * b.r
    }
}

fn overlapping(bots: &[Bot], bot: &Bot) -> i64 {
    let mut c = 0;
    for b in bots {
        let overlaps = overlap(b, bot);
        if overlaps {
            c += 1;
        }
    }
    c
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Thing {
    overlaps: i64,
    bot: Bot
}

impl Ord for Thing {
    fn cmp(&self, other: &Thing) -> Ordering {
        self.overlaps.cmp(&other.overlaps)
            .then_with(|| other.bot.r.cmp(&self.bot.r))
            .then_with(|| self.bot.p.cmp(&other.bot.p))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Thing {
    fn partial_cmp(&self, other: &Thing) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let bots : Vec<Bot> = buffered.lines().filter_map(Result::ok).map(|a| a.parse::<Bot>().unwrap()).collect();
    let max_r = bots.iter().max_by(|a, b| a.r.cmp(&b.r)).unwrap();
    let in_range = bots.iter().filter(|c| manhattan(&c.p, &max_r.p) <= max_r.r).count();
    println!("in range: {}", in_range);

    let max_x = bots.iter().map(|a| (a.p.x.abs() + a.r)).max().unwrap();
    let max_y = bots.iter().map(|a| (a.p.y.abs() + a.r)).max().unwrap();
    let max_z = bots.iter().map(|a| (a.p.z.abs() + a.r)).max().unwrap();
    let extents = vec![max_x, max_y, max_z];
    let extent = (extents.iter().cloned().max().unwrap() as u64).checked_next_power_of_two().unwrap() as i64;
    let radius = extent;

    let initial = Bot { p: Point { x: 0, y: 0, z: 0}, r: radius };
    let mut q = BinaryHeap::new();
    q.push(Thing { overlaps: overlapping(&bots, &initial), bot: initial });
    let mut cnt = 0;
    let mut max_ovl = 0;
    let mut pts = vec![];
    let origin = Point { x: 0, y: 0, z: 0 };
    while let Some(t) = q.pop() {
        cnt += 1;
//        if cnt % 10000 == 0 {
            println!("cnt: {}, ovl: {}, {:?}", cnt, t.overlaps, t.bot);
//        }
        if t.bot.r == 1 {
            if t.overlaps < max_ovl {
                break;
            }
            max_ovl = t.overlaps;
            println!("answer: {}, {}", t.overlaps, manhattan(&t.bot.p, &origin));
            pts.push(t);
            continue;
        }
        // Subdivide
        let radius = t.bot.r / 2;
        for x in 0..2 {
            let new_x = t.bot.p.x + radius * x;
            for y in 0..2 {
                let new_y = t.bot.p.y + radius * y;
                for z in 0..2 {
                    let new_z = t.bot.p.z + radius * z;
                    let p = Point { x: new_x, y: new_y,  z: new_z };
                    let new_b = Bot { p: p, r: radius };
                    let o = overlapping(&bots, &new_b);
                    if o > 0 {
                        q.push(Thing { overlaps: o, bot: new_b });
                    }
                }
            }
        }
    }
    let answer = pts.iter().map(|p| manhattan(&origin, &p.bot.p)).min().unwrap();
    println!("answer: {}", answer);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
