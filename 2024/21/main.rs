use std::{cmp::Reverse, collections::BinaryHeap, iter::*};

type Parsed = Vec<Vec<char>>;

fn keypad(p: aoc::Point) -> Option<char> {
    match p {
        [0, 0] => Some('7'),
        [1, 0] => Some('8'),
        [2, 0] => Some('9'),
        [0, 1] => Some('4'),
        [1, 1] => Some('5'),
        [2, 1] => Some('6'),
        [0, 2] => Some('1'),
        [1, 2] => Some('2'),
        [2, 2] => Some('3'),
        [1, 3] => Some('0'),
        [2, 3] => Some('A'),
        _ => None,
    }
}

fn dirpad(p: aoc::Point) -> Option<char> {
    match p {
        [1, 0] => Some('^'),
        [2, 0] => Some('A'),
        [0, 1] => Some('<'),
        [1, 1] => Some('v'),
        [2, 1] => Some('>'),
        _ => None,
    }
}

fn to_dir(c: char) -> aoc::Point {
    match c {
        '<' => aoc::WEST,
        '^' => aoc::NORTH,
        '>' => aoc::EAST,
        'v' => aoc::SOUTH,
        _ => panic!(),
    }
}

enum Action {
    Move(aoc::Point),
    Activate,
    Idle,
}

fn press(c: char) -> Action {
    match c {
        'A' => Action::Activate,
        _ => Action::Move(to_dir(c)),
    }
}

fn make_move(
    state: (aoc::Point, aoc::Point, aoc::Point),
    c: char,
) -> Option<((aoc::Point, aoc::Point, aoc::Point), Option<char>)> {
    let mut new_state = state;
    // The press moves the first robot's position
    let action = press(c);
    // The action applies to the second robot
    let action = match action {
        Action::Move(m) => {
            new_state.0 = aoc::point_add(new_state.0, m);
            dirpad(new_state.0)?;
            Action::Idle
        }
        Action::Activate => press(dirpad(new_state.0).unwrap()),
        Action::Idle => panic!(),
    };
    // Which applies to the third robot
    let action = match action {
        Action::Move(m) => {
            new_state.1 = aoc::point_add(new_state.1, m);
            dirpad(new_state.1)?;
            Action::Idle
        }
        Action::Activate => press(dirpad(new_state.1).unwrap()),
        Action::Idle => {
            // second robot didn't activate
            Action::Idle
        }
    };
    // Which applies to the last robot
    let key = match action {
        Action::Move(m) => {
            new_state.2 = aoc::point_add(new_state.2, m);
            keypad(new_state.2)?;
            None
        }
        Action::Activate => Some(keypad(new_state.2).unwrap()),
        Action::Idle => {
            // third robot didn't activate
            None
        }
    };
    Some((new_state, key))
}

fn part1(data: &Parsed) -> i64 {
    for wanted_code in data {
        let mut todo = BinaryHeap::new();
        todo.push(Reverse((
            (wanted_code.len(), 0),
            ([2, 0], [2, 0], [2, 3]),
            Vec::<char>::new(),
            Vec::<char>::new(),
        )));
        let mut seen = aoc::FxHashSet::default();
        while let Some(Reverse((_score, state, presses, code))) = todo.pop() {
            if !code.is_empty() && !wanted_code.starts_with(&code) {
                continue;
            }
            if !code.is_empty() {
                println!("{:?}, {:?}", code, presses);
            }
            if &code == wanted_code {
                dbg!(presses);
                continue;
            }
            for c in ['A', '<', '^', '>', 'v'] {
                if let Some((ns, kpc)) = make_move(state, c) {
                    let mut new_code = code.clone();
                    if let Some(c) = kpc {
                        new_code.push(c);
                    }
                    let mut new_presses = presses.clone();
                    new_presses.push(c);
                    let score = (wanted_code.len() - new_code.len(), new_presses.len());
                    if seen.insert((ns, new_presses.clone(), new_code.clone())) {
                        todo.push(Reverse((score, ns, new_presses, new_code)));
                    }
                }
            }
        }
    }
    0
}

fn part2(_: &Parsed) -> i64 {
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.chars().collect()).collect()
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
