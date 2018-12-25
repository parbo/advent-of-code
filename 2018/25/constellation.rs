use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Star {
    a: i64,
    b: i64,
    c: i64,
    d: i64
}

impl FromStr for Star {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        let a = coords[0].trim().parse::<i64>()?;
        let b = coords[1].trim().parse::<i64>()?;
        let c = coords[2].trim().parse::<i64>()?;
        let d = coords[3].trim().parse::<i64>()?;
        Ok(Star { a, b, c, d })
    }
}

fn manhattan(a: &Star, b: &Star) -> i64 {
    (a.a - b.a).abs() + (a.b - b.b).abs() + (a.c - b.c).abs() + (a.d - b.d).abs()
}

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let stars : Vec<Star> = buffered.lines().filter_map(Result::ok).map(|a| a.parse::<Star>().unwrap()).collect();
    let mut constellations = vec![];
    let mut grouped : HashSet<Star> = HashSet::new();
    for i in 0..stars.len() {
        let si = stars[i];
        if grouped.contains(&si) {
            continue;
        }
        let mut group = vec![si];
        grouped.insert(si);
        loop {
            let mut any_grouped = false;
            for j in i..stars.len() {
                let sj = stars[j];
                if grouped.contains(&sj) {
                    continue;
                }
                let mut in_group = false;
                for sg in &group {
                    let d = manhattan(&sj, &sg);
                    if d <= 3 {
                        in_group = true;
                        break;
                    }
                }
                if in_group {
                    group.push(sj);
                    grouped.insert(sj);
                    any_grouped = true;
                }
            }
            if !any_grouped {
                break;
            }
        }
        constellations.push(group);
    }
    for c in &constellations {
        println!("{:?}", c);
    }
    println!("constellations: {}", constellations.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
