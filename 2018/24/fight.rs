use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;
use std::str::FromStr;
use std::num::ParseIntError;

// 2202 units each with 4950 hit points (weak to fire; immune to slashing) with an attack that does 18 cold damage at initiative 2
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Unit {
    num: i64,
    hp: i64,
    weak: Vec<String>,
    immune: Vec<String>,
    attack: String,
    dmg: i64,
    initiative: i64
}

impl FromStr for Unit {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = s.find(" ").unwrap();
        let num = s[0..n].parse::<i64>()?;
        let mut weak = vec![];
        let mut immune = vec![];
        let mut hp_end = 0;
        if let Some(begin_paren) = s.find('(') {
            let end_paren = s.find(')').unwrap();
            let things: Vec<&str> = s[(begin_paren+1)..end_paren].split("; ").collect();
            for t in things {
                if t.starts_with("weak") {
                    weak = t[8..].split(", ").map(|c| c.to_string()).collect();
                } else if t.starts_with("immune") {
                    immune = t[10..].split(", ").map(|c| c.to_string()).collect();
                }
            }
            hp_end = begin_paren - 1;
        }
        let b = s.find("with an attack").unwrap() + 25;
        if hp_end == 0 {
            hp_end = b - 26;
        }
        let d = s[b..].find(" ").unwrap() + b;
        let e = s.find("at initiative").unwrap();
        let hp = s[n..hp_end].trim_start_matches(" units each with ").trim_end_matches(" hit points").parse::<i64>()?;
        let dmg = s[b..d].parse::<i64>()?;
        let initiative = s[(e+14)..].parse::<i64>()?;
        let attack = s[d..e].trim().to_string();
        Ok(Unit { num, hp, weak, immune, attack, dmg, initiative })
    }
}

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut immune = vec![];
    let mut infection = vec![];
    let mut in_immune = false;
    let mut in_infection = false;
    for line in lines {
        if line == "" {
            continue;
        }
        if line == "Immune System:" {
            in_immune = true;
            in_infection = false;
            continue;
        }
        if line == "Infection:" {
            in_infection = true;
            in_immune = false;
            continue;
        }
        let unit = line.parse::<Unit>().unwrap();
        println!("{:?}", unit);
        if in_infection {
            infection.push(unit);
        } else if in_immune {
            immune.push(unit);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
