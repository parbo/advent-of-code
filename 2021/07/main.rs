use std::iter::*;

type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(crabs: &Parsed) -> Answer {
    let s = *crabs.iter().min().unwrap();
    let e = *crabs.iter().max().unwrap();
    (s..e)
        .map(|i| crabs.iter().map(|c| (c - i).abs()).sum())
        .min()
        .unwrap()
}

fn part2(crabs: &Parsed) -> Answer {
    let s = *crabs.iter().min().unwrap();
    let e = *crabs.iter().max().unwrap();
    (s..e)
        .map(|i| {
            crabs
                .iter()
                .map(|c| (1..((c - i).abs() + 1)).sum::<i64>())
                .sum()
        })
        .min()
        .unwrap()
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_ch(&lines[0], ',')
        .iter()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
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
