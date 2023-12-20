use std::{collections::VecDeque, iter::*};

use aoc::{FxHashMap, FxHashSet};

type Parsed = FxHashMap<String, Vec<String>>;

fn part1(data: &Parsed) -> i64 {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
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
    for i in 0..1000 {
        let mut todo = VecDeque::from([("broadcaster".to_string(), false, "button".to_string())]);
        while let Some((s, p, from)) = todo.pop_back() {
            // if high_pulses + low_pulses == 40 {
            //     return 0;
            // }
            if p {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
            // println!("{} -{}-> {}", from, if p { "high" } else { "low" }, s);
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
                        let vals = inputs
                            .get(&s)
                            .unwrap()
                            .iter()
                            .map(|x| *m.get(x).unwrap_or(&false))
                            .collect::<Vec<_>>();
                        // dbg!(&vals);
                        let pp = !vals.iter().all(|x| *x);
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

fn part2(data: &Parsed) -> i64 {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
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
    let mut i = 0;
    loop {
        i += 1;
        if i % 10000 == 0 {
            println!("{}", i);
        }
        let mut todo = VecDeque::from([("broadcaster".to_string(), false, "button".to_string())]);
        while let Some((s, p, from)) = todo.pop_back() {
            // if high_pulses + low_pulses == 40 {
            //     return 0;
            // }
            if p {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
            // println!("{} -{}-> {}", from, if p { "high" } else { "low" }, s);
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
                        let vals = inputs
                            .get(&s)
                            .unwrap()
                            .iter()
                            .map(|x| *m.get(x).unwrap_or(&false))
                            .collect::<Vec<_>>();
                        // dbg!(&vals);
                        let pp = !vals.iter().all(|x| *x);
                        for o in out {
                            todo.push_front((o.clone(), pp, s.clone()));
                        }
                    }
                    _ => panic!(),
                }
            } else if !p {
                dbg!(s);
                return i;
            }
        }
    }
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
