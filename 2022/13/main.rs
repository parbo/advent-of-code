use std::{cmp::Ordering, iter::*, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    List(Vec<Value>),
    Number(u32),
}

impl FromStr for Value {
    type Err = aoc::ParseError;
    fn from_str(s: &str) -> Result<Value, aoc::ParseError> {
        let mut stack = vec![];
        let mut curr = vec![];
        let mut curr_n = None;
        for c in s.chars() {
            match c {
                '[' => {
                    stack.push(Value::List(curr));
                    curr = vec![];
                }
                ']' => {
                    if let Some(n) = curr_n {
                        curr.push(Value::Number(n));
                    }
                    curr_n = None;
                    if let Some(Value::List(mut v)) = stack.pop() {
                        v.push(Value::List(curr));
                        curr = v;
                    } else {
                        return Err(aoc::ParseError::Generic);
                    }
                }
                '0'..='9' => {
                    let n = curr_n.unwrap_or_default() * 10
                        + c.to_digit(10).ok_or(aoc::ParseError::Generic)?;
                    curr_n = Some(n);
                }
                ',' => {
                    if let Some(n) = curr_n {
                        curr.push(Value::Number(n));
                    }
                    curr_n = None;
                }
                _ => return Err(aoc::ParseError::Generic),
            }
        }
        if !stack.is_empty() || curr_n.is_some() {
            Err(aoc::ParseError::Generic)
        } else {
            Ok(Value::List(curr))
        }
    }
}

type Parsed = Vec<(Value, Value)>;
type Answer = usize;

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.cmp(y),
        (Value::List(x), Value::List(y)) => {
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
        (Value::List(_), Value::Number(_)) => compare(a, &Value::List(vec![b.clone()])),
        (Value::Number(_), Value::List(_)) => compare(&Value::List(vec![a.clone()]), b),
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
    let div1: Value = "[[2]]".parse().unwrap();
    let div2: Value = "[[6]]".parse().unwrap();
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
        .map(|x| (x[0].parse().unwrap(), x[1].parse().unwrap()))
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
