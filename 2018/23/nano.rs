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
struct Point {
    x: i64,
    y: i64,
    z: i64
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

fn fill(a: &Bot, b: &Bot, pos: Point, pred: &Fn(&Bot, &Bot, &Point) -> bool, result: &mut HashSet<Point>) {
    let mut ps = VecDeque::new();
    ps.push_back(pos);
    while let Some(pos) = ps.pop_front() {
        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    let p = Point { x: pos.x + dx, y: pos.y + dy, z: pos.z + dz };
                    if result.contains(&p) {
                        continue;
                    }
                    if pred(a, b, &p) {
                        result.insert(p);
                        ps.push_back(p);
                    }
                }
            }
        }
    }
}

fn find_intersection_point(a: &Bot, b: &Bot) -> Point {
    let mut edge_a = a.p;
    edge_a.x += a.r;
    // Walk towards the intersection
    let mut p = edge_a;
    let db = manhattan(&p, &b.p);
//    println!("db: {}", db);
    if db == b.r {
        return p;
    }
    let mut found = false;
    // println!("a: {:?}", a);
    // println!("b: {:?}", b);
    while !found {
       // println!("p: {:?}", p);
        // Only need to walk in -x and +-z
        for dx in -1..1 {
            for dz in -1..2 {
                let c = Point { x: p.x + dx, y: p.y, z: p.z + dz };
               // println!("c: {:?}", c);
                let da = manhattan(&c, &a.p);
               // println!("da: {}", da);
                if da != a.r {
                    continue;
                }
                p = c;
                let db = manhattan(&c, &b.p);
               // println!("db: {}", db);
                if db == b.r {
                    found = true;
                    break;
                }
            }
        }
    }
    p
}

fn intersect(a: &Bot, b: &Bot) -> HashSet<Point> {
    let mut intersections = HashSet::new();
    let pred = |ba: &Bot, bb: &Bot, pc: &Point| manhattan(&pc, &ba.p) == ba.r && manhattan(&pc, &bb.p) == bb.r;
    let p = find_intersection_point(a, b);
    fill(a, b, p, &pred, &mut intersections);
    intersections
}

fn draw(i: i64, bots: &[Bot], scale: i64, from: i64, to: i64) -> (i64, Point) {
    println!("draw: {}, {}, {}, {}", i, scale, from, to);
    // let name = format!("nano_x_{}.ppm", scale);
    // let path = Path::new(&name);
    // let mut file = File::create(&path).unwrap();
    let min_a = from;
    let min_b = from;
    let w = (to - from) / scale;
    let h = (to - from) / scale;
    println!("w: {}, h: {}", w, h);
    // let header = format!("P6 {} {} 255\n", w, h);
    // let mut data = vec![];
    // data.extend(header.as_bytes());
    let mut max_in = 0;
    let mut max_a = -1;
    let mut max_b = -1;
    for a in 0..h {
        for b in 0..w {
            let mut inside = 0;
            for bot in bots {
                let p = match i {
                    0 => Point { x: bot.p.x, y: a * scale + min_a, z: b * scale + min_b },
                    1 => Point { x: a * scale + min_a, y: bot.p.y, z: b * scale + min_b },
                    2 => Point { x: a * scale + min_a, y: b * scale + min_b, z: bot.p.z },
                    _ => panic!()
                };
                if manhattan(&p, &bot.p) <= bot.r {
                    inside += 1;
                }
            }
            // let shade = (255.0 * (inside as f64) / (bots.len() as f64)) as u8;
            // // if shade != 0 {
            // //     println!("shade: {}", shade);
            // // }
            // data.push(shade);
            // data.push(shade);
            // data.push(shade);
            if inside > max_in {
                max_in = inside;
                max_a = a;
                max_b = b;
            }
        }
    }
    let p = match i {
        0 => Point { x: 0, y: max_a * scale + min_a, z: max_b * scale + min_b },
        1 => Point { x: max_a * scale + min_a, y: 0, z: max_b * scale + min_b },
        2 => Point { x: max_a * scale + min_a, y: max_b * scale + min_b, z: 0 },
        _ => panic!()
    };
    println!("in: {}, {:?}", max_in, p);
    // file.write(&data).unwrap();
    (max_in, p)
}

fn brute(bots: &[Bot], scale: i64) {
    let sbots : Vec<Bot> = bots.iter().map(|c| Bot { p: Point { x: c.p.x / scale, y: c.p.y / scale, z: c.p.z / scale }, r: c.r / scale }).collect();
    let min_x = sbots.iter().map(|a| a.p.x - a.r).min().unwrap();
    let max_x = sbots.iter().map(|a| a.p.x + a.r).max().unwrap();
    let min_y = sbots.iter().map(|a| a.p.y - a.r).min().unwrap();
    let max_y = sbots.iter().map(|a| a.p.y + a.r).max().unwrap();
    let min_z = sbots.iter().map(|a| a.p.z - a.r).min().unwrap();
    let max_z = sbots.iter().map(|a| a.p.z + a.r).max().unwrap();
    let mut insides = vec![];
    for x in min_x..(max_x+1) {
        for y in min_y..(max_y+1) {
            for z in min_z..(max_z+1) {
                let mut inside = 0;
                for b in &sbots {
                    let p = Point { x: x * scale, y: y * scale, z: z * scale };
                    if manhattan(&p, &b.p) <= b.r {
                        inside += 1;
                    }
                }
                insides.push((inside, Point { x, y, z }));
            }
        }
    }
    let max_i = insides.iter().map(|c| c.0).max().unwrap();
    let c : Vec<Point> = insides.iter().filter(|c| c.0 == max_i).map(|c| c.1).collect();
    println!("found: {} / {}, max_i: {}", c.len(), insides.len(), max_i);
}

