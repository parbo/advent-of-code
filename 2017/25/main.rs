use std::{collections::HashMap, iter::*};

#[derive(Debug, PartialEq, Eq)]
enum Step {
    Left,
    Right,
}

#[derive(Debug)]
struct Rule {
    value: usize,
    step: Step,
    next: char,
}

#[derive(Debug)]
struct State {
    name: char,
    actions: [Rule; 2],
}

type Parsed = (char, usize, Vec<State>);
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    let mut tape = HashMap::new();
    let mut pos = 0;
    let mut state = data.0;
    let states = data
        .2
        .iter()
        .map(|s| (s.name, s))
        .collect::<HashMap<_, _>>();
    for _ in 0..data.1 {
        let v = *tape.get(&pos).unwrap_or(&0);
        let curr_state = states.get(&state).unwrap();
        let action = &curr_state.actions[v];
        tape.insert(pos, action.value);
        if action.step == Step::Left {
            pos -= 1;
        } else {
            pos += 1;
        }
        state = action.next;
    }
    tape.values().filter(|x| **x == 1).count() as i64
}

fn part2(_: &Parsed) -> Answer {
    0
}

fn parse(lines: &[String]) -> Parsed {
    let defs = aoc::split_by_empty_line(lines);
    let start = defs[0][0][15..16].chars().next().unwrap();
    let steps = aoc::split_w(defs[0][1])[5].parse::<usize>().unwrap();
    let mut states = vec![];
    for def in &defs[1..] {
        states.push(State {
            name: aoc::split_w(def[0])[2][0..1].chars().next().unwrap(),
            actions: [
                Rule {
                    value: usize::from(def[2].ends_with("1.")),
                    step: if def[3].ends_with("right.") {
                        Step::Right
                    } else {
                        Step::Left
                    },
                    next: def[4][26..27].chars().next().unwrap(),
                },
                Rule {
                    value: usize::from(def[6].ends_with("1.")),
                    step: if def[7].ends_with("right.") {
                        Step::Right
                    } else {
                        Step::Left
                    },
                    next: def[8][26..27].chars().next().unwrap(),
                },
            ],
        });
    }
    (start, steps, states)
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
