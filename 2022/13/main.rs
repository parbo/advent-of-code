use serde_json::Value;
use std::{cmp::Ordering, iter::*};

type Parsed = Vec<(Value, Value)>;
type Answer = usize;

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_i64().unwrap().cmp(&y.as_i64().unwrap()),
        (Value::Array(x), Value::Array(y)) => {
            let mut i = 0;
            for (xx, yy) in zip(x, y) {
                let r = compare(xx, yy);
                if r != Ordering::Equal {
                    return r;
                }
                i += 1
            }
            if x.len() == y.len() {
                Ordering::Equal
            } else if i == x.len() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        (Value::Array(_), Value::Number(_)) => compare(a, &Value::Array(vec![b.clone()])),
        (Value::Number(_), Value::Array(_)) => compare(&Value::Array(vec![a.clone()]), b),
        _ => panic!(),
    }
}

fn part1(data: &Parsed) -> Answer {
    data.iter()
        .enumerate()
        .filter_map(|(ix, (a, b))| {
            if compare(a, b) == Ordering::Less {
                Some(ix + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part2(data: &Parsed) -> Answer {
    let div1: Value = serde_json::from_str("[[2]]").unwrap();
    let div2: Value = serde_json::from_str("[[6]]").unwrap();
    let mut all = vec![div1.clone(), div2.clone()];
    for (a, b) in data {
        all.push(a.clone());
        all.push(b.clone());
    }
    all.sort_by(compare);
    let ix1 = all.iter().position(|a| *a == div1).unwrap();
    let ix2 = all.iter().position(|a| *a == div2).unwrap();
    (ix1 + 1) * (ix2 + 1)
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    parts
        .iter()
        .map(|x| {
            (
                serde_json::from_str(x[0]).unwrap(),
                serde_json::from_str(x[1]).unwrap(),
            )
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
