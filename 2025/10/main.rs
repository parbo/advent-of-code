use std::collections::VecDeque;
use z3::{
    ast::{Array, Int},
    with_z3_config, Config, Optimize, SatResult, Sort,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Machine {
    lights: i64,
    buttons: Vec<i64>,
    joltages: Vec<i64>,
}

type ParsedItem = Machine;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let mut sum = 0;
    for m in data {
        let mut todo = VecDeque::new();
        todo.push_back((0, 0));
        let mut res = -1;
        'outer: while let Some((state, num)) = todo.pop_front() {
            for b in &m.buttons {
                let new_state = state ^ b;
                if new_state == m.lights {
                    res = num + 1;
                    break 'outer;
                }
                todo.push_back((new_state, num + 1));
            }
        }
        sum += res;
    }
    sum
}

fn part2(data: &Parsed) -> i64 {
    let mut tot_sum = 0;
    for m in data {
        // Set up Z3 globally via a thread-local context
        let cfg = Config::new();

        with_z3_config(&cfg, || {
            let opt = Optimize::new();

            let int_sort = Sort::int();
            let num_presses = Array::new_const("num_presses", &int_sort, &int_sort);

            for (i, joltage) in m.joltages.iter().enumerate() {
                let mut tot = Int::from_i64(0);
                // For each button that contributes to this joltage, add its button presses
                for (k, b) in m.buttons.iter().enumerate() {
                    if *b & (1 << i) != 0 {
                        let bdx = Int::from_i64(k as i64);
                        let pval = num_presses.select(&bdx).as_int().unwrap();
                        tot += pval;
                    }
                }
                let target = Int::from_i64(*joltage as i64);
                opt.assert(&tot.eq(&target));
            }
            let mut sum = Int::from_i64(0);
            for (k, _b) in m.buttons.iter().enumerate() {
                let bdx = Int::from_i64(k as i64);
                let pval = num_presses.select(&bdx).as_int().unwrap();
                opt.assert(&pval.ge(0));
                sum += pval;
            }

            // Minimize the sum of the array entries
            opt.minimize(&sum);

            match opt.check(&[]) {
                SatResult::Sat => {
                    let model = opt.get_model().unwrap();

                    let sum_val = model.eval(&sum, true).unwrap();
                    tot_sum += sum_val.as_i64().unwrap();
                }
                SatResult::Unsat => {
                    panic!();
                }
                SatResult::Unknown => {
                    panic!();
                }
            }
        });
    }
    tot_sum
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let s = x.find(']').unwrap();
            let lights = x[1..s]
                .chars()
                .enumerate()
                .map(|(i, c)| if c == '#' { 1 << i } else { 0 })
                .sum();
            let b1 = x.find('{').unwrap();
            let b2 = x.find('}').unwrap();
            let mut buttons: Vec<i64> = vec![];
            let mut p = s;
            while let Some(a) = x[p..b1].find('(') {
                let b = x[(p + a)..b1].find(')').unwrap();
                let btn: Vec<i64> = aoc::things(&x[(p + a + 1)..(p + a + b)]);
                buttons.push(btn.iter().map(|x| 1 << x).sum());
                p = p + a + b + 1;
            }
            let joltages = aoc::things(&x[(b1 + 1)..b2]);
            Machine {
                lights,
                buttons,
                joltages,
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

    fn example() -> Vec<String> {
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 33);
    }
}
