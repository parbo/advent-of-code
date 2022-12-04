use std::iter::*;

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("{a}-{b},{c}-{d}")]
struct Assignment {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

impl Assignment {
    fn contains(&self) -> bool {
        self.a <= self.c && self.d <= self.b || self.c <= self.a && self.b <= self.d
    }
    fn overlaps(&self) -> bool {
        !(self.c > self.b || self.a > self.d)
    }
}

type ParsedItem = Assignment;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    data.iter().filter(|x| x.contains()).count() as i64
}

fn part2(data: &Parsed) -> Answer {
    data.iter().filter(|x| x.overlaps()).count() as i64
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
