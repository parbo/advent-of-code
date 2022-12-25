use std::collections::HashMap;
use std::iter::*;

type Parsed = Vec<i64>;

fn find(numbers: &[i64], ix: usize) -> i64 {
    let mut seen = HashMap::new();
    let mut spoken = numbers.to_owned();
    for i in 0..ix {
        if i < numbers.len() {
            seen.insert(spoken[i], [i, i]);
        } else {
            let n = spoken.last().unwrap();
            let x = seen.entry(*n).or_insert([i, i]);
            let new = (x[1] - x[0]) as i64;
            let y = seen.entry(new).or_insert([i, i]);
            y[0] = y[1];
            y[1] = i;
            spoken.push(new);
        }
    }
    *spoken.last().unwrap()
}

fn part1(numbers: &Parsed) -> i64 {
    find(numbers, 2020)
}

fn part2(numbers: &Parsed) -> i64 {
    find(numbers, 30000000)
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
        assert_eq!(part1(&vec![0, 3, 6]), 436);
        assert_eq!(part1(&vec![1, 3, 2]), 1);
        assert_eq!(part1(&vec![2, 1, 3]), 10);
        assert_eq!(part1(&vec![1, 2, 3]), 27);
        assert_eq!(part1(&vec![2, 3, 1]), 78);
        assert_eq!(part1(&vec![3, 2, 1]), 438);
        assert_eq!(part1(&vec![3, 1, 2]), 1836);
    }
}
