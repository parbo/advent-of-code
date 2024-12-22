use std::collections::BTreeSet;
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

fn keypad_pos(c: char) -> Option<aoc::Point> {
    match c {
        '7' => Some([0, 0]),
        '8' => Some([1, 0]),
        '9' => Some([2, 0]),
        '4' => Some([0, 1]),
        '5' => Some([1, 1]),
        '6' => Some([2, 1]),
        '1' => Some([0, 2]),
        '2' => Some([1, 2]),
        '3' => Some([2, 2]),
        '0' => Some([1, 3]),
        'A' => Some([2, 3]),
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

fn find_kp_moves(wanted_code: &[char], pos: aoc::Point, depth: i64) -> Vec<(Vec<char>, i64)> {
    let mut todo = BinaryHeap::new();
    todo.push(Reverse((
        0,
        wanted_code.len() as i64,
        0,
        BTreeSet::new(),
        pos,
        Vec::<char>::new(),
        Vec::<char>::new(),
    )));
    let mut seen = aoc::FxHashSet::default();
    let mut result: Vec<(Vec<char>, i64)> = vec![];
    let mut best = None;
    while let Some(Reverse((_, score, 0, vis, pos, presses, code))) = todo.pop() {
        if !code.is_empty() && !wanted_code.starts_with(&code) {
            continue;
        }
        println!(
            "{:?} {:?} -> {}",
            presses.iter().join(""),
            code.iter().join(""),
            score
        );
        if code == wanted_code {
            println!("{:?} -> {}", presses.iter().join(""), score);
            if let Some(b) = best {
                if score > b {
                    // + depth.pow(3) {
                    // continue;
                    break;
                }
            } else {
                best = Some(score);
            }
            println!("{}, {:?}", result.len(), presses.to_vec().iter().join(""));
            result.push((presses, score));
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
                .count() as i64;
            let mut x = presses.clone();
            if !x.ends_with(&['A']) {
                x.push('A');
            }
            let seqs = get_sequences(x);
            let mut ssss: aoc::FxHashMap<Vec<char>, i64> = aoc::FxHashMap::default();
            for s in &seqs {
                let sss = solve_sequence(s.clone(), depth);
                for (k, v) in sss {
                    *ssss.entry(k).or_default() += v;
                }
            }
            let tot: i64 = ssss.iter().map(|(k, v)| k.len() as i64 * v).sum();
            let mut visited = vis.clone();
            visited.insert(new_pos);
            if seen.insert((
                new_pos,
                // visited.clone(),
                new_presses.clone(),
                new_code.clone(),
            )) {
                todo.push(Reverse((
                    0, //new_presses.len() as i64,
                    tot,
                    (wanted_code.len() - new_code.len()) as i64,
                    visited,
                    new_pos,
                    new_presses,
                    new_code,
                )));
            }
        }
    }
    result
}

fn find_dir_moves(pos: aoc::Point, wanted_code: &[char]) -> Vec<char> {
    let mut todo = BinaryHeap::new();
    todo.push(Reverse((
        (0, 0),
        pos,
        Vec::<char>::new(),
        Vec::<char>::new(),
    )));
    let mut seen = aoc::FxHashSet::default();
    while let Some(Reverse((_score, pos, presses, code))) = todo.pop() {
        if !code.is_empty() && !wanted_code.starts_with(&code) {
            continue;
        }
        if code == wanted_code {
            return presses;
        }
        for c in &['A', '>', '^', 'v', '<'] {
            let mut new_pos = pos;
            let kpc = match c {
                'A' => Some(dirpad(pos).unwrap()),
                _ => {
                    new_pos = aoc::point_add(pos, to_dir(*c));
                    if dirpad(new_pos).is_none() {
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
    panic!()
}

#[memoize]
fn get_sequences(pp: Vec<char>) -> Vec<Vec<char>> {
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

#[memoize]
fn find_dir_move_and_back(s: Vec<char>) -> Vec<char> {
    let mut m1 = find_dir_moves([2, 0], &s);
    let m2 = find_dir_moves(dirpad_pos(*s.last().unwrap()).unwrap(), &['A']);
    m1.extend(m2);
    m1
}

#[memoize]
fn solve_sequence(seq: Vec<char>, depth: i64) -> aoc::FxHashMap<Vec<char>, i64> {
    let mut ss = aoc::FxHashMap::default();
    if seq.is_empty() {
        *ss.entry(vec!['A']).or_default() += 1;
    } else if depth > 0 {
        let d = find_dir_move_and_back(seq.clone());
        let seqs = get_sequences(d);
        for s in &seqs {
            let sss = solve_sequence(s.clone(), depth - 1);
            for (k, v) in sss {
                *ss.entry(k).or_default() += v;
            }
        }
    } else {
        let mut sss = seq.to_vec();
        sss.push('A');
        *ss.entry(sss).or_default() += 1;
    }
    // dbg!(depth, seq.len(), ss.len());
    ss
}

fn find_presses(wanted_code: &[char], num: i64) -> i64 {
    let mut tot = 0;
    dbg!(wanted_code);
    dbg!(&wanted_code[0..=1]);
    let p = find_kp_moves(&wanted_code[0..1], [2, 3], num);
    dbg!(&p);
    tot += p.iter().min_by_key(|x| x.1).unwrap().1;
    let p = find_kp_moves(&wanted_code[1..2], keypad_pos(wanted_code[0]).unwrap(), num);
    tot += p.iter().min_by_key(|x| x.1).unwrap().1;
    let p = find_kp_moves(&wanted_code[2..3], keypad_pos(wanted_code[1]).unwrap(), num);
    tot += p.iter().min_by_key(|x| x.1).unwrap().1;
    let p = find_kp_moves(&wanted_code[3..], keypad_pos(wanted_code[2]).unwrap(), num);
    tot += p.iter().min_by_key(|x| x.1).unwrap().1;
    tot
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
        dbg!(p, num);
        complexity += p * num;
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
        dbg!(p, num);
        complexity += p * num;
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

    #[test]
    fn test_part2() {
        // TODO: verify this
        assert_eq!(part2(&parse(&example())), 175406229326698);
    }
}
