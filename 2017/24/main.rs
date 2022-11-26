use std::{cmp::Reverse, collections::BinaryHeap, iter::*};

type ParsedItem = (i64, i64);
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn strength(bridge: &[(i64, i64)]) -> i64 {
    bridge.iter().map(|(a, b)| a + b).sum()
}

fn is_valid(bridge: &[(i64, i64)]) -> bool {
    // Must start with 0
    let mut port = 0;
    for ix in 0..bridge.len() {
        if bridge[ix].0 != port && bridge[ix].1 != port {
            return false;
        }
        port = if bridge[ix].0 != port {
            bridge[ix].0
        } else {
            bridge[ix].1
        };
    }
    true
}

fn part1(data: &Parsed) -> Answer {
    let mut queue = BinaryHeap::new();
    for component in data {
        let bridge = vec![*component];
        if is_valid(&bridge) {
            queue.push(Reverse((strength(&bridge), bridge)));
        }
    }
    let mut max_s = 0;
    while let Some(Reverse((s, bridge))) = queue.pop() {
        max_s = max_s.max(s);
        for component in data {
            if bridge.iter().position(|x| x == component).is_some() {
                continue;
            }
            let mut bridge = bridge.to_vec();
            bridge.push(*component);
            if is_valid(&bridge) {
                queue.push(Reverse((strength(&bridge), bridge)));
            }
        }
    }
    max_s
}

fn part2(data: &Parsed) -> Answer {
    let mut queue = BinaryHeap::new();
    for component in data {
        let bridge = vec![*component];
        if is_valid(&bridge) {
            queue.push(Reverse((strength(&bridge), bridge)));
        }
    }
    let mut max_s = 0;
    let mut max_l = 0;
    while let Some(Reverse((s, bridge))) = queue.pop() {
        if bridge.len() >= max_l {
            max_l = bridge.len();
            max_s = max_s.max(s);
        }
        for component in data {
            if bridge.iter().position(|x| x == component).is_some() {
                continue;
            }
            let mut bridge = bridge.to_vec();
            bridge.push(*component);
            if is_valid(&bridge) {
                queue.push(Reverse((strength(&bridge), bridge)));
            }
        }
    }
    max_s
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let parts = aoc::split_ch(x, '/');
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn example() -> Vec<String> {
    // 	   vec![
    //         "0".into()
    //     ]
    // }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&parse(&example())), 0);
    // }
}
