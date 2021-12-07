use std::{collections::HashMap, iter::*};

type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(crabs: &[ParsedItem]) -> Answer {
    let s = *crabs.iter().min().unwrap();
    let e = *crabs.iter().max().unwrap();
    let mut cost = HashMap::new();
    for i in s..e {
        let mut fuel = 0;
        for c in crabs {
            fuel += (c - i).abs();
        }
        cost.insert(i, fuel);
    }
    *cost.iter().min_by_key(|(_k, v)| **v).unwrap().1
}

fn part2(crabs: &[ParsedItem]) -> Answer {
    let s = *crabs.iter().min().unwrap();
    let e = *crabs.iter().max().unwrap();
    let mut cost = HashMap::new();
    for i in s..e {
        let mut fuel = 0;
        for c in crabs {
            fuel += (1..((c - i).abs() + 1)).sum::<i64>();
        }
        cost.insert(i, fuel);
    }
    *cost.iter().min_by_key(|(_k, v)| **v).unwrap().1
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_ch(&lines[0], ',')
        .iter()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 168);
    }
}
