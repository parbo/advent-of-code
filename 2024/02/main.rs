use std::iter::*;

type ParsedItem = Vec<i64>;
type Parsed = Vec<ParsedItem>;

fn is_safe(row: &[i64]) -> bool {
    let mut inc = 0;
    row.windows(2).all(|a| {
        let d = a[1] - a[0];
        if d.abs() < 1 || d.abs() > 3 || (inc != 0 && inc != d.signum()) {
            return false;
        }
        inc = d.signum();
        true
    })
}

fn part1(data: &Parsed) -> i64 {
    data.iter().filter(|x| is_safe(x)).count() as i64
}

fn part2(data: &Parsed) -> i64 {
    data.iter()
        .filter(|x| {
            (0..x.len()).any(|i| {
                let v: Vec<_> = x
                    .iter()
                    .enumerate()
                    .filter_map(|(ix, xx)| if ix != i { Some(*xx) } else { None })
                    .collect();
                is_safe(&v)
            })
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
        assert!(is_safe(&row));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 4);
    }
}
