use std::{cmp::Ordering, iter::*};

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone)]
enum Op {
    #[display("{0}")]
    Number(i64),
    #[display("{0} + {1}")]
    Add(String, String),
    #[display("{0} * {1}")]
    Mult(String, String),
    #[display("{0} / {1}")]
    Div(String, String),
    #[display("{0} - {1}")]
    Sub(String, String),
}

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone)]
#[display("{name}: {op}")]
struct Monkey {
    name: String,
    op: Op,
}

type ParsedItem = Monkey;
type Parsed = Vec<ParsedItem>;

fn calc<T: num::Num + num::FromPrimitive + Copy>(
    data: &Parsed,
    name: &str,
    cmp: fn(T, T) -> Ordering,
) -> (T, Option<Ordering>) {
    for m in data {
        if m.name == name {
            match &m.op {
                Op::Number(x) => return (T::from_i64(*x).unwrap(), None),
                Op::Add(a, b) => {
                    let aa = calc(data, a, cmp).0;
                    let bb = calc(data, b, cmp).0;
                    if m.name == "root" {
                        return (aa + bb, Some(cmp(aa, bb)));
                    }
                    return (aa + bb, None);
                }
                Op::Mult(a, b) => return (calc(data, a, cmp).0 * calc(data, b, cmp).0, None),
                Op::Div(a, b) => {
                    return (calc(data, a, cmp).0 / calc(data, b, cmp).0, None);
                }
                Op::Sub(a, b) => return (calc(data, a, cmp).0 - calc(data, b, cmp).0, None),
            }
        }
    }
    (T::zero(), None)
}

fn part1(data: &Parsed) -> i64 {
    calc::<i64>(data, "root", |_a, _b| Ordering::Equal).0
}

fn try_with(data: &Parsed, humn: i64, cmp: fn(f64, f64) -> Ordering) -> (f64, Option<Ordering>) {
    let mut m = data.clone();
    for mm in &mut m {
        if mm.name == "humn" {
            mm.op = Op::Number(humn);
            break;
        }
    }
    calc::<f64>(&m, "root", cmp)
}

fn binary_search(data: &Parsed, cmp: fn(f64, f64) -> Ordering) -> Option<i64> {
    let mut low = 0;
    let mut high = 10000000000000;

    while low <= high {
        let middle = (high + low) / 2;
        let (_current, ord) = try_with(data, middle, cmp);
        if ord == Some(Ordering::Equal) {
            return Some(middle);
        } else if ord == Some(Ordering::Greater) {
            if middle == 0 {
                return None;
            }
            high = middle - 1
        } else if ord == Some(Ordering::Less) {
            low = middle + 1
        } else {
            panic!();
        }
    }
    None
}

fn part2(data: &Parsed) -> i64 {
    if let Some(v) = binary_search(data, |a, b| a.total_cmp(&b)) {
        v
    } else if let Some(v) = binary_search(data, |a, b| b.total_cmp(&a)) {
        v
    } else {
        panic!()
    }
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
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
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 301);
    }
}
