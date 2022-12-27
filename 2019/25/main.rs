use intcode::*;
use rustyline::Editor;

type Parsed = Vec<i128>;

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
                        state = m.run_to_next_io();
                    }
                    Err(_) => break,
                }
            }
            _ => break,
        }
    }
    0
}

fn part2(_: &Parsed) -> i128 {
    0
}

fn main() {
    aoc::run_main(intcode::parse_intcode, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::part1;

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }
}
