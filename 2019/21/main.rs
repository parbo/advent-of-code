use intcode::*;
use rustyline::Editor;
use std::{cmp::Ordering, collections::VecDeque};

type Parsed = Vec<i128>;

// >> NOT A J
// >> NOT B T
// >> OR T J
// >> NOT C T
// >> OR T J
// >> AND D J
// >> WALK
fn part1(program: &Parsed) -> i128 {
    let mut m = Machine::new(program);
    let mut rl = Editor::<()>::new();
    let mut state = m.run_to_next_io();
    loop {
        match state {
            State::Output => {
                for o in m.outputs() {
                    if (0..256).contains(&o) {
                        print!("{}", std::char::from_u32(o as u32).unwrap());
                    } else {
                        println!("value: {}", o);
                    }
                }
                state = m.run_to_next_io();
            }
            State::Input => {
                let readline = rl.readline(">> ");
                match readline {
                    Ok(s) => {
                        let x = s.trim();
                        for c in x.chars() {
                            m.add_input(c as i128);
                        }
                        m.add_input(10);
                        if x == "WALK" || x == "RUN" {
                            state = m.run_to_next_io();
                        }
                    }
                    Err(_) => break,
                }
            }
            _ => break,
        }
    }
    0
}

fn read(v: u32, ch: char) -> bool {
    let bit = (ch as u32) - 65;
    let mask = 1 << bit;
    (v & mask) == mask
}

