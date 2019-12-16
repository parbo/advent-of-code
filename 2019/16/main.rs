use aoc;
// use intcode;
use std::iter::*;

fn digs_to_num(digs: &[i64], len: usize) -> i64 {
    let mut d = 1;
    let mut num = 0;
    for i in 0..len {
	num += d * digs[len - 1 - i];
	d = d * 10;
    }
    num
}

fn calc(input: &Vec<i64>, offset: usize) -> i64 {
    let base = [0, 1, 0, - 1];
    let mut inp = input.clone();
    let len = inp.len();
    for _ in 1..=100 {
        // println!("phase: {}", phase);
	let mut out = vec![0; inp.len()];
        for x in 0..len {
            let mut base_ix = 0;
	    let mut s = 0;
            for i in 0..len {
		if ((i + 1) % (x + 1)) == 0 {
                    base_ix += 1
		}
		if base_ix > 3 {
                    base_ix = 0;
		}
		let a = base[base_ix] * inp[i];
		s += a;
            }
	    let b = s.abs() % 10;
            out[x] = b;
	}
	inp = out;
    }
    digs_to_num(&inp[offset..], 8)
}

fn part1(input: &Vec<i64>) -> i64 {
    calc(input, 0)
}

fn part2(input: &Vec<i64>) -> i64 {
    let mut inp = vec![];
    for _ in 0..10000 {
	inp.extend(input);
    }
    calc(&inp, digs_to_num(input, 7) as usize)
}

fn parse(lines: &Vec<String>) -> Vec<i64> {
    lines[0]
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(&vec![8,0,8,7,1,2,2,4,5,8,5,9,1,4,5,4,6,6,1,9,0,8,3,2,1,8,6,4,5,5,9,5]), 24176176);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![0,3,0,3,6,7,3,2,5,7,7,2,1,2,9,4,4,0,6,3,4,9,1,5,6,5,4,7,4,6,6,4]), 84462026);
    }
}
