use aoc;
use std::iter::*;
use std::time::Instant;

fn digs_to_num(digs: &[i64], len: usize) -> i64 {
    let mut d = 1;
    let mut num = 0;
    for i in 0..len {
        num += d * digs[len - 1 - i];
        d = d * 10;
    }
    num
}

fn calc_digit(cs: &[i64], x: usize) -> i64 {
    let mut s: i64 = 0;
    let mut i = x;
    let l = cs.len();
    let mut p: i64 = 1;
    while i < l {
        let e = std::cmp::min(i + x + 1, l);
        let a: i64 = aoc::range_sum(cs, i, e);
        s += p * a;
        p = p * -1;
        i += 2 * (x + 1);
    }
    s.abs() % 10
}

fn calc(input: &Vec<i64>, phases: usize, offset: usize) -> i64 {
    let mut inp = input.clone();
    let len = inp.len();
    for phase in 1..=phases {
        let now = Instant::now();
        println!("phase: {}", phase);
        let cs = aoc::cum_sum(&inp);
        let out: Vec<_> = (0..len).into_iter().map(|x| calc_digit(&cs, x)).collect();
        inp = out;
        println!("done in {} millis", now.elapsed().as_millis());
    }
    digs_to_num(&inp[offset..], 8)
}

fn part1(input: &Vec<i64>) -> i64 {
    calc(input, 100, 0)
}

fn part2(input: &Vec<i64>) -> i64 {
    let mut inp = vec![];
    for _ in 0..10000 {
        inp.extend(input);
    }
    calc(&inp, 100, digs_to_num(input, 7) as usize)
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
    use super::{calc, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(calc(&vec![1, 2, 3, 4, 5, 6, 7, 8], 4, 0), 1029498);
        assert_eq!(
            part1(&vec![
                8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8, 6, 4,
                5, 5, 9, 5
            ]),
            24176176
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&vec![
                0, 3, 0, 3, 6, 7, 3, 2, 5, 7, 7, 2, 1, 2, 9, 4, 4, 0, 6, 3, 4, 9, 1, 5, 6, 5, 4, 7,
                4, 6, 6, 4
            ]),
            84462026
        );
    }
}
