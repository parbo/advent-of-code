use std::{collections::VecDeque, iter::*};

type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn solve(f: &[ParsedItem], days: i32) -> Answer {
    let mut fish: VecDeque<i64> = VecDeque::from([0, 0, 0, 0, 0, 0, 0, 0, 0]);
    for ff in f {
        fish[*ff as usize] += 1;
    }
    for _i in 0..days {
        let born = *fish.front().unwrap();
        fish.rotate_left(1);
        fish[6] += born;
    }
    fish.iter().sum::<i64>() as Answer
}

fn part1(f: &[ParsedItem]) -> Answer {
    solve(f, 80)
}

fn part2(f: &[ParsedItem]) -> Answer {
    solve(f, 256)
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_ch(&lines[0], ',')
        .iter()
        .map(|x| x.parse().unwrap())
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
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&vec![3, 4, 3, 1, 2]), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![3, 4, 3, 1, 2]), 26984457539);
    }
}
