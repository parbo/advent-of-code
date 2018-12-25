use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;

// 2202 units each with 4950 hit points (weak to fire; immune to slashing) with an attack that does 18 cold damage at initiative 2
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Group {
    faction: String,
    num: i64,
    hp: i64,
    weak: Vec<String>,
    immune: Vec<String>,
    attack: String,
    dmg: i64,
    initiative: i64
}

impl FromStr for Group {
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
        Ok(Group { faction: "".to_string(), num, hp, weak, immune, attack, dmg, initiative })
    }
}

impl Group {
    fn effective_power(&self) -> i64 {
        self.num * self.dmg
    }

    fn is_immune(&self, attack: &String) -> bool {
        self.immune.contains(attack)
    }

    fn is_weak(&self, attack: &String) -> bool {
        self.weak.contains(attack)
    }

    fn damage(&self, defender: &Group) -> i64 {
        if defender.is_immune(&self.attack) {
            0
        } else if defender.is_weak(&self.attack) {
            2 * self.effective_power()
        } else {
            self.effective_power()
        }
    }
}

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut groups = vec![];
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
        let mut group = line.parse::<Group>().unwrap();
        if in_infection {
            group.faction = "infection".to_string();
        } else if in_immune {
            group.faction = "immune".to_string();
        }
        groups.push(group);
    }
    loop {
        // Selection phase
        groups.sort_by(|a, b| b.effective_power().cmp(&a.effective_power())
                       .then_with(|| b.initiative.cmp(&a.initiative)));
        let mut selection = HashMap::new();
        for (i, g) in groups.iter().enumerate() {
            let (j, _) = groups.iter().enumerate().filter(|(_, c)| g.faction != c.faction).max_by(|a, b| g.damage(&a.1).cmp(&g.damage(&b.1))).unwrap();
            selection.insert(i, j);
        }
        for (i, j) in selection {
            let d = groups[i].damage(&groups[j]);
            let killed = d / groups[j].hp;
            if killed < groups[j].num {
                groups[j].num -= killed;
            } else {
                groups[j].num = 0;
            }
        }
        let immune_count : i64 = groups.iter().filter(|c| c.faction == "immune".to_string()).map(|c| c.num).sum();
        let infection_count : i64 = groups.iter().filter(|c| c.faction == "infection".to_string()).map(|c| c.num).sum();
        if immune_count == 0 || infection_count == 0 {
            break;
        }
        println!("immune: {}, infection: {}", immune_count, infection_count);
    }
    for g in &groups {
        println!("{:?}", g);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
