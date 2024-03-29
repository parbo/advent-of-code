use std::{collections::BTreeSet, iter::*};

use aoc::{FxHashMap, FxHashSet};

type Parsed = FxHashMap<String, Vec<String>>;

fn connected<'a>(g: &'a Parsed, from: &'a str) -> BTreeSet<&'a str> {
    let mut c = BTreeSet::new();
    for (k, v) in g {
        if k == from {
            for vv in v {
                c.insert(vv.as_str());
            }
        } else {
            for vv in v {
                if vv == from {
                    c.insert(k);
                }
            }
        }
    }
    c
}

fn reachable<'a>(
    g: &'a Parsed,
    from: &'a str,
    removed: &[(&'a str, &'a str)],
) -> BTreeSet<&'a str> {
    let mut r = BTreeSet::default();
    let mut todo = vec![from];
    let mut seen = FxHashSet::default();
    while let Some(f) = todo.pop() {
        r.insert(f);
        let conn = connected(g, f);
        for vv in conn {
            if removed.contains(&(f, vv)) || removed.contains(&(vv, f)) {
                continue;
            }
            if seen.insert(vv) {
                todo.push(vv);
            }
        }
    }
    r
}

fn nodes(g: &Parsed) -> BTreeSet<&str> {
    let mut c = BTreeSet::new();
    for (k, v) in g {
        c.insert(k.as_str());
        for vv in v {
            c.insert(vv.as_str());
        }
    }
    c
}

fn solve(data: &Parsed, rr: &[(&str, &str)]) -> Option<i64> {
    let mut n = nodes(data);
    let num = n.len();
    let mut groups = BTreeSet::new();
    while let Some(k) = n.pop_last() {
        let r = reachable(data, k, rr);
        if r.len() == num {
            return None;
        }
        n = n.difference(&r).cloned().collect::<BTreeSet<&str>>();
        groups.insert(r);
        if groups.len() > 2 {
            break;
        }
    }
    if groups.len() == 2 {
        Some(groups.iter().map(|g| g.len()).product::<usize>() as i64)
    } else {
        None
    }
}

fn part1(data: &Parsed) -> i64 {
    // Identified by running graphviz neato engine on input
    let rr = vec![("fvm", "ccp"), ("lhg", "llm"), ("thx", "frl")];
    solve(data, &rr).unwrap()
}

fn part2(_: &Parsed) -> i64 {
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let (a, b) = x.split_once(':').unwrap();
            let b = aoc::split_w(b);
            (a.into(), b.iter().map(|&x| x.into()).collect())
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::Itertools;

    fn to_remove(data: &Parsed) -> Vec<Vec<(&str, &str)>> {
        let n = nodes(data);
        let pairs: Vec<Vec<&str>> = n.into_iter().combinations(2).collect();
        let pairs: Vec<Vec<&str>> = pairs
            .into_iter()
            .filter(|x| {
                let empty = vec![];
                data.get(x[0]).unwrap_or(&empty).contains(&x[1].to_string())
                    || data.get(x[1]).unwrap_or(&empty).contains(&x[0].to_string())
            })
            .collect();
        pairs
            .iter()
            .combinations(3)
            .map(|x| x.iter().map(|x| (x[0], x[1])).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    fn example() -> Vec<String> {
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        let data = parse(&example());
        let rr = to_remove(&data);
        let mut solutions = vec![];
        for r in &rr {
            if let Some(x) = solve(&data, r) {
                solutions.push(x);
            }
        }
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], 54);
    }
}
