use std::{cmp::Reverse, collections::BinaryHeap};

use aoc::memoize;
use aoc::Itertools;

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

fn dirpad_pos(c: char) -> Option<aoc::Point> {
    match c {
        '^' => Some([1, 0]),
        'A' => Some([2, 0]),
        '<' => Some([0, 1]),
        'v' => Some([1, 1]),
        '>' => Some([2, 1]),
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

fn find_kp_moves(wanted_code: &[char]) -> Vec<Vec<char>> {
    let mut todo = BinaryHeap::new();
    todo.push(Reverse((0, [2, 3], Vec::<char>::new(), Vec::<char>::new())));
    let mut seen = aoc::FxHashSet::default();
    let mut result: Vec<Vec<char>> = vec![];
    let mut best = None;
    while let Some(Reverse((score, pos, presses, code))) = todo.pop() {
        if !code.is_empty() && !wanted_code.starts_with(&code) {
            continue;
        }
        if code == wanted_code {
            // if result.contains(&presses) {
            //     continue;
            // }
            if let Some(b) = best {
                if score > b + 0 {
                    break;
                }
            } else {
                best = Some(score);
            }
            println!("{:?}", presses.to_vec().iter().join(""));
            result.push(presses);
            continue;
            // break;
        }
        for c in ['A', '>', '^', '<', 'v'] {
            match (c, presses.last()) {
                ('>', Some('<')) => continue,
                ('<', Some('>')) => continue,
                ('^', Some('v')) => continue,
                ('v', Some('^')) => continue,
                _ => {}
            }
            let mut new_pos = pos;
            let kpc = match c {
                'A' => Some(keypad(pos).unwrap()),
                _ => {
                    new_pos = aoc::point_add(pos, to_dir(c));
                    if keypad(new_pos).is_none() {
                        continue;
                    }
                    None
                }
            };
            let mut new_code = code.clone();
            if let Some(c) = kpc {
                new_code.push(c);
            }
            let mut new_presses = presses.clone();
            new_presses.push(c);
            let turns = new_presses
                .windows(2)
                .filter(|x| x[0] != 'A' && x[1] != 'A' && x[0] != x[1])
                .count();
            if seen.insert((new_pos, new_presses.clone(), new_code.clone())) {
                todo.push(Reverse((turns, new_pos, new_presses, new_code)));
            }
        }
    }
    dbg!(result.len());
    result
}

fn find_dir_moves(pos: aoc::Point, wanted_code: &[char], order: &[char]) -> Vec<char> {
    let mut todo = BinaryHeap::new();
    todo.push(Reverse((
        (0, 0),
        pos,
        Vec::<char>::new(),
        Vec::<char>::new(),
    )));
    let mut seen = aoc::FxHashSet::default();
    // let mut result: Vec<Vec<char>> = vec![];
    // let mut best = None;
    while let Some(Reverse((score, pos, presses, code))) = todo.pop() {
        if !code.is_empty() && !wanted_code.starts_with(&code) {
            continue;
        }
        // println!(
        //     "{}, {:?}, {:?}, {:?}",
        //     score,
        //     wanted_code.to_vec().iter().join(""),
        //     code.to_vec().iter().join(""),
        //     presses.to_vec().iter().join(""),
        // );
        if code == wanted_code {
            // if result.contains(&presses) {
            //     continue;
            // }
            // println!("{:?}", presses.to_vec().iter().join(""));
            // if let Some(b) = best {
            //     if score > b {
            //         break;
            //     }
            // }
            // result.push(presses);
            // best = Some(score);
            // continue;
            // break;
            return presses;
        }
        for c in order {
            // match (c, presses.last()) {
            //     ('>', Some('<')) => continue,
            //     ('<', Some('>')) => continue,
            //     ('^', Some('v')) => continue,
            //     ('v', Some('^')) => continue,
            //     _ => {}
            // }
            let mut new_pos = pos;
            let kpc = match c {
                'A' => Some(dirpad(pos).unwrap()),
                _ => {
                    new_pos = aoc::point_add(pos, to_dir(*c));
                    if dirpad(new_pos).is_none() {
                        // println!("{} not possible at {:?}", c, pos);
                        continue;
                    }
                    None
                }
            };
            let mut new_code = code.clone();
            if let Some(c) = kpc {
                new_code.push(c);
            }
            let mut new_presses = presses.clone();
            new_presses.push(*c);
            let turns = new_presses
                .windows(2)
                .filter(|x| x[0] != 'A' && x[1] != 'A' && x[0] != x[1])
                .count();
            // println!("{:?}, {}", new_presses.to_vec().iter().join(""), turns);
            if seen.insert((new_pos, new_presses.clone(), new_code.clone())) {
                todo.push(Reverse((
                    (turns, new_presses.len()),
                    new_pos,
                    new_presses,
                    new_code,
                )));
            }
        }
    }
    // dbg!(result.len());
    // result
    panic!()
}

fn get_sequences(pp: &[char]) -> Vec<Vec<char>> {
    let mut pos = 0;
    let mut seqs = vec![];
    while pos < pp.len() {
        let next = pp[pos..].iter().position(|x| *x == 'A').unwrap();
        let seq = pp[pos..][..next].to_vec();
        seqs.push(seq);
        pos += next + 1;
    }
    seqs
}

fn find_dir_move_and_back(s: &[char]) -> Vec<char> {
    let mut m1 = find_dir_moves([2, 0], s, &['A', '>', '^', 'v', '<']);
    let m2 = find_dir_moves(
        dirpad_pos(*s.last().unwrap()).unwrap(),
        &['A'],
        &['A', '>', '^', 'v', '<'],
    );
    m1.extend(m2);
    // println!("s: {:?} m1: {:?}", s.iter().join(""), m1.iter().join(""));
    m1
}

#[memoize]
fn solve_sequence(seq: Vec<char>, depth: i64) -> Vec<char> {
    // println!("{:?}, {}", seq.iter().join(""), depth);
    let mut ss = vec![];
    if seq.is_empty() {
        ss.push('A');
    } else if depth > 0 {
        let d = find_dir_move_and_back(&seq);
        let seqs = get_sequences(&d);
        for s in &seqs {
            ss.extend(solve_sequence(s.clone(), depth - 1));
        }
    } else {
        ss = seq.to_vec();
        ss.push('A');
    }
    ss
}

fn find_presses(wanted_code: &[char], num: i64) -> Vec<char> {
    // println!("code: {:?}", wanted_code.to_vec().iter().join(""));
    let p = find_kp_moves(wanted_code);
    let mut ss = vec![];
    // for pp in &p {
    //     println!("kp: {:?}", pp.iter().join(""));
    // }
    for pp in p {
        let seqs = get_sequences(&pp);
        let mut ssss = vec![];
        for s in &seqs {
            // println!("seq: {:?}", s.iter().join(""));
            ssss.extend(solve_sequence(s.clone(), num));
        }
        println!("ssss: {:?}", ssss.iter().join(""));
        ss.push(ssss);
    }
    ss.iter().min_by_key(|x| x.len()).unwrap().clone()
}

fn part1(data: &Parsed) -> i64 {
    let mut complexity = 0;
    for wanted_code in data {
        let num = wanted_code
            .iter()
            .copied()
            .filter(|x| x.is_ascii_digit())
            .skip_while(|x| *x == '0')
            .join("")
            .parse::<i64>()
            .unwrap();
        let p = find_presses(wanted_code, 2);
        dbg!(p.len(), num);
        complexity += p.len() as i64 * num;
    }
    complexity
}

fn part2(data: &Parsed) -> i64 {
    let mut complexity = 0;
    for wanted_code in data {
        let num = wanted_code
            .iter()
            .copied()
            .filter(|x| x.is_ascii_digit())
            .skip_while(|x| *x == '0')
            .join("")
            .parse::<i64>()
            .unwrap();
        let p = find_presses(wanted_code, 25);
        dbg!(p.len(), num);
        complexity += p.len() as i64 * num;
    }
    complexity
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.chars().collect()).collect()
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
        assert_eq!(part1(&parse(&example())), 126384);
    }
}
