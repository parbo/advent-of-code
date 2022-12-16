use std::{
    cmp::Ordering,
    collections::{BTreeSet, BinaryHeap, HashSet},
    iter::*,
};

use aoc::UnGraphMap;

#[derive(Debug, Clone)]
struct Valve {
    name: u8,
    rate: i64,
    tunnels: Vec<u8>,
}

type Parsed = (Vec<Valve>, aoc::FxHashMap<(u8, u8), i64>);
type Answer = i64;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct State {
    pos: u8,
    opened: [i64; 64],
}

fn walk(
    pos: u8,
    scan: &[Valve],
    paths: &aoc::FxHashMap<(u8, u8), i64>,
    minute: i64,
    time_cap: i64,
) -> i64 {
    let mut frontier = BinaryHeap::new();
    frontier.push((
        0,
        State {
            pos,
            opened: [0; 64],
        },
        minute,
    ));
    let mut visited = HashSet::new();
    let mut best = 0;
    while let Some((escore, state, minute)) = frontier.pop() {
        if escore < best {
            break;
        }
        if minute == time_cap {
            let score: i64 = state
                .opened
                .iter()
                .take(scan.len())
                .enumerate()
                .filter(|(_, v)| **v != 0)
                .map(|(v, t)| (time_cap - t) * scan[v].rate)
                .sum();
            if score > best {
                best = score;
            }
            continue;
        }
        let v = &scan[state.pos as usize];
        // dbg!(v);
        for t in &v.tunnels {
            for x in 0..2 {
                if x == 1 && v.rate == 0 {
                    // Can't open
                    continue;
                }
                let mut o = state.opened;
                let mut tl = minute;
                // Should/can we open?
                if x == 1 && o[state.pos as usize] == 0 {
                    o[state.pos as usize] = tl;
                    tl += 1;
                }
                if tl > time_cap {
                    continue;
                }
                let score: i64 = o
                    .iter()
                    .take(scan.len())
                    .enumerate()
                    .filter(|(_, v)| **v != 0)
                    .map(|(v, t)| (time_cap - t) * scan[v].rate)
                    .sum();
                let e: i64 = scan
                    .iter()
                    .map(|v| v.name)
                    // Filter already opened
                    .filter(|x| o[*x as usize] == 0)
                    // Filter unreachable
                    .map(|x| (x, *paths.get(&(*t, x)).unwrap()))
                    .map(|(v, d)| (time_cap - (tl + 1 + d)).max(0) * scan[v as usize].rate)
                    .sum();
                let ns = State { pos: *t, opened: o };
                if visited.insert(ns.clone()) {
                    let next = (score + e, ns, tl + 1);
                    frontier.push(next);
                }
            }
        }
    }
    best
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State2 {
    posa: u8,
    posb: u8,
    opened: [i64; 64],
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
    pos: u8,
    scan: &[Valve],
    paths: &aoc::FxHashMap<(u8, u8), i64>,
    minute: i64,
    time_cap: i64,
) -> i64 {
    let mut frontier = BinaryHeap::new();
    frontier.push((
        0,
        State2 {
            posa: pos,
            posb: pos,
            opened: [0; 64],
        },
        minute,
    ));
    let mut visited = HashSet::new();
    let mut best = 0;
    while let Some((escore, state, minute)) = frontier.pop() {
        if escore < best {
            break;
        }
        if minute == time_cap {
            let score: i64 = state
                .opened
                .iter()
                .take(scan.len())
                .enumerate()
                .filter(|(_, v)| **v != 0)
                .map(|(v, t)| (time_cap - *t) * scan[v].rate)
                .sum();
            if score > best {
                best = score;
            }
            continue;
        }
        let va = &scan[state.posa as usize];
        let vb = &scan[state.posb as usize];
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
                if oa && (va.rate == 0 || state.opened[*ta as usize] != 0) {
                    // Can't open
                    continue;
                }
                if ob && (vb.rate == 0 || state.opened[*tb as usize] != 0) {
                    // Can't open
                    continue;
                }
                let mut o = state.opened;
                // Should we open?
                if oa {
                    o[state.posa as usize] = minute;
                }
                if ob {
                    o[state.posb as usize] = minute;
                }
                if minute + 1 > time_cap {
                    continue;
                }
                let score: i64 = o
                    .iter()
                    .take(scan.len())
                    .enumerate()
                    .filter(|(_, v)| **v != 0)
                    .map(|(v, t)| (time_cap - *t) * scan[v].rate)
                    .sum();
                let e: i64 = scan
                    .iter()
                    .map(|v| v.name)
                    // Filter already opened
                    .filter(|x| o[*x as usize] == 0)
                    // Filter unreachable
                    .map(|x| {
                        (
                            x,
                            *paths
                                .get(&(*ta, x))
                                .unwrap()
                                .min(paths.get(&(*tb, x)).unwrap()),
                        )
                    })
                    .map(|(v, d)| (time_cap - (minute + 1 + d)).max(0) * scan[v as usize].rate)
                    .sum();
                let ns = State2 {
                    posa: *ta,
                    posb: *tb,
                    opened: o,
                };
                if visited.insert(ns.clone()) {
                    let next = (score + e, ns, minute + 1);
                    frontier.push(next);
                }
            }
        }
    }
    best
}

fn get_paths(scan: &[Valve]) -> aoc::FxHashMap<(u8, u8), i64> {
    // Find all distances
    let mut graph = UnGraphMap::new();
    for v in scan {
        let gp = graph.add_node(v.name);
        for t in &v.tunnels {
            let gnp = graph.add_node(*t);
            graph.add_edge(gp, gnp, 1);
        }
    }
    scan.iter()
        .flat_map(|n| {
            let res = aoc::algo::dijkstra(&graph, n.name, None, |_| 1);
            res.iter()
                .map(|(nn, d)| ((n.name, *nn), *d as i64))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part1(data: &Parsed) -> Answer {
    walk(0, &data.0, &data.1, 1, 30)
}

fn part2(data: &Parsed) -> Answer {
    walk2(0, &data.0, &data.1, 1, 26)
}

fn parse(lines: &[String]) -> Parsed {
    let data: Vec<_> = lines
        .iter()
        .map(|x| {
            let p = aoc::split_ch(x, ';');
            let pp = aoc::split_w(p[0]);
            let name = pp[1];
            let rate = pp[4][5..].parse::<i64>().unwrap();
            let tunnels = aoc::split_ch(&p[1][22..], ',');
            (name, rate, tunnels)
        })
        .collect();
    let names: BTreeSet<_> = data.iter().map(|(name, _, _)| *name).collect();
    let get_name = |name: &str| {
        names
            .iter()
            .enumerate()
            .find(|(_, n)| **n == name)
            .unwrap()
            .0 as u8
    };

    let mut scan: Vec<Valve> = data
        .iter()
        .map(|v| Valve {
            name: get_name(v.0),
            rate: v.1,
            tunnels: v.2.iter().map(|t| get_name(t)).collect(),
        })
        .collect();
    // Scan must be indexable by name
    scan.sort_by(|a, b| a.name.cmp(&b.name));
    for (i, v) in scan.iter().enumerate() {
        assert_eq!(i as u8, v.name);
    }
    assert_eq!(get_name("AA"), 0);
    let paths = get_paths(&scan);
    (scan, paths)
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
