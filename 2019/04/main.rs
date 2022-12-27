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
        assert_eq!(check(111111), true);
        assert_eq!(check(567789), true);
        assert_eq!(check(223450), false);
        assert_eq!(check(123789), false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(check2(112233), true);
        assert_eq!(check2(123444), false);
        assert_eq!(check2(111122), true);
        assert_eq!(check2(111223), true);
    }
}
