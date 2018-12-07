use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64
}

fn solve(path: &Path) -> i64 {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut coords = vec![];
    for line in lines {
        let mut t = line.split(|c| [',', ' '].contains(&c)).filter(|s| s.len() > 0).map(|v| v.parse::<i64>().unwrap());
        let c = Coord { x: t.next().unwrap(), y: t.next().unwrap() };
        coords.push(c);
    }
    let min_x = coords.iter().map(|c| c.x).min().unwrap();
    let min_y = coords.iter().map(|c| c.y).min().unwrap();
    let max_x = coords.iter().map(|c| c.x).max().unwrap();
    let max_y = coords.iter().map(|c| c.y).max().unwrap();
    let w = max_x - min_x + 1;
    let h = max_y - min_y + 1;
    let mut v = vec![];
    v.resize((w * h) as usize, 0);
    for y in min_y..(max_y+1) {
        for x in min_x..(max_x+1) {
            let mut min_mh_dist = max_x + max_y;
            let mut min_i : i64 = -1;
            for i in 0..coords.len() {
                let c = &coords[i];
                let mh_dist = (c.x - x).abs() + (c.y - y).abs();
                if mh_dist < min_mh_dist {
                    min_mh_dist = mh_dist;
                    min_i = i as i64;
                }
            }
            v[((y - min_y) * w + (x - min_x)) as usize] = min_i;
        }
    }
    // Find edge touchers
    let mut e = HashSet::new();
    for i in 0..w {
        e.insert(v[i as usize]);
        let x = (((h - 1) * w) + i) as usize;
        e.insert(v[x]);
    }
    for i in 0..h {
        e.insert(v[(i * w) as usize]);
        e.insert(v[(i * w + (w - 1)) as usize]);
    }
    let mut max_a = 0;
    let mut max_i = -1;
    for i in 0..coords.len() {
        let x = i as i64;
        if e.contains(&x) {
            continue;
        }
        let a = v.iter().filter(|&c| *c == x).count();
        if a > max_a {
            max_a = a;
            max_i = x;
        }
    }
    println!("{:?}", e);
    return max_a as i64;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{}", result);
}
