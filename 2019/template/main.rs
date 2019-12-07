use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn part1(things: &Vec<i64>) -> i64 {
    things[0]
}

fn part2(things: &Vec<i64>) -> i64 {
    things[0]
}

fn parse(lines: &Vec<String>) -> Vec<i64> {
    lines
        .iter()
        .map(|x| x.parse::<i64>().unwrap()).
        .collect())
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
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(&vec![0]), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![0]), 0);
    }
}
