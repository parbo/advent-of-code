use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    iter::*,
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
    let mut sum = 0;
    for m in data {
        let mut todo = BinaryHeap::new();
        let lcm = aoc::lcm_arr(&m.joltages);
        dbg!(lcm);
        let len = m.joltages.len();
        let v = vec![0; len];
        todo.push(Reverse((m.joltages.iter().sum(), 0, v)));
        let mut res = -1;
        let mut last_num = 0;
        'outer: while let Some(Reverse((_d, num, state))) = todo.pop() {
            println!("{:?} {:?}, {}", state, m.joltages, num);
            if state == m.joltages {
                if num > last_num {
                    res = last_num;
                    break;
                }
                last_num = num;
            }
            for b in &m.buttons {
                let mut new_state = state.clone();
                let mut d = 0;
                for i in 0..len {
                    if *b & (1 << i) != 0 {
                        new_state[i] += 1;
                    }
                    if new_state[i] > m.joltages[i] {
                        continue 'outer;
                    }
                    d += m.joltages[i] - new_state[i];
                }
                todo.push(Reverse((d, num + 1, new_state)));
            }
        }
        sum += res;
    }
    sum
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
    // use super::*;

    // fn example() -> Vec<String> {
    //     let s = include_str!("example.txt");
    //     s.lines().map(|x| x.to_string()).collect()
    // }

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
