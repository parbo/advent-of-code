use aoc::Itertools;
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

fn run(
    values: &aoc::FxHashMap<String, i64>,
    bits: usize,
    gates: &[(String, Op, String, String)],
) -> Option<i64> {
    let mut values = values.clone();
    let mut zvals = aoc::FxHashSet::default();
    for (a, _op, b, out) in gates {
        for x in [a, b, out] {
            if let Some(y) = x.strip_prefix("z") {
                let num: usize = y.trim_start_matches('0').parse().unwrap_or(0);
                if num < bits {
                    zvals.insert(x.to_string());
                }
            }
        }
    }
    //    dbg!(&zvals);
    loop {
        let mut v = values.clone();
        for (a, op, b, out) in gates {
            let av = values.get(a);
            let bv = values.get(b);
            if let (Some(avv), Some(bvv)) = (av, bv) {
                match op {
                    Op::And => {
                        v.insert(out.clone(), avv & bvv);
                    }
                    Op::Xor => {
                        v.insert(out.clone(), avv ^ bvv);
                    }
                    Op::Or => {
                        v.insert(out.clone(), avv | bvv);
                    }
                }
            }
        }
        let changed = values != v;
        values = v;
        if zvals.iter().all(|x| values.contains_key(x)) {
            break;
        }
        if !changed {
            // println!("strange");
            return None;
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
    Some(zd)
}

fn add(
    a: i64,
    b: i64,
    total_bits: usize,
    bits: usize,
    gates: &[(String, Op, String, String)],
) -> Option<i64> {
    let mut values = aoc::FxHashMap::default();
    let digs = total_bits.checked_ilog10().unwrap_or(0) as usize + 1;
    //    dbg!(bits, digs);
    for i in 0..bits {
        let av = a & (1 << i);
        let bv = b & (1 << i);
        let x = format!("x{:0digs$}", i, digs = digs);
        let y = format!("y{:0digs$}", i, digs = digs);
        values.insert(x, av >> i);
        values.insert(y, bv >> i);
    }
    // dbg!(&values);
    run(&values, bits, gates)
}

fn check_valid(
    total_bits: usize,
    from_bits: usize,
    bits: usize,
    gates: &[(String, Op, String, String)],
) -> bool {
    for a in (1 << from_bits)..(1 << bits) {
        for b in (1 << from_bits)..(1 << bits) {
            let Some(v) = add(a, b, total_bits, bits, &gates) else {
                return false;
            };
            // dbg!(a, b, v);
            if v != (a + b) & ((1 << bits) - 1) {
                return false;
            }
        }
    }
    true
}

fn part1(data: &Parsed) -> i64 {
    //    run(&data.0, data.0.len() / 2, &data.1)
    0
}

fn part2(data: &Parsed) -> i64 {
    let n = data.0.len() / 2;
    let bits = n;
    let digs = bits.checked_ilog10().unwrap_or(0) as usize + 1;

    let mut gates = data.1.clone();

    let mut swapped = vec![];

    // Fix it bit by bit
    'outer: for k in 1..bits {
        if check_valid(bits, k - 1, k, &gates) {
            println!("k == {} is valid", k);
            continue;
        }

        let mut suspect = BTreeSet::new();
        for i in k..=k {
            let z = format!("z{:0digs$}", i, digs = digs);

            dbg!(&z);
            // Check what's reachable
            let mut seen = BTreeSet::new();
            let mut todo = vec![z];
            while let Some(c) = todo.pop() {
                for (a, op, b, out) in &gates {
                    if *out == c {
                        if seen.insert(a.clone()) {
                            todo.push(a.clone());
                        }
                        if seen.insert(b.clone()) {
                            todo.push(b.clone());
                        }
                    }
                }
            }
            // for j in 0..=i {
            //     let x = format!("x{:0digs$}", j, digs = digs);
            //     let y = format!("y{:0digs$}", j, digs = digs);
            //     seen.remove(&x);
            //     seen.remove(&y);
            // }
            // if seen
            //     .iter()
            //     .any(|x| x.starts_with("x") || x.starts_with("y"))
            // {
            //     suspect = suspect
            //         .union(&seen)
            //         .filter(|x| !x.starts_with("x") && !x.starts_with("y"))
            //         .cloned()
            //         .collect();
            // }
            suspect = suspect
                .union(&seen)
                .filter(|x| !x.starts_with("x") && !x.starts_with("y"))
                .cloned()
                .collect();
        }
        dbg!(&suspect);

        let mut to_swap = vec![];
        for (ix, (a, op, b, out)) in gates.iter().enumerate() {
            if suspect.contains(a) || suspect.contains(b) {
                to_swap.push(ix);
            }
        }

        for i in &to_swap {
            dbg!(i, &gates[*i], gates.len());
        }

        for x in to_swap.windows(2) {
            let mut new_gates = gates.clone();
            let v = new_gates[x[1]].3.clone();
            new_gates[x[1]].3 = new_gates[x[0]].3.clone();
            new_gates[x[0]].3 = v;
            if check_valid(bits, k - 1, k, &new_gates) {
                println!("SUCCESS for k: {}", k);
                swapped.push(gates[x[0]].0.clone());
                swapped.push(gates[x[0]].2.clone());
                swapped.push(gates[x[1]].0.clone());
                swapped.push(gates[x[1]].2.clone());
                gates = new_gates;
                continue 'outer;
            } else {
                println!(
                    "swapping {:?} and {:?} did not help",
                    gates[x[0]], gates[x[1]]
                );
            }
        }
        panic!()
    }
    dbg!(swapped);
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
