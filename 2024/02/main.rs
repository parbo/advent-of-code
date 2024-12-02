use std::iter::*;

// #[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
// #[display("{thing}: {al}-{ah} or {bl}-{bh}")]
// struct Rule {
//     thing: String,
//     al: i64,
//     ah: i64,
//     bl: i64,
//     bh: i64,
// }

type ParsedItem = Vec<i64>;
type Parsed = Vec<ParsedItem>;

fn is_safe(row: &[i64]) -> bool {
    let mut last = None;
    let mut inc = None;
    for r in row {
        let mut d: i64 = 0;
        if let Some(v) = last {
            d = r - v;
            if d.abs() < 1 || d.abs() > 3 {
                return false;
            }
        }
        if let Some(i) = inc {
            if (i && d < 0) || (!i && d > 0) {
                return false;
            }
        }
        last = Some(r);
        if d != 0 {
            inc = Some(d > 0);
        }
    }
    true
}

fn part1(data: &Parsed) -> i64 {
    data.iter().filter(|x| is_safe(x)).count() as i64
}

fn part2(data: &Parsed) -> i64 {
    data.iter()
        .filter(|x| {
            for i in 0..x.len() {
                let mut v = vec![];
                for (ix, xx) in x.iter().enumerate() {
                    if ix != i {
                        v.push(*xx);
                    }
                }
                if is_safe(&v) {
                    return true;
                }
            }
            false
        })
        .count() as i64
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
            "7 6 4 2 1".into(),
            "1 2 7 8 9".into(),
            "9 7 6 2 1".into(),
            "1 3 2 4 5".into(),
            "8 6 4 4 1".into(),
            "1 3 6 7 9".into(),
        ]
    }

    #[test]
    fn test_is_safe1() {
        let row = vec![55, 56, 57, 60, 63, 64, 65, 66];
        assert_eq!(is_safe(&row), true);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 2);
    }
}
