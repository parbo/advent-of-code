use std::iter::*;

#[derive(
    parse_display::Display, parse_display::FromStr, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
enum Op {
    #[display("+")]
    Add,
    #[display("*")]
    Mul,
}

type Parsed = Vec<(Vec<i64>, Vec<i64>, Op)>;

fn solve(nums: &[i64], op: Op) -> i64 {
    match op {
        Op::Add => nums.iter().sum::<i64>(),
        Op::Mul => nums.iter().product::<i64>(),
    }
}

fn part1(data: &Parsed) -> i64 {
    data.iter().map(|(nums, _, op)| solve(nums, *op)).sum()
}

fn part2(data: &Parsed) -> i64 {
    data.iter().map(|(_, nums, op)| solve(nums, *op)).sum()
}

fn parse(lines: &[String]) -> Parsed {
    let mut splits = vec![0];
    let num = lines.iter().map(|x| x.len()).max().unwrap();
    for i in 0..num {
        if lines.iter().all(|x| x.len() > i && x.as_bytes()[i] == b' ') {
            splits.push(i);
        }
    }
    splits.push(num);
    let mut result = Vec::new();
    for s in splits.windows(2) {
        let mut nums = Vec::new();
        let (j, k) = (&s[0], &s[1]);
        for line in lines.iter().take(lines.len() - 1) {
            let slice = &line[*j..*k].trim();
            nums.push(slice.parse::<i64>().unwrap());
        }
        let mut nums2 = Vec::new();
        for p in *j..*k {
            let mut s: Vec<char> = Vec::new();
            for line in lines.iter().take(lines.len() - 1) {
                s.push(line.chars().nth(p).unwrap());
            }
            let slice = s.into_iter().collect::<String>();
            let slice = slice.trim();
            if !slice.is_empty() {
                nums2.push(slice.parse::<i64>().unwrap());
            }
        }
        let op = lines[lines.len() - 1][*j..*k].trim().parse::<Op>().unwrap();
        result.push((nums, nums2, op));
    }
    result
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
