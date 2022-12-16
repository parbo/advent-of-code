use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
    iter::*,
};

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    rate: i64,
    tunnels: Vec<String>,
}

type ParsedItem = Valve;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    pos: String,
    opened: BTreeMap<String, i64>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn walk(pos: &str, scan: &HashMap<String, Valve>, minute: i64) -> i64 {
    let mut frontier = BinaryHeap::new();
    let mut visited: HashSet<State> = HashSet::new();
    frontier.push((
        0,
        State {
            pos: pos.to_string(),
            opened: BTreeMap::new(),
        },
        minute,
    ));
    let mut gscore: HashMap<State, i64> = HashMap::new();
    while let Some((_score, state, minute)) = frontier.pop() {
        if visited.contains(&state) {
            continue;
        }
        println!("{}, {:?}, {}", _score, state, minute);
        if minute == 30 {
            let score: i64 = state
                .opened
                .iter()
                .map(|(v, t)| (31 - t) * scan.get(v).unwrap().rate)
                .sum();
            println!("score: {}, {}", score, _score);
            if score > _score {
                println!("{:?}", state);
            }
            return score;
        }
        let v = scan.get(&state.pos).unwrap();
        // dbg!(v);
        for t in &v.tunnels {
            for x in 0..2 {
                if x == 1 && v.rate == 0 {
                    // Can't open
                    continue;
                }
                let mut o = state.opened.clone();
                let mut tl = minute;
                // Should/can we open?
                if x == 1 && !o.contains_key(&v.name) {
                    o.insert(state.pos.clone(), tl);
                    tl += 1;
                }
                if tl > 30 {
                    continue;
                }
                let ns = State {
                    pos: t.clone(),
                    opened: o.clone(),
                };
                let gs =
                    gscore.get(&state).unwrap_or(&0) + (31 - (tl + 1)) * scan.get(t).unwrap().rate;
                let e = gscore.entry(ns.clone()).or_default();
                if gs > *e {
                    *e = gs;
                    // Overestimate score by opening all remaining valves now
                    let h = scan
                        .keys()
                        .filter(|vvv| !o.contains_key(*vvv))
                        .map(|v| (31 - (tl + 1)) * scan.get(v).unwrap().rate)
                        .sum::<i64>();
                    let fscore = gs + h;
                    //                println!("next: {:?}", next);
                    if !visited.contains(&ns) {
                        let next = (fscore, ns, tl + 1);
                        frontier.push(next);
                    }
                }
            }
        }
        visited.insert(state);
    }
    panic!();
}

fn part1(data: &Parsed) -> Answer {
    let mut scan = HashMap::new();
    for v in data {
        scan.insert(v.name.clone(), v.clone());
    }
    walk("AA", &scan, 1)
}

fn part2(_: &Parsed) -> Answer {
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let p = aoc::split_ch(x, ';');
            let pp = aoc::split_w(p[0]);
            let name = pp[1].to_string();
            let rate = pp[4][5..].parse::<i64>().unwrap();
            let tunnels = aoc::split_ch(&p[1][22..], ',')
                .iter()
                .map(|x| x.to_string())
                .collect();
            Valve {
                name,
                rate,
                tunnels,
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
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 1651);
    }
}
