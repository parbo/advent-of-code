use std::collections::BTreeSet;

use aoc::Itertools;

type Parsed = Vec<(String, String)>;

fn clusters(data: &Parsed, max_len: usize) -> BTreeSet<BTreeSet<String>> {
    let mut conns: aoc::FxHashMap<String, aoc::FxHashSet<String>> = aoc::FxHashMap::default();
    for d in data {
        conns.entry(d.0.clone()).or_default().insert(d.1.clone());
        conns.entry(d.1.clone()).or_default().insert(d.0.clone());
    }
    let mut chains = BTreeSet::new();
    let mut todo: Vec<_> = conns.keys().map(|x| BTreeSet::from([x.clone()])).collect();
    let mut seen = aoc::FxHashSet::default();
    while let Some(chain) = todo.pop() {
        if chain.len() > max_len {
            continue;
        }
        let c = chain.last().unwrap();
        if let Some(comps) = conns.get(c) {
            for c in comps {
                let cc = conns.get(c).unwrap();
                if chain.iter().all(|x| cc.contains(x)) {
                    let mut nc = chain.clone();
                    nc.insert(c.clone());
                    if seen.insert(nc.clone()) {
                        todo.push(nc);
                    }
                } else {
                    chains.insert(chain.clone());
                }
            }
        }
    }
    chains
}

fn part1(data: &Parsed) -> i64 {
    clusters(data, 3)
        .iter()
        .filter(|x| x.len() == 3)
        .filter(|x| x.iter().any(|x| x.starts_with("t")))
        .count() as i64
}

fn part2(data: &Parsed) -> i64 {
    let pw = clusters(data, usize::MAX)
        .iter()
        .max_by_key(|x| x.len())
        .unwrap()
        .iter()
        .join(",");
    println!("pw: {}", pw);
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let p = aoc::split_ch(x, '-');
            (p[0].to_string(), p[1].to_string())
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
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 7);
    }
}
