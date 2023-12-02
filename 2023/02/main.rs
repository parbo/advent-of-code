use std::num::ParseIntError;
use std::str::FromStr;
use std::{collections::BTreeMap, iter::*};

#[derive(Debug)]
struct Game {
    id: i64,
    picks: Vec<BTreeMap<String, i64>>,
}

impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let gg = aoc::split_ch(s, ':');
        let pp = aoc::split_ch(gg[gg.len() - 1], ';');
        let mut picks = vec![];
        for p in pp {
            let mut m = BTreeMap::new();
            let cubes = aoc::split_ch(p, ',');
            for c in cubes {
                let vals = aoc::split_w(c);
                m.insert(vals[1].to_string(), vals[0].parse()?);
            }
            picks.push(m);
        }
        let id = aoc::split_w(gg[0])[1].parse()?;

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
