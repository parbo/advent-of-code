use std::{collections::HashSet, iter::*};

type ParsedItem = String;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn prio(common: char) -> i64 {
    if common as i64 > 90 {
        common as i64 - 96
    } else {
        common as i64 - 65 + 27
    }
}

fn part1(data: &Parsed) -> Answer {
    let mut sum = 0;
    for rs in data {
        let half = rs.len() / 2;
        let first: HashSet<char> = rs[0..half].chars().collect();
        let second: HashSet<char> = rs[half..].chars().collect();
        let common = first.intersection(&second).collect::<Vec<_>>();
        let common = prio(*common[0]);
        sum += common;
    }
    sum
}

fn part2(data: &Parsed) -> Answer {
    let mut sum = 0;
    for grp in data.chunks(3) {
        let a: HashSet<char> = grp[0].chars().collect();
        let b: HashSet<char> = grp[1].chars().collect();
        let c: HashSet<char> = grp[2].chars().collect();
        let common: HashSet<char> = a.intersection(&b).copied().collect();
        let common: Vec<char> = common.intersection(&c).copied().collect();
        sum += prio(common[0]);
    }
    sum
}

fn parse(lines: &[String]) -> Parsed {
    lines.to_vec()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".into(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".into(),
            "PmmdzqPrVvPwwTWBwg".into(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".into(),
            "ttgJtRGJQctTZtZT".into(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 157);
    }
}
