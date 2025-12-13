use std::{cmp::Reverse, collections::BinaryHeap, iter::*};

use aoc::{FxHashMap, FxHashSet};

type Parsed = FxHashMap<String, Vec<String>>;

fn is_reachable(
    data: &Parsed,
    from: &str,
    to: &str,
    cache: &mut FxHashMap<(String, String), bool>,
) -> bool {
    if let Some(v) = cache.get(&(from.to_string(), to.to_string())) {
        return *v;
    }
    let mut frontier = BinaryHeap::new();
    let mut visited: FxHashSet<String> = FxHashSet::default();
    let mut came_from = FxHashMap::default();
    frontier.push(Reverse((0, from.to_string())));
    while let Some(Reverse((score, current))) = frontier.pop() {
        if visited.contains(&current) {
            continue;
        }
        if current == to {
            cache.insert((from.to_string(), to.to_string()), true);
            return true;
        }
        if let Some(neighbors) = data.get(&current) {
            for nb in neighbors {
                if visited.contains(nb) {
                    continue;
                }
                let new_score = score + 1;
                came_from.insert(nb.clone(), current.clone());
                frontier.push(Reverse((new_score, nb.clone())));
            }
        }
        visited.insert(current);
    }
    cache.insert((from.to_string(), to.to_string()), false);
    false
}

fn part1(data: &Parsed) -> i64 {
    let mut num = 0;
    let mut todo = vec!["you".to_string()];
    while let Some(device) = todo.pop() {
        if device == "out" {
            num += 1;
        }
        if let Some(conns) = data.get(&device) {
            for c in conns {
                todo.push(c.clone());
            }
        }
    }
    num
}

fn find_num_paths(data: &Parsed, from: &str, to: &str) -> i64 {
    let mut num = 0;
    let mut todo: Vec<(String, Vec<String>)> = vec![(from.to_string(), vec![from.to_string()])];
    let mut is_reachable_cache: FxHashMap<(String, String), bool> = FxHashMap::default();
    let mut seen = FxHashSet::default();
    while let Some((device, path)) = todo.pop() {
        if device == to {
            num += 1;
        }
        if let Some(conns) = data.get(&device) {
            for c in conns {
                if path.contains(c) {
                    // avoid loops
                    continue;
                }
                if !is_reachable(data, c, to, &mut is_reachable_cache) {
                    // must be able to reach to
                    continue;
                }
                let mut p = path.clone();
                p.push(c.clone());
                if seen.insert((c.clone(), p.clone())) {
                    todo.push((c.clone(), p));
                }
            }
        }
    }
    num
}

fn part2(data: &Parsed) -> i64 {
    let dac = "dac".to_string();
    let fft = "fft".to_string();
    let mut is_reachable_cache: FxHashMap<(String, String), bool> = FxHashMap::default();
    let dac_to_fft = is_reachable(data, &dac, &fft, &mut is_reachable_cache);
    let fft_to_dac = is_reachable(data, &fft, &dac, &mut is_reachable_cache);
    assert!(dac_to_fft || fft_to_dac);
    assert!(!(dac_to_fft && fft_to_dac));
    let order = if dac_to_fft {
        vec![&dac, &fft]
    } else {
        vec![&fft, &dac]
    };
    let a = find_num_paths(data, "svr", order[0]);
    let b = find_num_paths(data, order[0], order[1]);
    let c = find_num_paths(data, order[1], "out");
    a * b * c
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let parts = aoc::split_ch(x, ':');
            (parts[0].to_string(), aoc::things(parts[1]))
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
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 2);
    }
}
