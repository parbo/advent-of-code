use std::{collections::BTreeSet, iter::*};

use aoc::FxHashMap;

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

fn run(values: &aoc::FxHashMap<String, i64>, gates: &[(String, Op, String, String)]) -> i64 {
    let mut values = values.clone();
    let mut zvals = aoc::FxHashSet::default();
    for (a, _op, b, out) in gates {
        for x in [a, b, out] {
            if x.starts_with("z") {
                zvals.insert(x.clone());
            }
        }
    }
    loop {
        let mut v = values.clone();
        for (a, op, b, out) in gates {
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
    let mut zd = 0;
    for (i, (_k, v)) in z.iter().enumerate() {
        zd |= v << i;
    }
    zd
}

fn add(a: i64, b: i64, gates: &[(String, Op, String, String)]) -> i64 {
    let mut values = aoc::FxHashMap::default();
    let mut zvals = aoc::FxHashSet::default();
    for (a, _op, b, out) in gates {
        for x in [a, b, out] {
            if x.starts_with("z") {
                zvals.insert(x.clone());
            }
        }
    }
    let bits = zvals.len();
    let digs = bits.checked_ilog10().unwrap_or(0) as usize + 1;
    for i in 0..bits {
        let av = a & (1 << i);
        let bv = b & (1 << i);
        let x = format!("x{:0digs$}", av, digs=digs);
        let y = format!("x{:0digs$}", bv, digs=digs);
        values.insert(x, av);
        values.insert(y, bv);
    }
    run(&values, gates)
}

fn part1(data: &Parsed) -> i64 {
    run(&data.0, &data.1)
}

fn part2(data: &Parsed) -> i64 {
    run(&data.0, &data.1)
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
