use std::{collections::VecDeque, iter::*};

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("move {num} from {from} to {to}")]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

type Parsed = (Vec<VecDeque<char>>, Vec<Move>);
type Answer = String;

fn part1(data: &Parsed) -> Answer {
    let mut stacks = data.0.clone();
    for mv in &data.1 {
        for _i in 0..mv.num {
            let c = stacks[mv.from - 1].pop_front().unwrap();
            stacks[mv.to - 1].push_front(c);
        }
    }
    stacks.iter().map(|x| x.front().unwrap()).copied().collect()
}

fn part2(data: &Parsed) -> Answer {
    let mut stacks = data.0.clone();
    for mv in &data.1 {
        let mut tmp = VecDeque::new();
        for _i in 0..mv.num {
            let c = stacks[mv.from - 1].pop_front().unwrap();
            tmp.push_front(c);
        }
        for x in tmp {
            stacks[mv.to - 1].push_front(x);
        }
    }
    stacks.iter().map(|x| x.front().unwrap()).copied().collect()
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    let mut stacks = vec![VecDeque::new(); 9];
    for line in &parts[0] {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let c = line.chars().nth(1 + 4 * i).unwrap();
            if c != ' ' && !c.is_ascii_digit() {
                stack.push_back(c);
            }
        }
    }
    let moves = parts[1].iter().map(|x| x.parse().unwrap()).collect();
    (stacks, moves)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
