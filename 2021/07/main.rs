use std::iter::*;

type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(crabs: &[ParsedItem]) -> Answer {
    let s = *crabs.iter().min().unwrap();
    let e = *crabs.iter().max().unwrap();
    let mut cost = vec![];
    for i in s..e {
        cost.push(crabs.iter().map(|c| (c - i).abs()).sum());
    }
    *cost.iter().min().unwrap()
}

fn part2(crabs: &[ParsedItem]) -> Answer {
    let s = *crabs.iter().min().unwrap();
    let e = *crabs.iter().max().unwrap();
    let mut cost = vec![];
    for i in s..e {
        cost.push(
            crabs
                .iter()
                .map(|c| (1..((c - i).abs() + 1)).sum::<i64>())
                .sum(),
        );
    }
    *cost.iter().min().unwrap()
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_ch(&lines[0], ',')
        .iter()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 168);
    }
}
