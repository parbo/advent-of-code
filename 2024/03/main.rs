use std::iter::*;

type Parsed = String;

fn part1(data: &Parsed) -> i64 {
    let mut muls = vec![];
    for i in 0..(data.len() - 8) {
        let res = if data[i..].starts_with("mul(") {
            if let Some(p) = data[(i + 4)..].find(',') {
                if let Ok(v) = data[(i + 4)..(i + 4 + p)].parse::<i64>() {
                    if let Some(p2) = data[(i + 4 + p + 1)..].find(')') {
                        if let Ok(v2) = data[(i + 4 + p + 1)..(i + 4 + p + 1 + p2)].parse::<i64>() {
                            Some((v, v2))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        if let Some((a, b)) = res {
            muls.push((a, b));
        }
    }
    muls.iter().map(|(a, b)| a * b).sum()
}

fn part2(data: &Parsed) -> i64 {
    let mut muls = vec![];
    let mut enabled = true;
    for i in 0..(data.len() - 8) {
        let res = if data[i..].starts_with("mul(") {
            if let Some(p) = data[(i + 4)..].find(',') {
                if let Ok(v) = data[(i + 4)..(i + 4 + p)].parse::<i64>() {
                    if let Some(p2) = data[(i + 4 + p + 1)..].find(')') {
                        if let Ok(v2) = data[(i + 4 + p + 1)..(i + 4 + p + 1 + p2)].parse::<i64>() {
                            Some((v, v2))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else if data[i..].starts_with("do()") {
            enabled = true;
            None
        } else if data[i..].starts_with("don't()") {
            enabled = false;
            None
        } else {
            None
        };
        if enabled {
            if let Some((a, b)) = res {
                muls.push((a, b));
            }
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
