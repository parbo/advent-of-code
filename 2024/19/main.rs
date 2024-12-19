use std::iter::*;

type Parsed = (Vec<String>, Vec<String>);

fn solve(design: String, towels: &[String], seen: &mut aoc::FxHashMap<String, i64>) -> i64 {
    if let Some(x) = seen.get(&design) {
        return *x;
    }
    let mut num = 0;
    for ss in towels {
        if design == *ss {
            num += 1;
        } else if design.starts_with(ss) {
            num += solve(design[ss.len()..].to_string(), towels, seen);
        }
    }
    seen.insert(design, num);
    num
}

fn part1(data: &Parsed) -> i64 {
    let mut possible = 0;
    let mut seen = aoc::FxHashMap::default();
    for design in &data.1 {
        if solve(design.clone(), &data.0, &mut seen) > 0 {
            possible += 1;
        }
    }
    possible
}

fn part2(data: &Parsed) -> i64 {
    let mut possible = 0;
    let mut seen = aoc::FxHashMap::default();
    for design in &data.1 {
        possible += solve(design.clone(), &data.0, &mut seen);
    }
    possible
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
