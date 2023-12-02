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
        for p in &self.picks {
            if let Some(c) = p.get("red") {
                if *c > 12 {
                    return false;
                }
            }
            if let Some(c) = p.get("green") {
                if *c > 13 {
                    return false;
                }
            }
            if let Some(c) = p.get("blue") {
                if *c > 14 {
                    return false;
                }
            }
        }
        true
    }

    fn power(&self) -> i64 {
        let max_red = self
            .picks
            .iter()
            .map(|x| *x.get("red").unwrap_or(&0))
            .max()
            .unwrap_or(0);
        let max_green = self
            .picks
            .iter()
            .map(|x| *x.get("green").unwrap_or(&0))
            .max()
            .unwrap_or(0);
        let max_blue = self
            .picks
            .iter()
            .map(|x| *x.get("blue").unwrap_or(&0))
            .max()
            .unwrap_or(0);
        max_red * max_green * max_blue
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
