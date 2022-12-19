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
    build_ore_robots: i64,
    build_clay_robots: i64,
    build_obsidian_robots: i64,
    build_geode_robots: i64,
}

fn quality(blueprint: &Vec<i64>) -> i64 {
    let mut visited = aoc::FxHashSet::default();
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        geodes: 0,
        obsidian: 0,
        clay: 0,
        ore: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
        build_ore_robots: 0,
        build_clay_robots: 0,
        build_obsidian_robots: 0,
        build_geode_robots: 0,
        minute: 0,
    });
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
    while let Some(state) = frontier.pop() {
        // if frontier.len() % 100 == 0 {
        // println!("{}", frontier.len());
        // println!("{:?}", state);
        // }
        if state.minute == 24 {
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
        let mut next_states = vec![];
        while let Some(mut ns) = states.pop() {
            let (ore_cost, clay_cost, obsidian_cost) = geode_robot_cost;
            next_states.push(ns);
            if ore_cost <= ns.ore
                && clay_cost <= ns.clay
                && obsidian_cost <= ns.obsidian
                && ns.build_geode_robots == 0
                && ns.build_obsidian_robots == 0
                && ns.build_clay_robots == 0
                && ns.build_ore_robots == 0
            {
                ns.ore -= ore_cost;
                ns.clay -= clay_cost;
                ns.obsidian -= obsidian_cost;
                ns.build_geode_robots += 1;
                next_states.push(ns);
            }
        }
        states = next_states;
        let mut next_states = vec![];
        while let Some(mut ns) = states.pop() {
            let (ore_cost, clay_cost, obsidian_cost) = obsidian_robot_cost;
            next_states.push(ns);
            if ore_cost <= ns.ore
                && clay_cost <= ns.clay
                && obsidian_cost <= ns.obsidian
                && ns.build_geode_robots == 0
                && ns.build_obsidian_robots == 0
                && ns.build_clay_robots == 0
                && ns.build_ore_robots == 0
            {
                ns.ore -= ore_cost;
                ns.clay -= clay_cost;
                ns.obsidian -= obsidian_cost;
                ns.build_obsidian_robots += 1;
                next_states.push(ns);
            }
        }
        states = next_states;
        let mut next_states = vec![];
        while let Some(mut ns) = states.pop() {
            let (ore_cost, clay_cost, obsidian_cost) = clay_robot_cost;
            next_states.push(ns);
            if ore_cost <= ns.ore
                && clay_cost <= ns.clay
                && obsidian_cost <= ns.obsidian
                && ns.build_geode_robots == 0
                && ns.build_obsidian_robots == 0
                && ns.build_clay_robots == 0
                && ns.build_ore_robots == 0
            {
                ns.ore -= ore_cost;
                ns.clay -= clay_cost;
                ns.obsidian -= obsidian_cost;
                ns.build_clay_robots += 1;
                next_states.push(ns);
            }
        }
        states = next_states;
        let mut next_states = vec![];
        while let Some(mut ns) = states.pop() {
            let (ore_cost, clay_cost, obsidian_cost) = ore_robot_cost;
            next_states.push(ns);
            if ore_cost <= ns.ore
                && clay_cost <= ns.clay
                && obsidian_cost <= ns.obsidian
                && ns.build_geode_robots == 0
                && ns.build_obsidian_robots == 0
                && ns.build_clay_robots == 0
                && ns.build_ore_robots == 0
            {
                ns.ore -= ore_cost;
                ns.clay -= clay_cost;
                ns.obsidian -= obsidian_cost;
                ns.build_ore_robots += 1;
                next_states.push(ns);
            }
        }
        states = next_states;
        for mut ns in states {
            ns.ore += ns.ore_robots;
            ns.clay += ns.clay_robots;
            ns.obsidian += ns.obsidian_robots;
            ns.geodes += ns.geode_robots;
            ns.ore_robots += ns.build_ore_robots;
            ns.build_ore_robots = 0;
            ns.clay_robots += ns.build_clay_robots;
            ns.build_clay_robots = 0;
            ns.obsidian_robots += ns.build_obsidian_robots;
            ns.build_obsidian_robots = 0;
            ns.geode_robots += ns.build_geode_robots;
            ns.build_geode_robots = 0;
            ns.minute += 1;
            if visited.insert(ns) {
                frontier.push(ns);
            } else {
                // println!("already visited: {:?}", ns);
            }
        }
    }
    println!("blueprint {} found {} geodes", blueprint[0], best);
    best * blueprint[0]
}

fn part1(data: &Parsed) -> i64 {
    data.iter().map(|x| quality(x)).sum()
}

fn part2(_: &Parsed) -> i64 {
    0
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
