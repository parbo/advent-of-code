use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::*;
use std::path::Path;

struct Data {
    things: HashSet<String>,
    orbits: HashMap<String, Vec<String>>,
    thing_orbits: HashMap<String, String>,
}

fn tree(obj: &str, end: &str, orbits: &HashMap<String, Vec<String>>, depth: i64) -> i64 {
    if obj == end {
        return depth;
    }
    if let Some(v) = orbits.get(obj) {
        for t in v {
            let x = tree(t, end, orbits, depth + 1);
            if x != 0 {
                return x;
            }
        }
    }
    return 0;
}

fn part1(data: &Data) -> i64 {
    let mut v = 0;
    for thing in data.things.iter() {
        let t = tree("COM", &thing, &data.orbits, 0);
        println!("{}, {:?}", thing, t);
        v += t;
    }
    v
}

fn part2(data: &Data) -> i64 {
    let mut dists = HashMap::new();
    for thing in data.things.iter() {
        let d1 = tree(thing, "YOU", &data.orbits, 0);
        let d2 = tree(thing, "SAN", &data.orbits, 0);
        if d1 > 0 && d2 > 0 {
            println!("common: {}, {}, {}", thing, d1, d2);
            dists.insert(thing, (d1, d2));
        }
    }
    let mut s = data.things.len() as i64;
    let mut st = None;
    for (thing, d) in dists {
        let dist = d.0 + d.1;
        if dist < s {
            println!("{} -> {}", thing, dist);
            s = dist;
            st = Some(thing);
        }
    }
    println!("Min: {:?} -> {}", st, s);
    s - 2
}

fn parse_lines(lines: &Vec<String>) -> Data {
    let mut things: HashSet<String> = HashSet::new();
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
    let mut thing_orbits: HashMap<String, String> = HashMap::new();
    for line in lines {
        let orbit = line
            .split(')')
            .map(|x| x.trim().to_string())
            .collect_tuple::<(String, String)>()
            .unwrap();
        things.insert(orbit.0.clone());
        things.insert(orbit.1.clone());
        thing_orbits.insert(orbit.1.clone(), orbit.0.clone());
        orbits.entry(orbit.0).or_insert(Vec::new()).push(orbit.1);
    }
    Data {
        things,
        orbits,
        thing_orbits,
    }
}

fn input(path: &Path) -> Data {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines: Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    parse_lines(&lines)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();
    let filename = &args[2];

    let parsed = input(Path::new(&filename));

    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::{parse_lines, part1, part2};

    #[test]
    fn test_part1() {
        let lines: Vec<String> = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        let d = parse_lines(&lines);
        assert_eq!(part1(&d), 42);
    }

    #[test]
    fn test_part2() {
        let lines: Vec<String> = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        let d = parse_lines(&lines);
        assert_eq!(part2(&d), 4);
    }
}
