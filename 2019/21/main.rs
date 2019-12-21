use aoc;
use intcode::*;
use rustyline::Editor;
use std::collections::VecDeque;

// >> NOT A J
// >> NOT B T
// >> OR T J
// >> NOT C T
// >> OR T J
// >> AND D J
// >> WALK
fn part1(program: &Vec<i128>) -> i128 {
    let mut m = Machine::new(program);
    let mut rl = Editor::<()>::new();
    let mut state = m.run_to_next_io();
    loop {
        match state {
            State::Output => {
                for o in m.outputs() {
                    if o >= 0 && o < 256 {
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

// >> NOT A J
// >> NOT B T
// >> OR T J
// >> NOT C T
// >> OR T J
// >> AND D J

fn read(v: u32, ch: char) -> bool {
    let bit = (ch as u32) - 97;
    let mask = 1 << bit;
    (v & mask) == mask
}

fn write(v: u32, ch: char, b: bool) -> u32 {
    let bit = (ch as u32) - 97;
    let mask = 1 << bit;
    if b {
	v | mask
    } else {
	v & !mask
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    NOT,
    AND,
    OR,
}

fn validate (prog: &[(Instruction, char, char)]) -> bool {
    for v in 0..512 {
	if v & 1 == 0 {
	    continue;
	}
	let mut pos = 0;
	while pos < 10 {
	    let mut s = v;
	    for (ins, o1, o2) in prog {
		match ins {
		    Instruction::NOT => {
			s = write(s, *o2, !read(s, *o1));
		    },
		    Instruction::OR => {
			s = write(s, *o2, read(s, *o1) || read(s, *o2));
		    },
		    Instruction::AND => {
			s = write(s, *o2, read(s, *o1) && read(s, *o2));
		    },
		}
	    }
	    if read(s, 'J') {
		pos += 4;
	    } else {
		pos += 1;
	    }
	    if pos < 10 {
		let mask = 1 << pos;
		if v & mask != mask {
		    return false;
		}
	    }
	}
    }
    true
}

fn search() {
    let mut q : VecDeque<Vec<(Instruction, char, char)>> = VecDeque::new();
    let mut solutions : Vec<Vec<(Instruction, char, char)>> = vec![];
    let mut s = vec![];
    let mut last_len = 0;
    loop {
	if s.len() != last_len {
	    println!("{}, {}", s.len(), q.len());
	    last_len = s.len();
	}
	for instruction in &[Instruction::NOT, Instruction::AND, Instruction::OR] {
	    for a in &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'T', 'J']  {
		if *a != 'T' && *a != 'J' {
		    if s.iter().map(|(_, i, _)| i).any(|x| x == a) {
			// println!("already read {}", a);
			continue;
		    }
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
		    if validate(&new_s) {
			solutions.push(new_s);
		    } else {
			q.push_back(new_s);
		    }
		}
	    }
	}
	if s.len() > 15 {
	    break;
	}
	if solutions.len() > 0 {
	    break;
	}
	s = if let Some(x) = q.pop_front() {
	    // println!("{:?}", x);
	    x
	} else {
	    break;
	}
    }
    println!("{:?}", solutions);
}

// NOT E T
// NOT T T
// OR H T
// AND T J
fn part2(_: &Vec<i128>) -> i128 {
    search();
    0
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = aoc::parse_intcode(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    // use super::part1;

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }
}
