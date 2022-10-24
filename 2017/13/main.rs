use std::collections::HashMap;
use std::iter::*;

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("{depth}: {range}")]
struct Scanner {
    depth: i64,
    range: i64,
}

type ParsedItem = Scanner;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(firewall: &Parsed) -> Answer {
    let ranges: HashMap<i64, i64> = firewall.iter().map(|x| (x.depth, x.range)).collect();
    let layers = firewall.iter().map(|x| x.depth).max().unwrap();
    let ranges: Vec<i64> = (0..=layers)
        .map(|x| *ranges.get(&x).unwrap_or(&0))
        .collect();
    (0..=layers)
        .map(|ps| {
            let range = ranges[ps as usize];
            if range > 0 && (ps % (2 * (range - 1))) == 0 {
                ps * range
            } else {
                0
            }
        })
        .sum()
}

fn part2(firewall: &Parsed) -> Answer {
    let ranges: HashMap<i64, i64> = firewall.iter().map(|x| (x.depth, x.range)).collect();
    let layers = firewall.iter().map(|x| x.depth).max().unwrap();
    let ranges: Vec<i64> = (0..=layers)
        .map(|x| *ranges.get(&x).unwrap_or(&0))
        .collect();
    let mut delay = 0;
    while (delay..=(delay + layers)).any(|ps| {
        let range = ranges[(ps - delay) as usize];
        range > 0 && (ps % (2 * (range - 1))) == 0
    }) {
        delay += 1;
    }
    delay
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
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
