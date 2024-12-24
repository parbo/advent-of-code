use std::{collections::BTreeSet, iter::*};

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display(style = "UPPERCASE")]
enum Op {
    And,
    Xor,
    Or,
}

type Parsed = (
    aoc::FxHashMap<String, i64>,
    Vec<(String, Op, String, String)>,
);

fn part1(data: &Parsed) -> i64 {
    let mut values = data.0.clone();
    let mut zvals = aoc::FxHashSet::default();
    for (a, op, b, out) in &data.1 {
        for x in [a, b, out] {
            if x.starts_with("z") {
                zvals.insert(x.clone());
            }
        }
    }
    loop {
        let mut v = values.clone();
        for (a, op, b, out) in &data.1 {
            let av = values.get(a);
            let bv = values.get(b);
            match (av, bv) {
                (Some(avv), Some(bvv)) => match op {
                    Op::And => {
                        v.insert(out.clone(), avv & bvv);
                    }
                    Op::Xor => {
                        v.insert(out.clone(), avv ^ bvv);
                    }
                    Op::Or => {
                        v.insert(out.clone(), avv | bvv);
                    }
                },
                _ => {}
            }
        }
        values = v;
        if zvals.iter().all(|x| values.contains_key(x)) {
            break;
        }
    }
    let z: BTreeSet<(String, i64)> = values
        .into_iter()
        .filter(|(k, _v)| k.starts_with("z"))
        .collect();
    dbg!(z);
    0
}

fn part2(_: &Parsed) -> i64 {
    0
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    let init: aoc::FxHashMap<String, i64> = parts[0]
        .iter()
        .map(|x| {
            let a = aoc::split_ch(x, ':');
            (a[0].to_string(), a[1].parse().unwrap())
        })
        .collect();
    let gates: Vec<_> = parts[1]
        .iter()
        .map(|x| {
            let a = aoc::split_w(x);
            (
                a[0].to_string(),
                a[1].parse().unwrap(),
                a[2].to_string(),
                a[4].to_string(),
            )
        })
        .collect();
    (init, gates)
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
