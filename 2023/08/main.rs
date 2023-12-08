use std::{collections::HashMap, iter::*};

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("{from} = ({left}, {right})")]
struct Node {
    from: String,
    left: String,
    right: String,
}

type Parsed = (Vec<char>, Vec<Node>);

fn part1(data: &Parsed) -> i64 {
    let graph: HashMap<String, (String, String)> = data
        .1
        .iter()
        .map(|x| (x.from.clone(), (x.left.clone(), x.right.clone())))
        .collect();
    let mut pos = "AAA".to_string();
    let mut ix = 0;
    loop {
        let (l, r) = graph.get(&pos).unwrap();
        match data.0[ix % data.0.len()] {
            'L' => pos = l.clone(),
            'R' => pos = r.clone(),
            _ => panic!(),
        }
        if pos == "ZZZ" {
            break;
        }
        ix += 1;
    }
    (ix + 1) as i64
}

fn lcm_arr(arr: &[i64]) -> i64 {
    let mut ans = arr[0];

    // ans contains LCM of arr[0], ..arr[i]
    // after i'th iteration,
    for a in arr[1..].iter() {
        ans = (a * ans) / (aoc::egcd(*a, ans).0);
    }

    ans
}

fn part2(data: &Parsed) -> i64 {
    let graph: HashMap<String, (String, String)> = data
        .1
        .iter()
        .map(|x| (x.from.clone(), (x.left.clone(), x.right.clone())))
        .collect();
    let mut pos: Vec<String> = data
        .1
        .iter()
        .filter_map(|x| {
            if x.from.ends_with('A') {
                Some(x.from.clone())
            } else {
                None
            }
        })
        .collect();
    let mut ix = 0;
    let mut periods = vec![];
    loop {
        pos = pos
            .iter()
            .map(|p| {
                let (l, r) = graph.get(p).unwrap();
                match data.0[ix % data.0.len()] {
                    'L' => l.clone(),
                    'R' => r.clone(),
                    _ => panic!(),
                }
            })
            .collect();
        for pp in &pos {
            if pp.ends_with('Z') {
                periods.push((ix + 1) as i64);
            }
        }
        if periods.len() == pos.len() {
            break;
        };
        ix += 1;
    }
    lcm_arr(&periods)
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    let dirs: Vec<char> = parts[0][0].chars().collect();
    (dirs, parts[1].iter().map(|x| x.parse().unwrap()).collect())
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "LLR".into(),
            "".into(),
            "AAA = (BBB, BBB)".into(),
            "BBB = (AAA, ZZZ)".into(),
            "ZZZ = (ZZZ, ZZZ)".into(),
        ]
    }

    fn example2() -> Vec<String> {
        vec![
            "LR".into(),
            "".into(),
            "11A = (11B, XXX)".into(),
            "11B = (XXX, 11Z)".into(),
            "11Z = (11B, XXX)".into(),
            "22A = (22B, XXX)".into(),
            "22B = (22C, 22C)".into(),
            "22C = (22Z, 22Z)".into(),
            "22Z = (22B, 22B)".into(),
            "XXX = (XXX, XXX)".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example2())), 6);
    }
}
