use itertools::Itertools;
use std::iter::*;

fn is_sorted<I>(data: I) -> bool
where
    I: IntoIterator,
    I::Item: Ord,
{
    let mut it = data.into_iter();
    match it.next() {
        None => true,
        Some(first) => it
            .scan(first, |state, next| {
                let cmp = *state <= next;
                *state = next;
                Some(cmp)
            })
            .all(|b| b),
    }
}

fn check(number: i64) -> bool {
    let s = number.to_string();
    is_sorted(s.chars())
        && s.chars()
            .group_by(|elt| *elt)
            .into_iter()
            .map(|(_, group)| group.count())
            .filter(|c| *c >= 2)
            .count()
            > 0
}

fn check2(number: i64) -> bool {
    let s = number.to_string();
    is_sorted(s.chars())
        && s.chars()
            .group_by(|elt| *elt)
            .into_iter()
            .map(|(_, group)| group.count())
            .filter(|c| *c == 2)
            .count()
            > 0
}

fn part1(range: &(i64, i64)) -> i64 {
    let (low, high) = *range;
    let r = low..=high;
    r.filter(|&x| check(x)).count() as i64
}

fn part2(range: &(i64, i64)) -> i64 {
    let (low, high) = *range;
    let r = low..=high;
    r.filter(|&x| check2(x)).count() as i64
}

fn parse(lines: &[String]) -> (i64, i64) {
    let range: Vec<_> = lines[0]
        .split('-')
        .map(|x| x.trim().parse::<i64>())
        .filter_map(Result::ok)
        .collect();
    (range[0], range[1])
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::{check, check2};

    #[test]
    fn test_part1() {
        assert!(check(111111));
        assert!(check(567789));
        assert!(!check(223450));
        assert!(!check(123789));
    }

    #[test]
    fn test_part2() {
        assert!(check2(112233));
        assert!(!check2(123444));
        assert!(check2(111122));
        assert!(check2(111223));
    }
}
