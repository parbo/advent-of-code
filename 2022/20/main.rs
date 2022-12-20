use std::iter::*;

type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;

fn solve(data: &Parsed, multiplier: i64, rounds: usize) -> i64 {
    let mut mixed = data
        .iter()
        .map(|x| x * multiplier)
        .enumerate()
        .collect::<Vec<_>>();
    for _round in 0..rounds {
        for i in 0..mixed.len() {
            let mut pos = mixed.iter().position(|(ix, _v)| *ix == i).unwrap() as i64;
            let (ix, v) = mixed.remove(pos as usize);
            let sz = mixed.len() as i64;
            pos += v % sz;
            while pos < 0 {
                pos += sz;
            }
            while pos > sz {
                pos -= sz;
            }
            mixed.insert(pos as usize, (ix, v));
        }
    }
    let pos = mixed.iter().position(|(_ix, v)| *v == 0).unwrap();
    mixed[(pos + 1000) % mixed.len()].1
        + mixed[(pos + 2000) % mixed.len()].1
        + mixed[(pos + 3000) % mixed.len()].1
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 1, 1)
}

fn part2(data: &Parsed) -> i64 {
    solve(data, 811589153, 10)
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
