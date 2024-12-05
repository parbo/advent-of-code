use std::iter::*;

// #[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
// #[display("{thing}: {al}-{ah} or {bl}-{bh}")]
// struct Rule {
//     thing: String,
//     al: i64,
//     ah: i64,
//     bl: i64,
//     bh: i64,
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Parsed {
    rules: Vec<(i64, i64)>,
    updates: Vec<Vec<i64>>,
}

fn part1(data: &Parsed) -> i64 {
    let mut sum = 0;
    'outer: for update in data.updates.iter() {
        for rule in data.rules.iter() {
            if let Some(p1) = update.iter().position(|x| *x == rule.0) {
                if let Some(p2) = update.iter().position(|x| *x == rule.1) {
                    if p1 > p2 {
                        continue 'outer;
                    }
                }
            }
        }
        sum += update[update.len() / 2];
    }
    sum
}

fn part2(data: &Parsed) -> i64 {
    let mut incorrect = vec![];
    'outer: for update in data.updates.iter() {
        for rule in data.rules.iter() {
            if let Some(p1) = update.iter().position(|x| *x == rule.0) {
                if let Some(p2) = update.iter().position(|x| *x == rule.1) {
                    if p1 > p2 {
                        incorrect.push(update.clone());
                        continue 'outer;
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for update in incorrect.iter_mut() {
        'outer: loop {
            for rule in data.rules.iter() {
                if let Some(p1) = update.iter().position(|x| *x == rule.0) {
                    if let Some(p2) = update.iter().position(|x| *x == rule.1) {
                        if p1 > p2 {
                            update.swap(p1, p2);
                            continue 'outer;
                        }
                    }
                }
            }
            break;
        }
        sum += update[update.len() / 2];
    }
    sum
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    let rules: Vec<(i64, i64)> = parts[0]
        .iter()
        .map(|x| {
            let p = aoc::split_ch(x, '|');
            (p[0].parse().unwrap(), p[1].parse().unwrap())
        })
        .collect();
    let updates = parts[1].iter().map(|x| aoc::things(x)).collect();
    Parsed { rules, updates }
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn example() -> Vec<String> {
    //     let s = include_str!("example.txt");
    //     s.lines().map(|x| x.to_string()).collect()
    // }

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
