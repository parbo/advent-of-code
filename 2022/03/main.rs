use aoc::AsciiSet;
use std::iter::*;

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
    data.iter()
        .map(|rs| {
            let half = rs.len() / 2;
            let first: AsciiSet = rs[0..half].chars().collect();
            let second: AsciiSet = rs[half..].chars().collect();
            prio(first.intersection(&second).next().unwrap())
        })
        .sum()
}

fn part2(data: &Parsed) -> Answer {
    data.chunks(3)
        .map(|grp| {
            let a: AsciiSet = grp[0].chars().collect();
            let b: AsciiSet = grp[1].chars().collect();
            let c: AsciiSet = grp[2].chars().collect();
            prio((a & b & c).iter().next().unwrap())
        })
        .sum()
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
