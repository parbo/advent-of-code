use std::iter::*;
use std::time::Instant;

type Parsed = Vec<Vec<i64>>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    data.iter()
        .map(|x| x.iter().max().unwrap() - x.iter().min().unwrap())
        .sum()
}

fn part2(data: &Parsed) -> Answer {
    data.iter()
        .map(|x| {
            for i in 0..x.len() {
                for j in (i + 1)..x.len() {
                    let a = x[i].max(x[j]);
                    let b = x[i].min(x[j]);
                    if a % b == 0 {
                        return a / b;
                    }
                }
            }
            0
        })
        .sum()
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| aoc::split_w(x).iter().map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn example() -> Vec<String> {
    // 	   vec![
    //         "0".into()
    //     ]
    // }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&parse(&example())), 0);
    // }
}
