use std::{collections::HashMap, iter::*};

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
    let mut grid = HashMap::new();
    for s in data {
        let sp = [s.sx, s.sy];
        let bp = [s.bx, s.by];
        grid.insert(sp, 'S');
        grid.insert(bp, 'B');
        let mh = aoc::manhattan(sp, bp);
        let y = row;
        for x in (s.sx - mh)..(s.sx + mh) {
            let mhp = aoc::manhattan(sp, [x, y]);
            if mhp <= mh {
                grid.entry([x, y]).or_insert('#');
            }
        }
    }
    grid.iter()
        .filter(|(p, c)| **c == '#' && p[1] == row)
        .count()
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
    for y in 0..4000000i64 {
        let mut ranges = vec![];
        for &(s, mh) in &sensors {
            let dy = s.sy.abs_diff(y) as i64;
            if dy > mh {
                // This sensor not in range of this row
                continue;
            }
            let d = mh - dy;
            let rg = ((s.sx - d).clamp(0, 4000000), (s.sx + d).clamp(0, 4000000));
            if rg.0 != rg.1 {
                ranges.push(rg);
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
