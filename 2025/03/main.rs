use rayon::prelude::*;
use std::{collections::BinaryHeap, iter::*};

type ParsedItem = Vec<i64>;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let mut sum = 0;
    for row in data {
        let mut maxj = 0;
        for i in 0..row.len() {
            for j in (i + 1)..row.len() {
                let joltage = row[i] * 10 + row[j];
                maxj = maxj.max(joltage);
            }
        }
        sum += maxj;
    }
    sum
}

fn find_biggest(v: &[i64]) -> i64 {
    let mut todo = BinaryHeap::new();
    todo.push((0, 0, 0, v.to_vec()));
    let mut best = None;
    while let Some((_pot, num, c, rem)) = todo.pop() {
        if c == 12 {
            if num > best.unwrap_or(0) {
                best = Some(num);
            } else {
                break;
            }
        }
        if c < 12 {
            for i in 0..rem.len() {
                if rem.len() - i + c >= 12 {
                    let n = rem[i];
                    let nnum = n * 10i64.pow((12 - c - 1) as u32);
                    let mut nines = 0i64;
                    for k in 0..(12 - c - 1) {
                        nines += 9 * 10i64.pow(k as u32);
                    }
                    let rr = rem[(i + 1)..].to_vec();
                    todo.push((num + nnum + nines, num + nnum, c + 1, rr));
                }
            }
        }
    }
    best.unwrap()
}

fn part2(data: &Parsed) -> i64 {
    data.par_iter().map(|row| find_biggest(row)).sum()
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as i64).collect())
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "987654321111111".into(),
            "811111111111119".into(),
            "234234234234278".into(),
            "818181911112111".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 3121910778619);
    }
}
