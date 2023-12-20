use std::{collections::VecDeque, iter::*};

use aoc::{FxHashMap, FxHashSet};

type Parsed = FxHashMap<String, Vec<String>>;

fn solve(data: &Parsed, max: i64) -> i64 {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut state: FxHashMap<String, (bool, i64)> = FxHashMap::default();
    let mut found = FxHashMap::default();
    let data: FxHashMap<String, (char, Vec<String>)> = data
        .iter()
        .map(|(op, out)| {
            if op == "broadcaster" {
                (op.clone(), ('$', out.clone()))
            } else {
                (
                    op[1..].to_string(),
                    (op.chars().next().unwrap(), out.clone()),
                )
            }
        })
        .collect();
    let mut inputs: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();
    for (s, (_, out)) in &data {
        for o in out {
            inputs.entry(o.clone()).or_default().insert(s.clone());
        }
    }
    let mut ff_memory: FxHashMap<String, bool> = FxHashMap::default();
    let mut cj_memory: FxHashMap<String, FxHashMap<String, bool>> = FxHashMap::default();
    for i in 0..max {
        let mut todo = VecDeque::from([("broadcaster".to_string(), false, "button".to_string())]);
        while let Some((s, p, from)) = todo.pop_back() {
            if p {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
            // println!("{} -{}-> {}", from, if p { "high" } else { "low" }, s);
            // Find the cycles
            for s in ["xt", "zv", "sp", "lk"] {
                let m = cj_memory.entry(s.to_string()).or_default();
                let pp = inputs
                    .get(s)
                    .unwrap()
                    .iter()
                    .map(|x| *m.get(x).unwrap_or(&false))
                    .all(|x| x);
                let (last, cnt) = state.entry(s.to_string()).or_default();
                if pp != *last {
                    if *cnt > 0 {
                        found.insert(s.to_string(), i + 1);
                    }
                    *cnt += 1;
                    *last = pp;
                }
            }
            if found.len() == 4 {
                return aoc::lcm_arr(&found.values().cloned().collect::<Vec<_>>());
            }
            if let Some((c, out)) = data.get(&s) {
                match c {
                    '$' => {
                        for o in out {
                            todo.push_front((o.clone(), p, s.clone()));
                        }
                    }
                    '%' => {
                        let m = ff_memory.entry(s.clone()).or_default();
                        if !p {
                            *m = !*m;
                            for o in out {
                                todo.push_front((o.clone(), *m, s.clone()));
                            }
                        }
                    }
                    '&' => {
                        let m = cj_memory.entry(s.clone()).or_default();
                        m.insert(from.clone(), p);
                        // dbg!(inputs.get(&s).unwrap());
                        let pp = !inputs
                            .get(&s)
                            .unwrap()
                            .iter()
                            .map(|x| *m.get(x).unwrap_or(&false))
                            .all(|x| x);
                        for o in out {
                            todo.push_front((o.clone(), pp, s.clone()));
                        }
                    }
                    _ => panic!(),
                }
            }
        }
    }
    low_pulses * high_pulses
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 1000)
}

fn part2(data: &Parsed) -> i64 {
    solve(data, 100000)
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let (op, out) = x.split_once("->").unwrap();
            let op = op.trim().to_string();
            let out = aoc::split_ch(out, ',')
                .iter()
                .map(|x| x.to_string())
                .collect();
            (op, out)
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example1() -> Vec<String> {
        let s = include_str!("example1.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    fn example2() -> Vec<String> {
        let s = include_str!("example2.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example1())), 32000000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part1(&parse(&example2())), 11687500);
    }
}
