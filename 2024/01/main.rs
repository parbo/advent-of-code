use std::iter::*;

type ParsedItem = (i64, i64);
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = data.iter().copied().unzip();
    list1.sort();
    list2.sort();
    zip(list1, list2).map(|(a, b)| (a - b).abs()).sum()
}

fn part2(data: &Parsed) -> i64 {
    let mut list = aoc::FxHashMap::<i64, i64>::default();
    for v in data.iter().map(|x| x.1) {
        *list.entry(v).or_default() += 1;
    }
    data.iter()
        .map(|x| x.0 * *list.get(&x.0).unwrap_or(&0))
        .sum()
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let vals: Vec<i64> = aoc::split_w(x).iter().map(|x| x.parse().unwrap()).collect();
            (vals[0], vals[1])
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "3   4".into(),
            "4   3".into(),
            "2   5".into(),
            "1   3".into(),
            "3   9".into(),
            "3   3".into(),
        ]
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 31);
    }
}
