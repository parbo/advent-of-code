use std::iter::*;

type Parsed = Vec<String>;

const SNAFU: [char; 5] = ['=', '-', '0', '1', '2'];

fn from_snafu(s: &str) -> i64 {
    s.chars().fold(0, |acc, c| {
        acc * 5 + (SNAFU.iter().position(|x| c == *x).unwrap() as i64 - 2)
    })
}

fn to_snafu(mut d: i64) -> String {
    let mut snafu = String::new();
    loop {
        let fives = (d + 2) / 5;
        let v = (d + 2) - 5 * fives;
        d = (d + 5 - v) / 5;
        snafu.insert(0, SNAFU[v as usize]);
        if fives == 0 {
            break;
        }
    }
    snafu
}

fn part1(data: &Parsed) -> String {
    let v = data.iter().map(|x| from_snafu(x)).sum();
    to_snafu(v)
}

fn part2(_: &Parsed) -> i64 {
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines.to_vec()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_to_snafu() {
        assert_eq!(to_snafu(1), "1".to_string());
        assert_eq!(to_snafu(2), "2".to_string());
        assert_eq!(to_snafu(3), "1=".to_string());
        assert_eq!(to_snafu(4), "1-".to_string());
        assert_eq!(to_snafu(5), "10".to_string());
        assert_eq!(to_snafu(6), "11".to_string());
        assert_eq!(to_snafu(7), "12".to_string());
        assert_eq!(to_snafu(8), "2=".to_string());
        assert_eq!(to_snafu(9), "2-".to_string());
        assert_eq!(to_snafu(10), "20".to_string());
        assert_eq!(to_snafu(15), "1=0".to_string());
        assert_eq!(to_snafu(20), "1-0".to_string());
        assert_eq!(to_snafu(2022), "1=11-2".to_string());
        assert_eq!(to_snafu(12345), "1-0---0".to_string());
        assert_eq!(to_snafu(314159265), "1121-1110-1=0".to_string());
    }

    #[test]
    fn test_from_snafu() {
        assert_eq!(1, from_snafu("1"));
        assert_eq!(2, from_snafu("2"));
        assert_eq!(3, from_snafu("1="));
        assert_eq!(4, from_snafu("1-"));
        assert_eq!(5, from_snafu("10"));
        assert_eq!(6, from_snafu("11"));
        assert_eq!(7, from_snafu("12"));
        assert_eq!(8, from_snafu("2="));
        assert_eq!(9, from_snafu("2-"));
        assert_eq!(10, from_snafu("20"));
        assert_eq!(15, from_snafu("1=0"));
        assert_eq!(20, from_snafu("1-0"));
        assert_eq!(2022, from_snafu("1=11-2"));
        assert_eq!(12345, from_snafu("1-0---0"));
        assert_eq!(314159265, from_snafu("1121-1110-1=0"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), "2=-1=0".to_string());
    }
}
