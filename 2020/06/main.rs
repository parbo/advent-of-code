use std::collections::HashMap;
use std::iter::*;

type Parsed = Vec<(usize, HashMap<char, usize>)>;

fn part1(groups: &Parsed) -> usize {
    groups.iter().map(|(_c, m)| m.len()).sum()
}

fn part2(groups: &Parsed) -> usize {
    groups
        .iter()
        .map(|(c, m)| m.iter().filter(|(_k, v)| *v == c).count())
        .sum()
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_by_empty_line(lines)
        .iter()
        .map(|group| {
            group.iter().fold((0, HashMap::new()), |mut acc, person| {
                acc.0 += 1;
                person.chars().for_each(|answer| {
                    *acc.1.entry(answer).or_insert(0 as usize) += 1;
                });
                acc
            })
        })
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
        let example: Vec<String> = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ]
        .into_iter()
        .map(|x| x.to_string())
        .collect();

        let parsed = parse(&example);
        println!("{:?}", parsed);

        assert_eq!(part1(&parsed), 11);
        assert_eq!(part2(&parsed), 6);
    }
}
