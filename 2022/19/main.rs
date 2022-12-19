use std::{cmp::Reverse, collections::BinaryHeap, iter::*};

type ParsedItem = Vec<i64>;
type Parsed = Vec<ParsedItem>;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct State {
    geodes: i64,
    geode_robots: i64,
    obsidian_robots: i64,
    clay_robots: i64,
    ore_robots: i64,
    obsidian: i64,
    clay: i64,
    ore: i64,
    minute: i64,
    build: [i64; 4],
}

fn geodes(blueprint: &Vec<i64>, time_cap: i64) -> i64 {
    let mut visited = aoc::FxHashSet::default();
    let mut frontier = BinaryHeap::new();
    frontier.push((
        0,
        State {
            geodes: 0,
            obsidian: 0,
            clay: 0,
            ore: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            build: [0; 4],
            minute: 0,
        },
    ));
    let ore_robot_cost = (blueprint[1], 0, 0);
    let clay_robot_cost = (blueprint[2], 0, 0);
    let obsidian_robot_cost = (blueprint[3], blueprint[4], 0);
    let geode_robot_cost = (blueprint[5], 0, blueprint[6]);
    let costs = [
        geode_robot_cost,
        obsidian_robot_cost,
        clay_robot_cost,
        ore_robot_cost,
    ];
    let mut best = 0;
    while let Some((e, state)) = frontier.pop() {
        // if frontier.len() % 100 == 0 {
        // println!("{}", frontier.len());
        // println!("{:?}", state);
        // }
        if e < best {
            break;
        }
        if state.minute == time_cap {
            if state.geodes > best {
                best = state.geodes;
                println!("{:?}", state);
            }
            // if state.geodes < best {
            //     break;
            // }
            continue;
        }
        let mut states = vec![state];
        for (i, (ore_cost, clay_cost, obsidian_cost)) in costs.iter().enumerate() {
            let mut ns = state;
            if *ore_cost <= ns.ore
                && *clay_cost <= ns.clay
                && *obsidian_cost <= ns.obsidian
                && ns.build.iter().all(|x| *x == 0)
            {
                ns.ore -= ore_cost;
                ns.clay -= clay_cost;
                ns.obsidian -= obsidian_cost;
                ns.build[i] = 1;
                states.push(ns);
            }
        }
        for mut ns in states {
            ns.ore += ns.ore_robots;
            ns.clay += ns.clay_robots;
            ns.obsidian += ns.obsidian_robots;
            ns.geodes += ns.geode_robots;
            ns.ore_robots += ns.build[3];
            ns.clay_robots += ns.build[2];
            ns.obsidian_robots += ns.build[1];
            ns.geode_robots += ns.build[0];
            ns.build = [0; 4];
            ns.minute += 1;
            if visited.insert(ns) {
                let mut e = ns.geodes;
                for i in 0..(time_cap - ns.minute) {
                    e += ns.geode_robots + i;
                }
                frontier.push((e, ns));
            } else {
                // println!("already visited: {:?}", ns);
            }
        }
    }
    println!("blueprint {} found {} geodes", blueprint[0], best);
    best
}

fn part1(data: &Parsed) -> i64 {
    data.iter().map(|x| geodes(x, 24) * x[0]).sum()
}

fn part2(data: &Parsed) -> i64 {
    data.iter().take(3).map(|x| geodes(x, 32)).product()
}

fn parse(lines: &[String]) -> Parsed {
    // lines.iter().map(|x| x.parse().unwrap()).collect()
    // lines[0].iter().map(|x| x.parse().unwrap()).collect()
    lines.iter().map(|x| aoc::things(x)).collect()
    // lines[0].iter().map(|x| aoc::things(x)).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn example() -> Vec<String> {
    //     let s = include_str!("example.txt");
    //     s.lines().map(|x| x.to_string()).collect()
    // }

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
}
