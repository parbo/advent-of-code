use std::str::FromStr;
use std::{collections::BTreeMap, iter::*};

#[derive(Debug)]
struct Game {
    id: i64,
    picks: Vec<BTreeMap<String, i64>>,
}

impl FromStr for Game {
    type Err = aoc::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (gg, rest) = s.split_once(':').ok_or(Self::Err::Generic)?;
        let pp = aoc::split_ch(rest, ';');
        let mut picks = vec![];
        for p in pp {
            let mut m = BTreeMap::new();
            let cubes = aoc::split_ch(p, ',');
            for c in cubes {
                let vals = aoc::split_w(c);
                m.insert(
                    vals.get(1).ok_or(Self::Err::Generic)?.to_string(),
                    vals.first().ok_or(Self::Err::Generic)?.parse()?,
                );
            }
            picks.push(m);
        }
        let id = aoc::split_w(gg).get(1).ok_or(Self::Err::Generic)?.parse()?;

        Ok(Game { id, picks })
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        !self.picks.iter().any(|p| {
            *p.get("red").unwrap_or(&0) > 12
                || *p.get("green").unwrap_or(&0) > 13
                || *p.get("blue").unwrap_or(&0) > 14
        })
    }

    fn max_color(&self, color: &str) -> i64 {
        self.picks
            .iter()
            .map(|x| *x.get(color).unwrap_or(&0))
            .max()
            .unwrap_or(0)
    }

    fn power(&self) -> i64 {
        self.max_color("red") * self.max_color("green") * self.max_color("blue")
    }
}

type ParsedItem = Game;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    data.iter()
        .filter_map(|x| if x.is_valid() { Some(x.id) } else { None })
        .sum()
}

fn part2(data: &Parsed) -> i64 {
    data.iter().map(|x| x.power()).sum()
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
