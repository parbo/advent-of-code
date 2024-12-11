type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;

fn solve(data: &[i64], num: usize) -> i64 {
    let mut counts = aoc::AHashMap::default();
    for s in data {
        counts.insert(*s, 1);
    }
    for _ in 0..num {
        let mut newcounts = aoc::AHashMap::default();
        for (s, c) in &counts {
            let numdigs = (s.checked_ilog10().unwrap_or(0) + 1) as usize;
            if *s == 0 {
                *newcounts.entry(1).or_default() += c;
            } else if numdigs % 2 == 0 {
                let ss = s.to_string();
                let s1 = ss[0..(numdigs / 2)].parse().unwrap();
                let s2 = ss[(numdigs / 2)..].parse().unwrap();
                *newcounts.entry(s1).or_default() += c;
                *newcounts.entry(s2).or_default() += c;
            } else {
                let s1 = s * 2024;
                *newcounts.entry(s1).or_default() += c;
            }
        }
        counts = newcounts;
    }
    counts.values().sum::<i64>()
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 25)
}

fn part2(data: &Parsed) -> i64 {
    solve(data, 75)
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
        vec!["125 17".into()]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 55312);
    }
}
