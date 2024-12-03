use std::iter::*;

type Parsed = String;

fn get_mul(data: &str) -> (usize, Option<(i64, i64)>) {
    if !data.starts_with("mul(") {
        return (1, None);
    }
    let Some(p) = data[4..].find(',') else {
        return (1, None);
    };
    let Ok(v) = data[4..(4 + p)].parse::<i64>() else {
        return (1, None);
    };
    let Some(p2) = data[(4 + p + 1)..].find(')') else {
        return (1, None);
    };
    let Ok(v2) = data[(4 + p + 1)..(4 + p + 1 + p2)].parse::<i64>() else {
        return (1, None);
    };
    (4 + p + 1 + p2 + 1, Some((v, v2)))
}

fn part1(data: &Parsed) -> i64 {
    let mut muls = vec![];
    let mut i = 0;
    while i < data.len() - 8 {
        let (p, res) = get_mul(&data[i..]);
        if let Some((a, b)) = res {
            muls.push((a, b));
        }
        i += p;
    }
    muls.iter().map(|(a, b)| a * b).sum()
}

fn part2(data: &Parsed) -> i64 {
    let mut muls = vec![];
    let mut enabled = true;
    let mut i = 0;
    while i < data.len() - 8 {
        if data[i..].starts_with("do()") {
            enabled = true;
            i += 4;
        } else if data[i..].starts_with("don't()") {
            enabled = false;
            i += 7;
        } else if enabled {
            let (p, res) = get_mul(&data[i..]);
            if let Some((a, b)) = res {
                muls.push((a, b));
            }
            i += p;
        } else {
            i += 1;
        }
    }
    muls.iter().map(|(a, b)| a * b).sum()
}

fn parse(lines: &[String]) -> Parsed {
    lines.join("")
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
