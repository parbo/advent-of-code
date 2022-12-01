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

type ParsedItem = i64;
type Parsed = Vec<Vec<ParsedItem>>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    data.iter().map(|x| x.iter().sum()).max().unwrap()
}

fn part2(data: &Parsed) -> Answer {
    let mut d = data.iter().map(|x| x.iter().sum()).collect::<Vec<i64>>();
    d.sort_by(|a, b| b.cmp(&a));
    d.iter().take(3).sum()
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_by_empty_line(lines)
        .iter()
        .map(|x| x.iter().map(|x| x.parse::<i64>().unwrap()).collect())
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn example() -> Vec<String> {
    // 	   vec![
    //         "0".into()
    //     ]
    // }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&parse(&example())), 0);
    // }
}
