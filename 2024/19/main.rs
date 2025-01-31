use std::iter::*;

type Parsed = (Vec<String>, Vec<String>);

fn solve<'a>(design: &'a str, towels: &[String], seen: &mut aoc::FxHashMap<&'a str, i64>) -> i64 {
    if let Some(x) = seen.get(design) {
        return *x;
    }
    let mut num = 0;
    for ss in towels {
        if design == *ss {
            num += 1;
        } else if design.starts_with(ss) {
            num += solve(&design[ss.len()..], towels, seen);
        }
    }
    seen.insert(design, num);
    num
}

fn part1(data: &Parsed) -> i64 {
    let mut seen = aoc::FxHashMap::default();
    data.1
        .iter()
        .filter(|x| solve(x, &data.0, &mut seen) > 0)
        .count() as i64
}

fn part2(data: &Parsed) -> i64 {
    let mut seen = aoc::FxHashMap::default();
    data.1
        .iter()
        .map(|x| solve(x, &data.0, &mut seen))
        .sum::<i64>()
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    (
        aoc::split_ch(parts[0][0], ',')
            .iter()
            .map(|x| x.to_string())
            .collect(),
        parts[1].iter().map(|x| x.to_string()).collect(),
    )
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
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 16);
    }
}
