use std::iter::*;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Triangle {
    a: i64,
    b: i64,
    c: i64,
}

impl FromStr for Triangle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lengths: Vec<&str> = aoc::split_w(s.trim()).into_iter().collect();
        let a = lengths[0].trim().parse::<i64>()?;
        let b = lengths[1].trim().parse::<i64>()?;
        let c = lengths[2].trim().parse::<i64>()?;
        Ok(Triangle { a, b, c })
    }
}

impl Triangle {
    fn is_valid(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.b + self.c > self.a
    }
}

type ParsedItem = Triangle;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    data.iter().filter(|x| x.is_valid()).count() as i64
}

fn transpose(a: &mut [Triangle]) {
    std::mem::swap(&mut a[0].b, &mut a[1].a);
    std::mem::swap(&mut a[2].a, &mut a[0].c);
    std::mem::swap(&mut a[2].b, &mut a[1].c);
}

fn part2(data: &Parsed) -> i64 {
    let mut d = data.clone();
    d.chunks_mut(3).for_each(|x| transpose(x));
    part1(&d)
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
