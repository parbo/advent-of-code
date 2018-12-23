use std::env;
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

fn walk(a: &Bot, b: &Bot) -> Bot {
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
                p = c;
                break;
            }
            if d < min_d {
                p = c;
                min_d = d;
                continue;
            }
        }
    }
    p
}

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let bots : Vec<Bot> = buffered.lines().filter_map(Result::ok).map(|a| a.parse::<Bot>().unwrap()).collect();
    let max_r = bots.iter().max_by(|a, b| a.radius.cmp(&b.radius)).unwrap();
    let in_range = bots.iter().filter(|c| manhattan(c, max_r) <= max_r.radius).count();
    println!("in range: {}", in_range);
    let origin = Bot{ x: 0, y: 0, z: 0, radius: 1 };
    let mut candidates = vec![];
    for a in &bots {
        let pa = walk(&origin, &a);
        let inside = bots.iter().filter(|c| manhattan(&pa, &c) <= c.radius).count();
        let d = manhattan(&pa, &origin);
        println!("{:?}, {}, {}", pa, inside, d);
        candidates.push((inside, d, pa));
    }
    candidates.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    println!("{:?}", candidates[0]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
