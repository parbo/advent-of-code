use rayon::prelude::*;
use std::iter::*;

type Parsed = Vec<(i64, Vec<i64>)>;

fn possible(wanted: i64, val: i64, operands: &[i64], res: &mut Vec<i64>, ops: u32) {
    if operands.is_empty() {
        res.push(val);
    } else {
        for i in 0..ops {
            let val = if i == 0 {
                val + operands[0]
            } else if i == 1 {
                val * operands[0]
            } else {
                format!("{}{}", val, operands[0]).parse().unwrap()
            };
            if val <= wanted {
                possible(wanted, val, &operands[1..], res, ops);
            }
        }
    }
}

fn solve(data: &Parsed, ops: u32) -> i64 {
    data.par_iter()
        .filter_map(|(res, operands)| {
            let mut p = Vec::with_capacity(operands.len().pow(ops));
            possible(*res, operands[0], &operands[1..], &mut p, ops);
            if p.contains(res) {
                Some(res)
            } else {
                None
            }
        })
        .sum()
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 2)
}

fn part2(data: &Parsed) -> i64 {
    solve(data, 3)
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|line| {
            let parts = aoc::split_ch(line, ':');
            (parts[0].parse().unwrap(), aoc::things(parts[1]))
        })
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
            "190: 10 19".into(),
            "3267: 81 40 27".into(),
            "83: 17 5".into(),
            "156: 15 6".into(),
            "7290: 6 8 6 15".into(),
            "161011: 16 10 13".into(),
            "192: 17 8 14".into(),
            "21037: 9 7 18 13".into(),
            "292: 11 6 16 20".into(),
        ]
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 11387);
    }
}
