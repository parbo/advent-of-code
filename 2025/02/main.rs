use aoc::Itertools;

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("{start}-{end}")]
struct Range {
    start: i64,
    end: i64,
}

type ParsedItem = Range;
type Parsed = Vec<ParsedItem>;

fn is_invalid(x: i64, max_div: Option<usize>) -> bool {
    let s = x.to_string();
    let len = s.len();
    let md = max_div.unwrap_or(len).min(len);
    (2..=md)
        .filter(|div| len.is_multiple_of(*div))
        .any(|div| s.as_bytes().chunks(len / div).all_equal())
}

fn part1(data: &Parsed) -> i64 {
    let mut sum = 0;
    for Range { start, end } in data {
        for x in *start..=*end {
            if is_invalid(x, Some(2)) {
                sum += x;
            }
        }
    }
    sum
}

fn part2(data: &Parsed) -> i64 {
    let mut sum = 0;
    for Range { start, end } in data {
        for x in *start..=*end {
            if is_invalid(x, None) {
                sum += x;
            }
        }
    }
    sum
}

fn parse(lines: &[String]) -> Parsed {
    aoc::things(&lines[0])
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!["11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"
            .into()]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 4174379265);
    }
}
