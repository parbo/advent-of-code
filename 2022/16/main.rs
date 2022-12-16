use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
    iter::*,
};

use aoc::UnGraphMap;

#[derive(Debug, Clone)]
struct Valve {
    name: u16,
    rate: i64,
    tunnels: Vec<u16>,
}

type ParsedItem = Valve;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    pos: u16,
    opened: BTreeMap<u16, i64>,
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

fn walk(
    pos: u16,
    scan: &aoc::FxHashMap<u16, Valve>,
    _paths: aoc::FxHashMap<(u16, u16), i64>,
    minute: i64,
) -> i64 {
    let mut frontier = BinaryHeap::new();
    frontier.push((
        0,
        State {
            pos,
            opened: BTreeMap::new(),
        },
        minute,
    ));
    let mut visited = HashSet::new();
    let mut gscore: HashMap<State, i64> = HashMap::new();
    let mut best = 0;
    while let Some((escore, state, minute)) = frontier.pop() {
        // println!("{}, {:?}, {}", escore, state, minute);
        if escore < best {
            break;
        }
        if minute == 30 {
            let score: i64 = state
                .opened
                .iter()
                .map(|(v, t)| (30 - t) * scan.get(v).unwrap().rate)
                .sum();
            // if score > best {
            // println!("{}, {:?}", score, state);
            // }
            best = best.max(score);
            gscore.insert(state.clone(), score);
            continue;
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
                let score: i64 = o
                    .iter()
                    .map(|(v, t)| (30 - t) * scan.get(v).unwrap().rate)
                    .sum();
                let e: i64 = scan
                    .keys()
                    .filter(|x| !o.contains_key(*x))
                    .map(|v| (30 - (tl + 1)) * scan.get(v).unwrap().rate)
                    .sum();
                let ns = State { pos: *t, opened: o };
                if !gscore.contains_key(&ns) && !visited.contains(&(tl + 1, ns.clone())) {
                    visited.insert((tl + 1, ns.clone()));
                    let next = (score + e, ns, tl + 1);
                    // println!("next: {:?}, {}, {}", next, score, e);
                    frontier.push(next);
                }
            }
        }
    }
    dbg!(gscore.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap());
    best
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State2 {
    posa: u16,
    posb: u16,
    opened: BTreeMap<u16, i64>,
}

impl Ord for State2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.posa.cmp(&other.posa).then(self.posb.cmp(&other.posb))
    }
}

impl PartialOrd for State2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn walk2(
    pos: u16,
    scan: &aoc::FxHashMap<u16, Valve>,
    paths: aoc::FxHashMap<(u16, u16), i64>,
    minute: i64,
) -> i64 {
    let mut frontier = BinaryHeap::new();
    frontier.push((
        0,
        State2 {
            posa: pos,
            posb: pos,
            opened: BTreeMap::new(),
        },
        minute,
    ));
    let mut visited = HashSet::new();
    let mut gscore: HashMap<State2, i64> = HashMap::new();
    let mut best = 0;
    while let Some((escore, state, minute)) = frontier.pop() {
        if frontier.len() % 10000 == 0 {
            println!("queue size: {}", frontier.len());
            println!("{}, {:?}, {}", escore, state, minute);
        }
        if escore < best {
            break;
        }
        if minute == 26 {
            let score: i64 = state
                .opened
                .iter()
                .map(|(v, t)| (26 - t) * scan.get(v).unwrap().rate)
                .sum();
            if score > best {
                println!("{}, {:?}", score, state);
            }
            best = best.max(score);
            gscore.insert(state.clone(), score);
            continue;
        }
        let va = scan.get(&state.posa).unwrap();
        let vb = scan.get(&state.posb).unwrap();
        for (ta, oa) in va
            .tunnels
            .iter()
            .map(|x| (x, false))
            .chain([(&va.name, true)])
        {
            for (tb, ob) in vb
                .tunnels
                .iter()
                .map(|x| (x, false))
                .chain([(&vb.name, true)])
            {
                if oa && ob && ta == tb {
                    // Don't go to the same place and open
                    continue;
                }
                // println!("{}, {}, {}, {}", ta, oa, tb, ob);
                if oa && (va.rate == 0 || state.opened.contains_key(ta)) {
                    // Can't open
                    continue;
                }
                if ob && (vb.rate == 0 || state.opened.contains_key(tb)) {
                    // Can't open
                    continue;
                }
                let mut o = state.opened.clone();
                // Should we open?
                if oa {
                    o.insert(state.posa, minute);
                }
                if ob {
                    o.insert(state.posb, minute);
                }
                if minute + 1 > 26 {
                    continue;
                }
                let score: i64 = o
                    .iter()
                    .map(|(v, t)| (26 - t) * scan.get(v).unwrap().rate)
                    .sum();
                let e: i64 = scan
                    .keys()
                    // Filter already opened
                    .filter(|x| !o.contains_key(*x))
                    // Filter unreachable
                    .map(|x| {
                        (
                            x,
                            *paths
                                .get(&(*ta, *x))
                                .unwrap()
                                .min(paths.get(&(*tb, *x)).unwrap()),
                        )
                    })
                    .map(|(v, d)| (26 - (minute + 1 + d)).max(0) * scan.get(v).unwrap().rate)
                    .sum();
                let ns = State2 {
                    posa: *ta,
                    posb: *tb,
                    opened: o,
                };
                if !gscore.contains_key(&ns) && visited.insert((0 /*minute + 1*/, ns.clone())) {
                    let next = (score + e, ns, minute + 1);
                    // println!("next: {:?}, {}, {}", next, score, e);
                    frontier.push(next);
                }
            }
        }
    }
    dbg!(gscore.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap());
    best
}

fn get_paths(scan: &aoc::FxHashMap<u16, Valve>) -> aoc::FxHashMap<(u16, u16), i64> {
    // Find all distances
    let mut graph = UnGraphMap::new();
    for (n, v) in scan {
        let gp = graph.add_node(*n);
        for t in &v.tunnels {
            let gnp = graph.add_node(*t);
            graph.add_edge(gp, gnp, 1);
        }
    }
    scan.keys()
        .flat_map(|n| {
            let res = aoc::algo::dijkstra(&graph, *n, None, |_| 1);
            res.iter()
                .map(|(nn, d)| ((*n, *nn), *d))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part1(data: &Parsed) -> Answer {
    let scan: aoc::FxHashMap<u16, Valve> = data.iter().map(|v| (v.name, v.clone())).collect();
    walk(name_to_u16("AA"), &scan, get_paths(&scan), 1)
}

fn name_to_u16(name: &str) -> u16 {
    ((name.chars().nth(0).unwrap() as u8) as u16) << 8
        | ((name.chars().nth(1).unwrap() as u8) as u16)
}

fn part2(data: &Parsed) -> Answer {
    let scan: aoc::FxHashMap<u16, Valve> = data.iter().map(|v| (v.name, v.clone())).collect();
    walk2(name_to_u16("AA"), &scan, get_paths(&scan), 1)
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let p = aoc::split_ch(x, ';');
            let pp = aoc::split_w(p[0]);
            let name = name_to_u16(pp[1]);
            let rate = pp[4][5..].parse::<i64>().unwrap();
            let tunnels = aoc::split_ch(&p[1][22..], ',')
                .iter()
                .map(|x| name_to_u16(x))
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 1707);
    }
}
