use aoc;
use intcode;
use std::iter::*;

fn part1(numbers: &Vec<i128>) -> i128 {
    let mut m = intcode::Machine::new(&numbers, &vec![1]);
    m.run().unwrap();
    *m.outputs().last().unwrap()
}

fn part2(numbers: &Vec<i128>) -> i128 {
    let mut m = intcode::Machine::new(&numbers, &vec![2]);
    m.run().unwrap();
    *m.outputs().last().unwrap()
}

fn parse(lines: &Vec<String>) -> Vec<i128> {
    let result: Vec<i128> = lines[0]
        .split(|c| c == ',')
        .map(|s| s.trim())
        .map(|v| v.parse::<i128>().unwrap())
        .collect();
    result
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
    // use super::{part1, part2};

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&vec![0]), 0);
    // }
}
