use std::iter::*;

use aoc::FxHashMap;

#[derive(
    parse_display::Display, parse_display::FromStr, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
enum Op {
    #[display(">")]
    Gt,
    #[display("<")]
    Lt,
}

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("")]
enum Outcome {
    #[display("R")]
    Rejected,
    #[display("A")]
    Accepted,
    #[from_str(regex = "(?<0>[a-z]+)")]
    Workflow(String),
}

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("")]
enum Rule {
    #[from_str(regex = "(?<0>x|m|a|s)(?<1><|>)(?<2>\\d+):(?<3>[a-zA-Z]+)")]
    Condition(char, Op, i64, Outcome),
    #[display("{0}")]
    Outcome(Outcome),
}

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("{{x={x},m={m},a={a},s={s}}}")]
struct Rating {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

type Parsed = (FxHashMap<String, Vec<Rule>>, Vec<Rating>);

fn is_accepted(workflows: &FxHashMap<String, Vec<Rule>>, rating: &Rating) -> bool {
    let mut start = "in".to_string();
    loop {
        let rules = workflows.get(&start).unwrap();
        for r in rules {
            match r {
                Rule::Outcome(Outcome::Accepted) => {
                    return true;
                }
                Rule::Outcome(Outcome::Rejected) => {
                    return false;
                }
                Rule::Outcome(Outcome::Workflow(s)) => {
                    start = s.clone();
                    break;
                }
                Rule::Condition(c, op, x, o) => {
                    let v = match c {
                        'x' => rating.x,
                        'm' => rating.m,
                        'a' => rating.a,
                        's' => rating.s,
                        _ => panic!(),
                    };
                    let res = match op {
                        Op::Gt => v > *x,
                        Op::Lt => v < *x,
                    };
                    if res {
                        match o {
                            Outcome::Rejected => {
                                return false;
                            }
                            Outcome::Accepted => {
                                return true;
                            }
                            Outcome::Workflow(s) => {
                                start = s.clone();
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn part1(data: &Parsed) -> i64 {
    let (workflows, ratings) = data;
    let mut accepted = vec![];
    for rating in ratings {
        if is_accepted(workflows, rating) {
            accepted.push(rating);
        }
    }
    accepted.iter().map(|r| r.x + r.m + r.a + r.s).sum()
}

fn part2(data: &Parsed) -> i64 {
    let (workflows, _ratings) = data;
    let mut accepted_constraints = vec![];
    let mut todo = vec![("in".to_string(), vec![])];
    while let Some((wf, cs)) = todo.pop() {
        let rules = workflows.get(&wf).unwrap();
        let mut cs = cs.clone();
        for rule in rules {
            match rule {
                Rule::Condition(c, op, num, o) => {
                    match o {
                        Outcome::Accepted => {
                            let mut cs = cs.clone();
                            cs.push((*c, *op, *num));
                            accepted_constraints.push(cs);
                        }
                        Outcome::Workflow(s) => {
                            let mut cs = cs.clone();
                            cs.push((*c, *op, *num));
                            todo.push((s.clone(), cs));
                        }
                        Outcome::Rejected => {}
                    }
                    // Push the reverse condition
                    let (op, num) = if *op == Op::Gt {
                        (Op::Lt, num + 1)
                    } else {
                        (Op::Gt, num - 1)
                    };
                    cs.push((*c, op, num));
                }
                Rule::Outcome(o) => match o {
                    Outcome::Accepted => {
                        accepted_constraints.push(cs.clone());
                    }
                    Outcome::Workflow(s) => {
                        todo.push((s.clone(), cs.clone()));
                    }
                    Outcome::Rejected => {}
                },
            }
        }
    }
    let mut num = 0;
    for cs in accepted_constraints {
        let mut xmin = 0;
        let mut xmax = 4001;
        let mut mmin = 0;
        let mut mmax = 4001;
        let mut amin = 0;
        let mut amax = 4001;
        let mut smin = 0;
        let mut smax = 4001;
        for (c, op, mut num) in cs {
            let (min, max) = match c {
                'x' => (&mut xmin, &mut xmax),
                'm' => (&mut mmin, &mut mmax),
                'a' => (&mut amin, &mut amax),
                's' => (&mut smin, &mut smax),
                _ => panic!(),
            };
            match op {
                Op::Lt => {
                    *max = *max.min(&mut num);
                }
                Op::Gt => {
                    *min = *min.max(&mut num);
                }
            }
        }
        num += (xmax.max(xmin) - xmin - 1)
            * (mmax.max(mmin) - mmin - 1)
            * (amax.max(amin) - amin - 1)
            * (smax.max(smin) - smin - 1);
    }
    num
}

fn parse(lines: &[String]) -> Parsed {
    let chunks = aoc::split_by_empty_line(lines);
    (
        chunks[0]
            .iter()
            .map(|x| {
                let (s, rest) = x.split_once('{').unwrap();
                let rules = aoc::split_ch(&rest[..rest.len() - 1], ',');
                (
                    s.to_string(),
                    rules.iter().map(|x| x.parse::<Rule>().unwrap()).collect(),
                )
            })
            .collect(),
        chunks[1].iter().map(|x| x.parse().unwrap()).collect(),
    )
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use parse_display::ParseError;

    use super::*;

    fn example() -> Vec<String> {
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_parse_op() {
        assert_eq!("<".parse::<Op>(), Ok(Op::Lt));
        assert_eq!(">".parse::<Op>(), Ok(Op::Gt));
    }

    #[test]
    fn test_parse_outcome() {
        assert_eq!("A".parse::<Outcome>(), Ok(Outcome::Accepted));
        assert_eq!("R".parse::<Outcome>(), Ok(Outcome::Rejected));
        assert_eq!(
            "abc".parse::<Outcome>(),
            Ok(Outcome::Workflow("abc".to_string()))
        );
    }

    #[test]
    fn test_parse_rule() {
        assert_eq!(
            "abc".parse::<Rule>(),
            Ok(Rule::Outcome(Outcome::Workflow("abc".to_string())))
        );
        assert_eq!(
            "a<bc".parse::<Rule>(),
            Err(ParseError::with_message("parse failed."))
        );
        assert_eq!(
            "m>1234:A".parse::<Rule>(),
            Ok(Rule::Condition('m', Op::Gt, 1234, Outcome::Accepted))
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 19114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 167409079868000);
    }
}
