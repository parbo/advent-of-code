use std::collections::{HashMap, HashSet};
use std::iter::*;
use std::time::Instant;

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("{depth}: {range}")]
struct Scanner {
    depth: i64,
    range: i64,
}

type ParsedItem = Scanner;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(firewall: &[ParsedItem]) -> Answer {
    let ranges: HashMap<i64, i64> = firewall.iter().map(|x| (x.depth, x.range)).collect();
    let mut state = HashMap::new();
    let layers = firewall.iter().map(|x| x.depth).max().unwrap();
    let mut severity = 0;
    for layer in 0..=layers {
        // Check if caught
        if state.entry(layer).or_insert((0, 1)).0 == 0 {
            if let Some(range) = ranges.get(&layer) {
                severity += layer * range;
            }
        }
        // Update scanners
        for scanner in firewall {
            let (pos, dir) = state.entry(scanner.depth).or_insert((0, 1));
            #[allow(clippy::collapsible_else_if)]
            if *dir > 0 {
                if *pos + 1 == scanner.range {
                    *dir = -1;
                }
            } else {
                if *pos == 0 {
                    *dir = 1;
                }
            }
            *pos += *dir;
        }
    }
    severity
}

fn part2(firewall: &[ParsedItem]) -> Answer {
    let ranges: HashMap<i64, i64> = firewall.iter().map(|x| (x.depth, x.range)).collect();
    let mut state = HashMap::new();
    let layers = firewall.iter().map(|x| x.depth).max().unwrap();
    let mut picosecond = 0;
    let mut failed = HashSet::new();
    loop {
        for delay in (picosecond - layers)..=picosecond {
            if failed.contains(&delay) {
                continue;
            }
            // Check if caught
            let layer = picosecond - delay;
            assert!(layer >= 0 && layer <= layers);
            if state.entry(layer).or_insert((0, 1)).0 == 0 {
                if let Some(_range) = ranges.get(&layer) {
                    failed.insert(delay);
                }
            }
        }
        // Update scanners
        for scanner in firewall {
            let (pos, dir) = state.entry(scanner.depth).or_insert((0, 1));
            #[allow(clippy::collapsible_else_if)]
            if *dir > 0 {
                if *pos + 1 == scanner.range {
                    *dir = -1;
                }
            } else {
                if *pos == 0 {
                    *dir = 1;
                }
            }
            *pos += *dir;
        }
        // Check if it succeded
        let delay = picosecond - layers;
        if delay >= 0 && !failed.contains(&delay) {
            return delay;
        }
        picosecond += 1;
    }
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let start_time = Instant::now();
    let (part, lines) = aoc::read_lines();
    let io_time = Instant::now();
    let parsed = parse(&lines);
    let parse_time = Instant::now();
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    let done_time = Instant::now();
    println!(
        "read: {:?}, parse: {:?}, solve: {:?}\n",
        io_time.duration_since(start_time),
        parse_time.duration_since(io_time),
        done_time.duration_since(parse_time)
    );
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!["0: 3".into(), "1: 2".into(), "4: 4".into(), "6: 4".into()]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 10);
    }
}