fn write(v: u32, ch: char, b: bool) -> u32 {
    let bit = (ch as u32) - 65;
    let mask = 1 << bit;
    if b {
        v | mask
    } else {
        v & !mask
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
enum Instruction {
    NOT,
    AND,
    OR,
}

#[allow(clippy::same_item_push)]
fn poss(curr: &Vec<bool>, res: &mut Vec<Vec<bool>>, len: usize) {
    match curr.len().cmp(&len) {
        Ordering::Less => {
            for a in &[false, true] {
                if *a {
                    let mut c = curr.clone();
                    c.push(true);
                    poss(&c, res, len);
                } else if curr.len() + 4 < len {
                    let mut c = curr.clone();
                    for _ in 0..3 {
                        c.push(false);
                    }
                    c.push(true);
                    poss(&c, res, len);
                }
            }
        }
        Ordering::Equal => {
            res.push(curr.clone());
        }
        _ => (),
    }
}

fn possible() -> Vec<Vec<bool>> {
    let mut res = vec![];
    poss(&vec![true], &mut res, 10);
    res
}

fn validate(possible: &Vec<Vec<bool>>, prog: &[(Instruction, char, char)]) -> bool {
    for v in possible {
        let mut pos = 0;
        while pos < 10 {
            let mut s: u32 = 0;
            v.iter().skip(pos).enumerate().for_each(
                |(i, x)| {
                    if *x {
                        s |= 1 << i
                    } else {
                        s &= !(1 << i)
                    }
                },
            );
            println!("{:b}", s);
            for (ins, o1, o2) in prog {
                match ins {
                    Instruction::NOT => {
                        s = write(s, *o2, !read(s, *o1));
                    }
                    Instruction::OR => {
                        s = write(s, *o2, read(s, *o1) || read(s, *o2));
                    }
                    Instruction::AND => {
                        s = write(s, *o2, read(s, *o1) && read(s, *o2));
                    }
                }
                println!("{:?}, {:b}", (ins, o1, o2), s);
            }
            if read(s, 'J') {
                println!("J");
                pos += 4;
            } else {
                println!("W");
                pos += 1;
            }
            if pos < 10 {
                if !v[pos] {
                    println!("hole {:?}, {}", v, pos);
                    return false;
                }
            } else {
                println!("too far {:?}, {}", v, pos);
                return false;
            }
        }
    }
    true
}

fn to_ins(p: u8) -> (Instruction, char, char) {
    let i = p & 3;
    let ins = match i {
        0 => Instruction::NOT,
        1 => Instruction::AND,
        2 => Instruction::OR,
        _ => panic!(),
    };
    let a = (p & (0xf << 2)) >> 2;
    let ch_a = match a {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'T',
        10 => 'J',
        _ => panic!(),
    };
    let b = (p & (0x3 << 6)) >> 6;
    let ch_b = match b {
        0 => 'T',
        1 => 'J',
        _ => panic!(),
    };
    (ins, ch_a, ch_b)
}

fn from_ins(ins: &(Instruction, char, char)) -> u8 {
    let mut r = match ins.0 {
        Instruction::NOT => 0,
        Instruction::AND => 1,
        Instruction::OR => 2,
    };
    let ch_a = match ins.1 {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'T' => 9,
        'J' => 10,
        _ => panic!(),
    };
    r |= ch_a << 2;
    let ch_b = match ins.2 {
        'T' => 0,
        'J' => 1,
        _ => panic!(),
    };
    r |= ch_b << 6;
    r
}

fn search() {
    let mut q: VecDeque<Vec<u8>> = VecDeque::new();
    let mut solutions: Vec<Vec<(Instruction, char, char)>> = vec![];
    let mut s = vec![];
    let mut last_len = 0;
    let poss = possible();
    loop {
        if s.len() != last_len {
            println!("{}, {}", s.len(), q.len());
            last_len = s.len();
        }
        for instruction in &[Instruction::NOT, Instruction::AND, Instruction::OR] {
            if *instruction == Instruction::NOT
                && s.iter().map(|(c, _, _)| c).any(|x| *x == Instruction::NOT)
            {
                // Only one NOT allowed
                continue;
            }
            for a in &['A', 'B', 'C', 'D', 'E', 'H', 'T', 'J'] {
                if *a != 'T' && *a != 'J' && s.iter().map(|(_, i, _)| i).any(|x| x == a) {
                    // println!("already read {}", a);
                    continue;
                }
                for b in &['T', 'J'] {
                    if *instruction != Instruction::NOT && a == b {
                        continue;
                    }
                    let mut new_s = s.clone();
                    let next = (*instruction, *a, *b);
                    // println!("next: {:?}", next);
                    new_s.push(next);
                    // println!("new_s: {:?}", new_s);
                    if validate(&poss, &new_s) {
                        solutions.push(new_s);
                        break;
                    } else {
                        q.push_back(new_s.iter().map(from_ins).collect());
                    }
                }
            }
        }
        if s.len() > 15 {
            break;
        }
        if !solutions.is_empty() {
            break;
        }
        s = if let Some(x) = q.pop_front() {
            // println!("{:?}", x);
            x.iter().map(|x| to_ins(*x)).collect()
        } else {
            break;
        }
    }
    println!("{:?}", solutions);
}

// >> NOT A J
// >> NOT B T
// >> OR T J
// >> NOT C T
// >> OR T J
// >> AND D J
// >> NOT E T
// >> NOT T T
// >> OR H T
// >> AND T J
fn part2(_: &Vec<i128>) -> i128 {
    search();
    0
}

fn main() {
    aoc::run_main(intcode::parse_intcode, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::{possible, validate, Instruction};

    // #[test]
    // fn test_valudate() {
    //     let p = possible();
    //     assert_eq!(
    //         validate(
    //             &p,
    //             &vec![
    //                 (Instruction::NOT, 'A', 'J'),
    //                 (Instruction::NOT, 'B', 'T'),
    //                 (Instruction::OR, 'T', 'J'),
    //                 (Instruction::NOT, 'C', 'T'),
    //                 (Instruction::OR, 'T', 'J'),
    //                 (Instruction::AND, 'D', 'J'),
    //                 (Instruction::NOT, 'E', 'T'),
    //                 (Instruction::NOT, 'T', 'T'),
    //                 (Instruction::OR, 'H', 'T'),
    //                 (Instruction::AND, 'T', 'J'),
    //             ]
    //         ),
    //         true
    //     );
    // }
}
