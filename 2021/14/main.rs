use std::collections::HashMap;
use std::iter::*;


#[derive(Debug)]
struct Polymer {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

type Parsed = Polymer;
type Answer = i64;

fn solve(polymer: &Parsed, gen: usize) -> Answer {
    let mut pairs = HashMap::new();
    for w in polymer.template.windows(2) {
        *pairs.entry((w[0], w[1])).or_insert(0) += 1;
    }
    for _ in 0..gen {
        let mut new_p = pairs.clone();
        for (p, num) in pairs {
            if let Some(c) = polymer.rules.get(&p) {
                *new_p.entry(p).or_insert(0) -= num;
                *new_p.entry((p.0, *c)).or_insert(0) += num;
                *new_p.entry((*c, p.1)).or_insert(0) += num;
            }
        }
        pairs = new_p;
    }
    let mut counts = HashMap::new();
    for (p, num) in pairs {
        *counts.entry(p.0).or_insert(0) += num;
    }
    // Also count the last letter
    *counts.entry(*polymer.template.last().unwrap()).or_insert(0) += 1;
    let max = counts.iter().map(|(_, num)| num).max().unwrap();
    let min = counts.iter().map(|(_, num)| num).min().unwrap();
    max - min
}

fn part1(polymer: &Parsed) -> Answer {
    solve(polymer, 10)
}

fn part2(polymer: &Parsed) -> Answer {
    solve(polymer, 40)
}

fn parse(lines: &[String]) -> Parsed {
    let sections = aoc::split_by_empty_line(lines);
    let template = sections[0][0].chars().collect();
    let rules = sections[1]
        .iter()
        .map(|x| aoc::split_str(x, "->"))
        .map(|x| {
            let mut c = x[0].chars();
            (
                (c.next().unwrap(), c.next().unwrap()),
                x[1].chars().next().unwrap(),
            )
        })
        .collect();
    Polymer { template, rules }
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "NNCB".into(),
            "".into(),
            "CH -> B".into(),
            "HH -> N".into(),
            "CB -> H".into(),
            "NH -> C".into(),
            "HB -> C".into(),
            "HC -> B".into(),
            "HN -> C".into(),
            "NN -> C".into(),
            "BH -> H".into(),
            "NC -> B".into(),
            "NB -> B".into(),
            "BN -> B".into(),
            "BB -> N".into(),
            "BC -> B".into(),
            "CC -> N".into(),
            "CN -> C".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 1588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 2188189693529);
    }
}
