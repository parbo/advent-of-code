use std::collections::HashMap;
use std::iter::*;

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, Copy, PartialEq, Eq)]
#[display(style = "snake_case")]
enum Op {
    Inc,
    Dec,
}

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, Copy, PartialEq, Eq)]
enum Cond {
    #[display(">")]
    Gt,
    #[display("<")]
    Lt,
    #[display(">=")]
    Gte,
    #[display("<=")]
    Lte,
    #[display("==")]
    Eq,
    #[display("!=")]
    Neq,
}

#[derive(Debug, Clone)]
struct Instruction {
    modify: String,
    verify: String,
    op: Op,
    cond: Cond,
    operand: i64,
    comparand: i64,
}

type ParsedItem = Instruction;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn run(data: &[ParsedItem]) -> (i64, i64) {
    let mut registers: HashMap<String, i64> = HashMap::new();
    let mut max = 0;
    for ins in data {
        let rver = registers.entry(ins.verify.to_string()).or_insert(0);
        let cmp_res = match ins.cond {
            Cond::Gt => *rver > ins.comparand,
            Cond::Lt => *rver < ins.comparand,
            Cond::Gte => *rver >= ins.comparand,
            Cond::Lte => *rver <= ins.comparand,
            Cond::Eq => *rver == ins.comparand,
            Cond::Neq => *rver != ins.comparand,
        };
        if cmp_res {
            let rmod = registers.entry(ins.modify.to_string()).or_insert(0);
            match ins.op {
                Op::Inc => {
                    *rmod += ins.operand;
                }
                Op::Dec => {
                    *rmod -= ins.operand;
                }
            }
            if *rmod > max {
                max = *rmod;
            }
        }
    }
    (*registers.values().max().unwrap(), max)
}

fn part1(data: &Parsed) -> Answer {
    run(data).0
}

fn part2(data: &Parsed) -> Answer {
    run(data).1
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let s = aoc::split_w(x);
            let modify = s[0].to_string();
            let op = s[1].parse::<Op>().unwrap();
            let operand = s[2].parse::<i64>().unwrap();
            let verify = s[4].to_string();
            let cond = s[5].parse::<Cond>().unwrap();
            let comparand = s[6].parse::<i64>().unwrap();
            Instruction {
                modify,
                verify,
                op,
                cond,
                operand,
                comparand,
            }
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::*;

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
