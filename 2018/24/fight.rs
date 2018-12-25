use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;
use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;
use std::collections::HashSet;

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
        let e = s.find("damage at initiative").unwrap();
        let hp = s[n..hp_end].trim_start_matches(" units each with ").trim_end_matches(" hit points").parse::<i64>()?;
        let dmg = s[b..d].parse::<i64>()?;
        let initiative = s[(e+21)..].parse::<i64>()?;
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

fn solve(path: &Path, b: i64) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut boost = b;
    loop {
        println!("boost: {}", boost);
        let mut groups = vec![];
        let mut in_immune = false;
        let mut in_infection = false;
        for line in &lines {
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
                group.dmg += boost;
            }
            groups.push(group);
        }
        let mut initiative_indices : Vec<usize> = (0..groups.len()).collect();
        initiative_indices.sort_by(|&a, &b| groups[b].initiative.cmp(&groups[a].initiative));
        loop {
            // Selection phase
            let mut selection = HashMap::new();
            let mut selected = HashSet::new();
            let mut indices : Vec<usize> = (0..groups.len()).collect();
            indices.sort_by(|&a, &b| groups[b].effective_power().cmp(&groups[a].effective_power())
                            .then_with(|| groups[b].initiative.cmp(&groups[a].initiative)));
            for i in indices.iter().cloned() {
                if groups[i].num == 0 {
                    continue;
                }
                let g = &groups[i];
                if let Some(j) = indices.iter().cloned()
                    .filter(|&c| !selected.contains(&c) && g.faction != groups[c].faction && groups[c].num > 0)
                    .max_by(|&a, &b| g.damage(&groups[a]).cmp(&g.damage(&groups[b]))
                            .then_with(|| groups[a].effective_power().cmp(&groups[b].effective_power()))
                            .then_with(|| groups[a].initiative.cmp(&groups[b].initiative))) {
                        if g.damage(&groups[j]) > 0 {
                            selection.insert(i, j);
                            selected.insert(j);
                        }
                    }
            }
            // Attack phase
            let mut attacks = 0;
            let mut kills = 0;
            for i in initiative_indices.iter().cloned() {
                if groups[i].num == 0 {
                    continue;
                }
                if let Some(&j) = selection.get(&i) {
                    let d = groups[i].damage(&groups[j]);
                    let killed = std::cmp::min(d / groups[j].hp, groups[j].num);
                    groups[j].num -= killed;
                    attacks += 1;
                    kills += killed;
                }
            }
            let immune_count : i64 = groups.iter().filter(|c| c.faction == "immune".to_string()).map(|c| c.num).sum();
            let infection_count : i64 = groups.iter().filter(|c| c.faction == "infection".to_string()).map(|c| c.num).sum();
//            println!("{}, {}", immune_count, infection_count);
            if immune_count == 0 || infection_count == 0 {
                if immune_count == 0 {
                    println!("infection wins: {}", infection_count);
                } else if infection_count == 0 {
                    println!("immune wins: {}", immune_count);
                    return;
                }
                break;
            }
            if attacks == 0 || kills == 0 {
                println!("this fight won't end");
                break;
            }
        }
        boost += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let boost = args[2].parse::<i64>().unwrap();

    solve(Path::new(&filename), boost);
}
