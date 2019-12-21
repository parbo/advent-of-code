use aoc;
use intcode::*;
use std::iter::*;
use rustyline::Editor;

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
	    },
	    State::Input => {
		let readline = rl.readline(">> ");
		match readline {
                    Ok(x) => {
			for c in x.chars() {
			    m.add_input(c as i128);
			}
			m.add_input(10);
			if x == "WALK" {
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

fn part2(_: &Vec<i128>) -> i128 {
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
