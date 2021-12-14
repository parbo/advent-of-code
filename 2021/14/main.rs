use std::collections::HashMap;
use std::iter::*;
use std::time::Instant;

#[derive(Debug)]
struct Polymer {
    template: Vec<char>,
    rules: HashMap<(char, char), char>,
}

type Parsed = Polymer;
type Answer = i64;

fn solve(polymer: &Parsed, gen: usize) -> Answer {
    let mut pairs = HashMap::new();
    let end = polymer.template.len() - 1;
    for i in 0..end {
	let p = (polymer.template[i], polymer.template[i + 1]);
        *pairs.entry(p).or_insert(0) += 1;
    }
    for _ in 0..gen {
	let mut new_p = pairs.clone();
	for (p, num) in pairs {
            if let Some(c) = polymer.rules.get(&p) {
		*new_p.entry(p).or_insert(0) -= num;
		let p1 = (p.0, *c);
		let p2 = (*c, p.1);
		*new_p.entry(p1).or_insert(0) += num;
		*new_p.entry(p2).or_insert(0) += num;
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
