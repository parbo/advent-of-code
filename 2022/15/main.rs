use std::{collections::HashSet, iter::*};

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("Sensor at x={sx}, y={sy}: closest beacon is at x={bx}, y={by}")]
struct Sensor {
    sx: i64,
    sy: i64,
    bx: i64,
    by: i64,
}

type ParsedItem = Sensor;
type Parsed = Vec<ParsedItem>;
type Answer = usize;

fn solve(data: &Parsed, row: i64) -> Answer {
    let mut ranges = vec![];
    let mut sb = HashSet::new();
    for s in data {
        let sp = [s.sx, s.sy];
        let bp = [s.bx, s.by];
        if s.by == row {
            sb.insert(s.bx);
        }
        let mh = aoc::manhattan(sp, bp);
        let dy = s.sy.abs_diff(row) as i64;
        if dy <= mh {
            let d = mh - dy;
            ranges.push(((s.sx - d), (s.sx + d)));
        }
    }
    ranges.sort();
    let mut c = -(sb.len() as i64);
    let mut s = ranges[0].0;
    let mut e = ranges[0].1;
    for rg in &ranges[1..] {
        if rg.0 > e {
            c += e - s + 1;
            s = rg.0;
            e = rg.1;
        } else if rg.1 > s {
            e = e.max(rg.1);
        }
    }
    c += e - s + 1;
    c as usize
}

fn part1(data: &Parsed) -> Answer {
    solve(data, 2000000)
}

fn part2(data: &Parsed) -> i64 {
    let sensors = data
        .iter()
        .map(|s| {
            let mh = aoc::manhattan([s.sx, s.sy], [s.bx, s.by]);
            (s, mh)
        })
        .collect::<Vec<_>>();
    let mut ranges = vec![];
    for y in 0..4000000i64 {
        for &(s, mh) in &sensors {
            let dy = s.sy.abs_diff(y) as i64;
            if dy <= mh {
                let d = mh - dy;
                ranges.push(((s.sx - d).clamp(0, 4000000), (s.sx + d).clamp(0, 4000000)));
            }
        }
        ranges.sort();
        let mut i = 0;
        while i + 1 < ranges.len() {
            let a = ranges[i];
            let b = ranges[i + 1];
            if a.0 <= b.0 && a.1 >= b.1 {
                // b is contained
                ranges.remove(i + 1);
            } else {
                i += 1;
            }
        }
        if let Some(gap) = ranges.windows(2).find(|g| g[0].1 < g[1].0) {
            return (gap[0].1 + 1) * 4000000 + y;
        }
        ranges.clear();
    }
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&parse(&example()), 10), 26);
    }
}
