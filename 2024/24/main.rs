use aoc::Itertools;
use std::io::prelude::*;
use std::{collections::BTreeSet, iter::*};

#[derive(
    parse_display::Display,
    parse_display::FromStr,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
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
            if let Some(_y) = x.strip_prefix("z") {
                zvals.insert(x.to_string());
            }
        }
    }
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

fn part1(data: &Parsed) -> i64 {
    run(&data.0, &data.1)
}

fn part2(data: &Parsed) -> i64 {
    let n = data.0.len() / 2;
    let gates = &data.1;
    let mut wrong = BTreeSet::new();
    // z should always be xor:ed
    for (ix, g) in gates.iter().enumerate() {
        if g.3.starts_with("z") && g.1 != Op::Xor {
            // Except for the last
            if g.3.ends_with(&n.to_string()) {
                continue;
            }
            wrong.insert(ix);
        }
    }
    // An or should always be from two ands
    for g in gates {
        if g.1 == Op::Or {
            for (ixx, gg) in gates.iter().enumerate() {
                if (gg.3 == g.0 || gg.3 == g.2) && gg.1 != Op::And {
                    wrong.insert(ixx);
                }
            }
        }
    }
    // An xor should always have xor and an or
    for g in gates {
        if g.1 == Op::Xor {
            let mut parents = vec![];
            for (ixx, gg) in gates.iter().enumerate() {
                if gg.3 == g.0 {
                    parents.push((ixx, 'L'));
                }
                if gg.3 == g.2 {
                    parents.push((ixx, 'R'));
                }
            }
            if parents.len() != 2 {
                continue;
            }
            let mut maybe_wrong = BTreeSet::new();
            if gates[parents[0].0].1 == Op::Xor && gates[parents[1].0].1 != Op::Or {
                maybe_wrong.insert(parents[1].0);
            }
            if gates[parents[0].0].1 == Op::Or && gates[parents[1].0].1 != Op::Xor {
                maybe_wrong.insert(parents[1].0);
            }
            if gates[parents[1].0].1 == Op::Xor && gates[parents[0].0].1 != Op::Or {
                maybe_wrong.insert(parents[0].0);
            }
            if gates[parents[1].0].1 == Op::Or && gates[parents[0].0].1 != Op::Xor {
                maybe_wrong.insert(parents[0].0);
            }
            for ix in maybe_wrong {
                let gg = &gates[ix];
                if (gg.0.starts_with("x") && gg.2.starts_with("y"))
                    || (gg.0.starts_with("y") && gg.2.starts_with("x"))
                {
                    // xor:s from x and y is ok
                } else {
                    wrong.insert(ix);
                }
            }
        }
    }
    // and of x and y should always go to or (except for x00 and y00)
    let re = regex::Regex::new(r"[xy]0+").unwrap();
    for (ix, g) in gates.iter().enumerate() {
        if ((g.0.starts_with("x") && g.2.starts_with("y"))
            || (g.0.starts_with("y") && g.2.starts_with("x")))
            && g.1 == Op::And
        {
            if re.is_match(&g.0) && re.is_match(&g.2) {
                continue;
            }
            for gg in gates {
                if (g.3 == gg.0 || g.3 == gg.2) && gg.1 != Op::Or && !gg.3.starts_with("z") {
                    wrong.insert(ix);
                }
            }
        }
    }

    let s = wrong.iter().map(|g| gates[*g].3.clone()).sorted().join(",");

    println!("{}", s);
    0
}

fn dump_dot(filename: &str, gates: &[(String, Op, String, String)]) {
    let mut zvals = BTreeSet::new();
    for (a, _op, b, out) in gates {
        for x in [a, b, out] {
            if let Some(_y) = x.strip_prefix("z") {
                zvals.insert(x.to_string());
            }
        }
    }
    let mut seen = BTreeSet::new();
    let mut todo: Vec<_> = zvals.into_iter().collect();
    let mut dot = vec!["digraph {".to_string()];
    while let Some(c) = todo.pop() {
        for g in gates {
            if g.3 == c && seen.insert(g.clone()) {
                dot.push(format!("{} -> \"{} {} {}\"", g.0, g.0, g.1, g.2));
                dot.push(format!("{} -> \"{} {} {}\"", g.2, g.0, g.1, g.2));
                dot.push(format!("\"{} {} {}\" -> {}", g.0, g.1, g.2, g.3));
                todo.push(g.0.clone());
                todo.push(g.2.clone());
            }
        }
    }
    dot.push("}".to_string());
    let dot = dot.iter().join("\n");
    let path = std::path::Path::new(filename);
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(dot.as_bytes()).unwrap();
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
    dump_dot("graph.dot", &gates);
    (init, gates)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
