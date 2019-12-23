use aoc;
// use intcode;
use std::iter::*;

fn part1(program: &Vec<i128>) -> i128 {
    let mut machines = Vec::new();
    for i in 0..50 {
	let mut m = intcode::Machine::new(program);
	m.add_input(i);
	println!("1 len: {}", m.input_len());
	m.run_to_next_input();
	println!("2 len: {}", m.input_len());
	machines.push(m);
    }
    let mut done = false;
    while !done {
	for i in 0..50 {
	    println!("i: {}", i);
	    if machines[i].input_len() == 0 {
		println!("no input");
		machines[i].add_input(-1);
	    } else {
		println!("len: {}", machines[i].input_len());
	    }
	    println!("1");
	    let to = machines[i].run_to_next_output().unwrap();
	    println!("2");
	    let x =  machines[i].run_to_next_output().unwrap();
	    println!("3");
	    let y =  machines[i].run_to_next_output().unwrap();
	    println!("{}, {}, {}", to, x, y);
	    if y == 255 {
		done = true;
		break;
	    } else {
		machines[to as usize].add_input(x);
		machines[to as usize].add_input(y);
	    }
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
