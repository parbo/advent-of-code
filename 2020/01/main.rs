use std::iter::*;

type Parsed = Vec<i64>;

fn part1(v: &Parsed) -> i64 {
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            if v[i] + v[j] == 2020 {
                return v[i] * v[j];
            }
        }
    }
    0
}

fn part2(v: &Parsed) -> i64 {
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            for k in j + 1..v.len() {
                if v[i] + v[j] + v[k] == 2020 {
                    return v[i] * v[j] * v[k];
                }
            }
        }
    }
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
