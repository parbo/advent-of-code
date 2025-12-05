use std::iter::*;

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("{from}-{to}")]
struct Range {
    from: i64,
    to: i64,
}

type Parsed = (Vec<Range>, Vec<i64>);

fn part1(data: &Parsed) -> i64 {
    let mut num = 0;
    for ingredient in &data.1 {
        let mut is_fresh = false;
        for range in &data.0 {
            if *ingredient >= range.from && *ingredient <= range.to {
                is_fresh = true;
                break;
            }
        }
        if is_fresh {
            num += 1;
        }
    }
    num
}

fn add_ranges(r1: &Range, r2: &Range) -> Range {
    Range {
        from: r1.from.min(r2.from),
        to: r1.to.max(r2.to),
    }
}

fn merge_ranges(ranges: &mut Vec<Range>) {
    let mut i = 0;
    while i + 1 < ranges.len() {
        let curr = &ranges[i];
        let next = &ranges[i + 1];
        if curr.from <= next.from && next.from <= curr.to {
            ranges[i] = add_ranges(curr, next);
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }
}

fn insert_range(ranges: &mut Vec<Range>, new_range: Range) {
    let i = ranges
        .binary_search_by(|r| r.from.cmp(&new_range.from))
        .unwrap_or_else(|x| x);
    ranges.insert(i, new_range);
    merge_ranges(ranges);
}

fn part2(data: &Parsed) -> i64 {
    let mut merged_ranges: Vec<Range> = Vec::new();
    for range in &data.0 {
        insert_range(&mut merged_ranges, range.clone());
    }
    let mut num = 0;
    for range in &merged_ranges {
        num += range.to - range.from + 1;
    }
    num
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    let ranges = parts[0]
        .iter()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<Range>>();
    let ingredients = parts[1]
        .iter()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();
    (ranges, ingredients)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "3-5".into(),
            "10-14".into(),
            "16-20".into(),
            "12-18".into(),
            "".into(),
            "1234".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part2(&parse(&example())), 14);
    }
}
