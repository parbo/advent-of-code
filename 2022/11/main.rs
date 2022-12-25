use std::{
    collections::{HashMap, VecDeque},
    iter::*,
};

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul(i64),
    Add(i64),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    op: Op,
    test: i64,
    throw: [usize; 2],
}

type ParsedItem = Monkey;
type Parsed = Vec<ParsedItem>;
type Answer = usize;

fn part1(data: &Parsed) -> Answer {
    let mut monkeys = data.clone();
    let num = monkeys.len();
    let mut inspect: HashMap<usize, usize> = HashMap::new();
    for _ in 0..20 {
        for m in 0..num {
            loop {
                let item = monkeys[m].items.pop_front();
                if let Some(mut worry) = item {
                    *inspect.entry(m).or_default() += 1;
                    match monkeys[m].op {
                        Op::Mul(x) => worry *= x,
                        Op::Add(x) => worry += x,
                        Op::Square => worry *= worry,
                    }
                    worry /= 3;
                    let t = (worry % monkeys[m].test != 0) as usize;
                    let to = monkeys[m].throw[t];
                    monkeys[to].items.push_back(worry);
                } else {
                    break;
                }
            }
        }
    }
    let mut active = inspect.iter().collect::<Vec<_>>();
    active.sort_by(|a, b| b.1.cmp(a.1));
    active.iter().take(2).map(|(_k, v)| **v).product::<usize>()
}

fn part2(data: &Parsed) -> Answer {
    let mut monkeys = data.clone();
    let num = monkeys.len();
    let mut inspect: HashMap<usize, usize> = HashMap::new();
    let gcd: i64 = data.iter().map(|m| m.test).product();
    for _ in 0..10000 {
        for m in 0..num {
            loop {
                let item = monkeys[m].items.pop_front();
                if let Some(mut worry) = item {
                    worry %= gcd;
                    *inspect.entry(m).or_default() += 1;
                    match monkeys[m].op {
                        Op::Mul(x) => worry *= x,
                        Op::Add(x) => worry += x,
                        Op::Square => worry *= worry,
                    }
                    let t = (worry % monkeys[m].test != 0) as usize;
                    let to = monkeys[m].throw[t];
                    monkeys[to].items.push_back(worry);
                } else {
                    break;
                }
            }
        }
    }
    let mut active = inspect.iter().collect::<Vec<_>>();
    active.sort_by(|a, b| b.1.cmp(a.1));
    active.iter().take(2).map(|(_k, v)| **v).product::<usize>()
}

fn parse(lines: &[String]) -> Parsed {
    let monkeys = aoc::split_by_empty_line(lines);
    monkeys
        .iter()
        .map(|x| {
            let items = aoc::split_ch(aoc::split_ch(x[1], ':')[1], ',')
                .iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            let operands = aoc::split_w(x[2]);
            let op = match (operands[4], operands[5]) {
                ("*", "old") => Op::Square,
                ("*", x) => Op::Mul(x.parse::<i64>().unwrap()),
                ("+", x) => Op::Add(x.parse::<i64>().unwrap()),
                _ => unreachable!(),
            };
            let test = aoc::split_w(x[3])[3].parse::<i64>().unwrap();
            let throw = [
                aoc::split_w(x[4])[5].parse::<usize>().unwrap(),
                aoc::split_w(x[5])[5].parse::<usize>().unwrap(),
            ];
            Monkey {
                items,
                op,
                test,
                throw,
            }
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse(
                &EXAMPLE.lines().map(String::from).collect::<Vec<_>>()
            )),
            10605
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse(
                &EXAMPLE.lines().map(String::from).collect::<Vec<_>>()
            )),
            2713310158
        );
    }
}