fn foo(bots: &[Bot]) -> Vec<Bot> {
    if bots.len() == 0 {
        return vec![];
    }
    let mut ret = vec![bots[0]];
    let mut overlapping = vec![];
    let a = bots[0];
    for j in 1..bots.len() {
        let b = bots[j];
        let d = manhattan(&a.p, &b.p);
        if d <= (a.r + b.r) {
            overlapping.push(b);
        }
    }
    ret.extend(foo(&overlapping));
    ret
}

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let bots : Vec<Bot> = buffered.lines().filter_map(Result::ok).map(|a| a.parse::<Bot>().unwrap()).collect();
    let max_r = bots.iter().max_by(|a, b| a.r.cmp(&b.r)).unwrap();
    let in_range = bots.iter().filter(|c| manhattan(&c.p, &max_r.p) <= max_r.r).count();
    println!("in range: {}", in_range);
    // brute(&bots, 897454);
    let mut min_x :i64 = bots.iter().map(|a| a.p.x - a.r).min().unwrap();
    let mut max_x : i64 = bots.iter().map(|a| a.p.x + a.r).max().unwrap();
    let mut min_y : i64 = bots.iter().map(|a| a.p.y - a.r).min().unwrap();
    let mut max_y : i64 = bots.iter().map(|a| a.p.y + a.r).max().unwrap();
    let mut min_z : i64 = bots.iter().map(|a| a.p.z - a.r).min().unwrap();
    let mut max_z : i64 = bots.iter().map(|a| a.p.z + a.r).max().unwrap();
    let xscale = (max_x - min_x) / 500;
    let yscale = (max_y - min_y) / 500;
    let zscale = (max_z - min_z) / 500;
    let (inx, px) = draw(0, &bots, xscale, min_x, max_x);
    let (iny, py) = draw(1, &bots, yscale, min_y, max_y);
    let (inz, pz) = draw(2, &bots, zscale, min_z, max_z);
    min_x = std::cmp::min(py.x, pz.x);
    max_x = std::cmp::max(py.x, pz.x);
    min_y = std::cmp::min(px.y, pz.y);
    max_y = std::cmp::max(px.y, pz.y);
    min_z = std::cmp::min(py.z, px.z);
    max_z = std::cmp::max(py.z, px.z);
    let mut points = vec![];
    let origin = Point { x: 0, y: 0, z: 0 };
    for x in min_x..max_x {
        if x % 100000 == 0 {
            println!("{} / {}, {}", x, min_x, max_x);
        }
        let p = Point { x: x, y: px.y, z: px.z };
        let mut inside = 0;
        for b in &bots {
            let d = manhattan(&p, &b.p);
            if d <= b.r {
                inside += 1;
            }
        }
        let dist = manhattan(&p, &origin);
        points.push((inside, dist, p));
    }
    for y in min_y..max_y {
        if y % 100000 == 0 {
            println!("{} / {}, {}", y, min_y, max_y);
        }
        let p = Point { x: py.x, y: y, z: px.z };
        let mut inside = 0;
        for b in &bots {
            let d = manhattan(&p, &b.p);
            if d <= b.r {
                inside += 1;
            }
        }
        let dist = manhattan(&p, &origin);
        points.push((inside, dist, p));
    }
    for z in min_z..max_z {
        if z % 100000 == 0 {
            println!("{} / {}, {}", z, min_z, max_z);
        }
        let p = Point { x: pz.x, y: pz.y, z: z };
        let mut inside = 0;
        for b in &bots {
            let d = manhattan(&p, &b.p);
            if d <= b.r {
                inside += 1;
            }
        }
        let dist = manhattan(&p, &origin);
        points.push((inside, dist, p));
    }
    println!("{}", points.len());
    points.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.0.cmp(&b.0)));
    println!("{:?}", points[0]);
    // Walk from here towards origin until the count drops
    let point = points[0];
    let mut p = point.2;
    let mut last_p = p;
    let mut min_d = point.1;
    let c = point.0;
    loop {
        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    let new_p = Point { x: p.x + dx, y: p.y + dy, z: p.z + dz };
                    let new_d = manhattan(&new_p, &origin);
                    if new_d < min_d {
                        min_d = new_d;
                        p = new_p;
                    }
                }
            }
        }
        let mut inside = 0;
        for b in &bots {
            let d = manhattan(&p, &b.p);
            if d <= b.r {
                inside += 1;
            }
        }
        if inside < c {
            break;
        }
        last_p = p;
    }
    println!("p: {:?}, last_p: {:?}, min_d: {:?}", p, last_p, min_d);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
