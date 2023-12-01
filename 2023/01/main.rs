use std::iter::*;

type ParsedItem = String;
type Parsed = Vec<ParsedItem>;

fn find_digits(s: &str, mappings: &[(&str, u32)]) -> Vec<u32> {
    (0..s.len())
        .map(|pos| {
            mappings.iter().find_map(move |mapping| {
                if s[pos..].starts_with(mapping.0) {
                    Some(mapping.1)
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect()
}

fn solve(data: &Parsed, mappings: &[(&str, u32)]) -> u32 {
    data.iter()
        .map(|x| find_digits(x, mappings))
        .map(|d| d[0] * 10 + d[d.len() - 1])
        .sum()
}

fn part1(data: &Parsed) -> u32 {
    let mappings = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    solve(data, &mappings)
}

fn part2(data: &Parsed) -> u32 {
    let mappings = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    solve(data, &mappings)
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
