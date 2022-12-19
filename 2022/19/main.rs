use rayon::prelude::*;
use std::{collections::BinaryHeap, iter::*};

type ParsedItem = Vec<i64>;
type Parsed = Vec<ParsedItem>;

const GEODE: usize = 0;
const OBSIDIAN: usize = 1;
const CLAY: usize = 2;
const ORE: usize = 3;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct State {
    resources: [i64; 4],
    robots: [i64; 4],
    build: [i64; 4],
    minute: i64,
}

fn geodes(blueprint: &[i64], time_cap: i64) -> i64 {
    let mut visited = aoc::FxHashSet::default();
    let mut frontier = BinaryHeap::new();
    frontier.reserve(200000);
    frontier.push((
        0,
        State {
            resources: [0, 0, 0, 0],
            robots: [0, 0, 0, 1],
            build: [0; 4],
            minute: 0,
        },
    ));
    let costs = [
        (blueprint[5], 0, blueprint[6]),
        (blueprint[3], blueprint[4], 0),
        (blueprint[2], 0, 0),
        (blueprint[1], 0, 0),
    ];
    let max_robots = [
        i64::MAX,
        costs.iter().map(|x| x.2).max().unwrap(),
        costs.iter().map(|x| x.1).max().unwrap(),
        costs.iter().map(|x| x.0).max().unwrap(),
    ];
    let mut best = 0;
    while let Some((e, state)) = frontier.pop() {
        if e < best {
            break;
        }
        if state.minute == time_cap {
            if state.resources[GEODE] > best {
                best = state.resources[GEODE];
            }
            continue;
        }
        let mut states = vec![state];
        for (i, (ore_cost, clay_cost, obsidian_cost)) in costs.iter().enumerate() {
            let mut ns = state;
            if *ore_cost <= ns.resources[ORE]
                && *clay_cost <= ns.resources[CLAY]
                && *obsidian_cost <= ns.resources[OBSIDIAN]
                && ns.build.iter().all(|x| *x == 0)
                && ns.robots[i] < max_robots[i]
            {
                ns.resources[ORE] -= ore_cost;
                ns.resources[CLAY] -= clay_cost;
                ns.resources[OBSIDIAN] -= obsidian_cost;
                ns.build[i] = 1;
                states.push(ns);
            }
        }
        for mut ns in states {
            for i in 0..4 {
                ns.resources[i] += ns.robots[i];
                ns.robots[i] += ns.build[i];
            }
            ns.build = [0; 4];
            ns.minute += 1;
            if visited.insert(ns) {
                let mut res = ns.resources;
                let mut gr = ns.robots[GEODE];
                for i in 0..(time_cap - ns.minute) {
                    #[allow(clippy::needless_range_loop)]
                    for r in OBSIDIAN..=ORE {
                        res[r] += ns.robots[r] + i;
                    }
                    if costs[GEODE].0 <= res[ORE]
                        && costs[GEODE].1 <= res[CLAY]
                        && costs[GEODE].2 <= res[OBSIDIAN]
                    {
                        gr += 1;
                    }
                    res[GEODE] += gr;
                }
                frontier.push((res[GEODE], ns));
            }
        }
    }
    println!("blueprint {} found {} geodes", blueprint[0], best);
    best
}

fn part1(data: &Parsed) -> i64 {
    data.par_iter().map(|x| geodes(x, 24) * x[0]).sum()
}

fn part2(data: &Parsed) -> i64 {
    data.par_iter().take(3).map(|x| geodes(x, 32)).product()
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| aoc::things(x)).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
"Blueprint 1:  Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.".into(),
"Blueprint 2:  Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian.".into()
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 33);
    }

    #[test]
    fn test_part2() {
        assert_eq!(geodes(&parse(&example())[0], 32), 56);
        assert_eq!(geodes(&parse(&example())[1], 32), 62);
    }
}
