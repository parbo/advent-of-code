use std::collections::{HashMap, HashSet};
use std::iter::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Program {
    name: String,
    weight: u64,
    supports: Vec<String>,
}

type ParsedItem = Program;
type Parsed = Vec<ParsedItem>;
type Answer = String;

fn part1(data: &Parsed) -> Answer {
    let names: HashSet<String> = data.iter().map(|x| x.name.clone()).collect();
    let supported: HashSet<String> = data.iter().flat_map(|x| x.supports.clone()).collect();
    let unsupported: Vec<&String> = names.difference(&supported).collect();
    unsupported[0].clone()
}

fn get_self_weight(data: &[ParsedItem], name: &str) -> u64 {
    for d in data {
        if d.name == name {
            return d.weight;
        }
    }
    0
}

fn get_weight(data: &[ParsedItem], name: &str) -> u64 {
    for d in data {
        if d.name == name {
            return d.weight + d.supports.iter().map(|n| get_weight(data, n)).sum::<u64>();
        }
    }
    0
}

fn part2(data: &Parsed) -> Answer {
    let mut unbalanced = HashMap::new();
    for d in data {
        let weights: Vec<u64> = d.supports.iter().map(|n| get_weight(data, n)).collect();
        if weights.len() > 1 {
            let mut counts: HashMap<u64, u64> = HashMap::new();
            for w in &weights {
                *counts.entry(*w).or_insert(0) += 1;
            }
            if let Some((w, _c)) = counts.iter().find(|(_w, c)| **c == 1) {
                for (ix, w2) in weights.iter().enumerate() {
                    if w == w2 {
                        unbalanced.insert(d.name.clone(), (d.supports[ix].clone(), counts.clone()));
                    }
                }
            }
        }
    }
    // Find which one to change
    let mut change = None;
    for (s, w) in unbalanced.values() {
        if !unbalanced.contains_key(s) {
            change = Some((s.clone(), w));
            break;
        }
    }
    let change = change.unwrap();
    for d in data {
        if d.name == change.0 {
            if let Some((w, _c)) = change.1.iter().find(|(_w, c)| **c != 1) {
                let w_w_c = get_weight(data, &d.name);
                if *w > w_w_c {
                    return (get_self_weight(data, &d.name) + (*w - w_w_c)).to_string();
                } else {
                    return (get_self_weight(data, &d.name) - (w_w_c - *w)).to_string();
                }
            }
        }
    }
    "".to_string()
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let s = aoc::split_w(x);
            let name = s[0].to_string();
            let weight: u64 = s[1][1..(s[1].len() - 1)].parse().unwrap();
            let supports: Vec<String> = if s.len() > 2 {
                s[3..].iter().map(|x| x.replace(',', "")).collect()
            } else {
                Vec::new()
            };
            Program {
                name,
                weight,
                supports,
            }
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "pbga (66)".into(),
            "xhth (57)".into(),
            "ebii (61)".into(),
            "havc (66)".into(),
            "ktlj (57)".into(),
            "fwft (72) -> ktlj, cntj, xhth".into(),
            "qoyq (66)".into(),
            "padx (45) -> pbga, havc, qoyq".into(),
            "tknk (41) -> ugml, padx, fwft".into(),
            "jptl (61)".into(),
            "ugml (68) -> gyxo, ebii, jptl".into(),
            "gyxo (61)".into(),
            "cntj (57)".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), "tknk".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), "60".to_string());
    }
}
