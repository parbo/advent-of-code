use std::{collections::VecDeque, iter::*};

type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn solve(f: &[ParsedItem], days: i32) -> Answer {
    let mut fish = VecDeque::from(vec![0; 9]);
    f.iter().for_each(|ff| fish[*ff as usize] += 1);
    (0..days).for_each(|_| {
        fish.rotate_left(1);
        fish[6] += fish[8];
    });
    fish.iter().sum::<i64>()
}

fn part1(f: &Parsed) -> Answer {
    solve(f, 80)
}

fn part2(f: &Parsed) -> Answer {
    solve(f, 256)
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_ch(&lines[0], ',')
        .iter()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
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
