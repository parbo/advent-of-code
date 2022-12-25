use std::iter::*;

type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    data.windows(2).filter(|a| a[1] > a[0]).count() as i64
}

fn part2(data: &Parsed) -> Answer {
    let sliding: Vec<_> = data.windows(3).map(|a| a.iter().sum()).collect();
    part1(&sliding)
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        );
    }
}
