use std::iter::*;

type Parsed = Vec<Vec<i64>>;

fn extrapolate(h: &[i64]) -> (i64, i64) {
    let mut diffs = h.to_vec();
    let mut diffslist = vec![diffs.clone()];
    loop {
        diffs = diffs.windows(2).map(|a| a[1] - a[0]).collect();
        diffslist.push(diffs.clone());
        if diffs.iter().all(|x| *x == 0) {
            break;
        }
    }
    (
        diffslist.iter().rev().fold(0, |e, d| e + d.last().unwrap()),
        diffslist
            .iter()
            .rev()
            .fold(0, |e, d| d.first().unwrap() - e),
    )
}

fn part1(data: &Parsed) -> i64 {
    data.iter().map(|x| extrapolate(&x).0).sum()
}

fn part2(data: &Parsed) -> i64 {
    data.iter().map(|x| extrapolate(&x).1).sum()
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
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "0 3 6 9 12 15".into(),
            "1 3 6 10 15 21".into(),
            "10 13 16 21 30 45".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 2);
    }
}
