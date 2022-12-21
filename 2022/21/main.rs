use std::{cmp::Ordering, iter::*};

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("{name}: {op}")]
struct Monkey {
    name: String,
    op: Op,
}

type ParsedItem = Monkey;
type Parsed = Vec<ParsedItem>;

fn calc(data: &Parsed, name: &str) -> (i64, Option<Ordering>) {
    for m in data {
        if m.name == name {
            match &m.op {
                Op::Number(x) => return (*x, None),
                Op::Add(a, b) => {
                    let aa = calc(data, a).0;
                    let bb = calc(data, b).0;
                    if m.name == "root" {
                        println!("a: {}, b: {}, {}", aa, bb, aa - bb);
                        return (aa + bb, Some(bb.cmp(&aa)));
                    }
                    return (aa + bb, None);
                }
                Op::Mult(a, b) => return (calc(data, a).0 * calc(data, b).0, None),
                Op::Div(a, b) => return (calc(data, a).0 / calc(data, b).0, None),
                Op::Sub(a, b) => return (calc(data, a).0 - calc(data, b).0, None),
            }
        }
    }
    (0, None)
}

fn part1(data: &Parsed) -> i64 {
    calc(data, "root").0
}

fn try_with(data: &Parsed, humn: i64) -> (i64, Option<Ordering>) {
    let mut m = data.clone();
    for mm in &mut m {
        if mm.name == "humn" {
            mm.op = Op::Number(humn);
            break;
        }
    }
    calc(&m, "root")
}

fn part2(data: &Parsed) -> i64 {
    let mut low = 0;
    let mut high = 10000000000000;

    while low <= high {
        let middle = (high + low) / 2;
        println!("middle: {}, {}, {}", middle, low, high);
        let (_current, ord) = try_with(data, middle);
        if ord == Some(Ordering::Equal) {
            return middle;
        } else if ord == Some(Ordering::Greater) {
            if middle == 0 {
                panic!();
            }
            high = middle - 1
        } else if ord == Some(Ordering::Less) {
            low = middle + 1
        } else {
            panic!();
        }
    }
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
    // lines[0].iter().map(|x| x.parse().unwrap()).collect()
    // lines.iter().map(|x| aoc::things(x)).collect()
    // lines[0].iter().map(|x| aoc::things(x)).collect()
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
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 301);
    }
}
