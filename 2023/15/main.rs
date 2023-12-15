use std::{collections::HashMap, iter::*};

type Parsed = Vec<String>;

fn hash(s: &str) -> i64 {
    s.chars()
        .fold(0, |acc, x| ((acc + (x as u8 as i64)) * 17) % 256)
}

fn part1(data: &Parsed) -> i64 {
    data.iter().map(|x| hash(x)).sum()
}

fn part2(data: &Parsed) -> i64 {
    let mut boxes: HashMap<i64, Vec<(String, i64)>> = HashMap::new();
    for d in data {
        if d.ends_with('-') {
            let label = &d[0..d.len() - 1];
            let h = hash(label);
            let v = boxes.entry(h).or_default();
            if let Some((i, _)) = v.iter().enumerate().find(|(_i, (l, _f))| l == label) {
                v.remove(i);
            }
        } else {
            let (label, focal_length) = d.split_once('=').unwrap();
            let focal_length = focal_length.parse::<i64>().unwrap();
            let h = hash(label);
            let v = boxes.entry(h).or_default();
            if let Some((i, _)) = v.iter().enumerate().find(|(_i, (l, _f))| l == label) {
                v[i] = (label.to_string(), focal_length);
            } else {
                v.push((label.to_string(), focal_length));
            }
        }
    }
    boxes
        .iter()
        .map(|(bx, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(ix, (_l, f))| (*bx + 1) * (ix as i64 + 1) * *f)
                .sum::<i64>()
        })
        .sum()
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_ch(&lines[0], ',')
        .iter()
        .map(|x| x.to_string())
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!["rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".into()]
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 145);
    }
}
