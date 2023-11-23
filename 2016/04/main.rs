use std::{collections::BTreeMap, iter::*};

#[derive(Debug)]
struct Room {
    name: String,
    sector: i64,
    checksum: String,
}

impl Room {
    fn checksum(&self) -> String {
        let mut counts = BTreeMap::new();
        for c in self.name.chars() {
            if c == '-' {
                continue;
            }
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut chk: Vec<_> = counts.into_iter().collect();
        chk.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        chk.into_iter().take(5).map(|x| x.0).collect()
    }

    fn decrypt(&self) -> String {
        let mut r = String::new();
        for c in self.name.chars() {
            if c == '-' {
                r.push(' ')
            } else {
                r.push(
                    char::from_u32(
                        (((c.to_ascii_lowercase() as u8 - b'a') as u32 + self.sector as u32) % 26)
                            + 'a' as u32,
                    )
                    .unwrap(),
                )
            }
        }
        r
    }
}

type ParsedItem = Room;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    data.iter()
        .filter(|x| x.checksum() == x.checksum)
        .map(|x| x.sector)
        .sum()
}

fn part2(data: &Parsed) -> i64 {
    data.iter()
        .filter(|x| x.checksum() == x.checksum)
        .map(|x| (x, x.decrypt()))
        .filter(|(_, decrypted)| decrypted == "northpole object storage")
        .map(|(x, _)| x.sector)
        .next()
        .unwrap()
}

fn parse(lines: &[String]) -> Parsed {
    let rx = regex::Regex::new(r"(.*?)-(\d+)\[(.*?)\]").unwrap();
    lines
        .iter()
        .map(|x| {
            let caps = rx.captures(x).unwrap();
            Room {
                name: caps[1].into(),
                sector: caps[2].parse::<i64>().unwrap(),
                checksum: caps[3].into(),
            }
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn example() -> Vec<String> {
    //     let s = include_str!("example.txt");
    //     s.lines().map(|x| x.to_string()).collect()
    // }

    // fn example() -> Vec<String> {
    // 	   vec![
    //         "0".into()
    //     ]
    // }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&parse(&example())), 0);
    // }
}
