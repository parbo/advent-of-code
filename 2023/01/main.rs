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

type ParsedItem = String;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let data: Vec<Vec<u32>> = data
        .iter()
        .map(|x| x.chars().filter_map(|x| x.to_digit(10)).collect())
        .collect();
    let mut s = 0;
    for d in data {
        s += d[0] * 10 + d[d.len() - 1]
    }
    s as i64
}

fn find_digits(s: &str) -> Vec<u32> {
    let mut d = vec![];
    let mut pos = 0;
    while pos < s.len() {
        if s[pos..].starts_with("one") {
            d.push(1);
            pos += 1;
        } else if s[pos..].starts_with("two") {
            d.push(2);
            pos += 1;
        } else if s[pos..].starts_with("three") {
            d.push(3);
            pos += 1;
        } else if s[pos..].starts_with("four") {
            d.push(4);
            pos += 1;
        } else if s[pos..].starts_with("five") {
            d.push(5);
            pos += 1;
        } else if s[pos..].starts_with("six") {
            d.push(6);
            pos += 1;
        } else if s[pos..].starts_with("seven") {
            d.push(7);
            pos += 1;
        } else if s[pos..].starts_with("eight") {
            d.push(8);
            pos += 1;
        } else if s[pos..].starts_with("nine") {
            d.push(9);
            pos += 1;
        } else if s[pos..].starts_with("1") {
            d.push(1);
            pos += 1;
        } else if s[pos..].starts_with("2") {
            d.push(2);
            pos += 1;
        } else if s[pos..].starts_with("3") {
            d.push(3);
            pos += 1;
        } else if s[pos..].starts_with("4") {
            d.push(4);
            pos += 1;
        } else if s[pos..].starts_with("5") {
            d.push(5);
            pos += 1;
        } else if s[pos..].starts_with("6") {
            d.push(6);
            pos += 1;
        } else if s[pos..].starts_with("7") {
            d.push(7);
            pos += 1;
        } else if s[pos..].starts_with("8") {
            d.push(8);
            pos += 1;
        } else if s[pos..].starts_with("9") {
            d.push(9);
            pos += 1;
        } else {
            pos += 1;
        }
    }
    dbg!(s);
    dbg!(&d);
    return d;
}

fn part2(data: &Parsed) -> i64 {
    let data: Vec<Vec<u32>> = data.iter().map(|x| find_digits(&x)).collect();
    let mut s = 0;
    for d in data {
        s += d[0] * 10 + d[d.len() - 1]
    }
    s as i64
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
        vec![
            "two1nine".into(),
            "eightwothree".into(),
            "abcone2threexyz".into(),
            "xtwone3four".into(),
            "4nineeightseven2".into(),
            "zoneight234".into(),
            "7pqrstsixteen".into(),
        ]
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 281);
    }
}
