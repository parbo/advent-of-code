use aoc::Grid;
use std::iter::*;

type Parsed = Vec<Vec<char>>;

fn part1(data: &Parsed) -> i64 {
    let sp = data
        .points()
        .find(|p| data.get_value(*p) == Some('S'))
        .unwrap();
    let mut beams = aoc::FxHashSet::default();
    beams.insert(sp);
    let mut num = 0;
    loop {
        let mut new_beams: aoc::FxHashSet<aoc::Point> = aoc::FxHashSet::default();
        for bp in &beams {
            let np = aoc::point_add(*bp, aoc::SOUTH);
            match data.get_value(np) {
                Some('.') => {
                    new_beams.insert(np);
                }
                Some('^') => {
                    num += 1;
                    let np1 = aoc::point_add(np, aoc::WEST);
                    if data.get_value(np1).is_some() {
                        new_beams.insert(np1);
                    }
                    let np2 = aoc::point_add(np, aoc::EAST);
                    if data.get_value(np2).is_some() {
                        new_beams.insert(np2);
                    }
                }
                _ => {}
            }
        }
        beams = new_beams;
        if beams.iter().any(|p| p[1] == (data.len() - 1) as i64) {
            break;
        }
    }
    num
}

fn part2(data: &Parsed) -> i64 {
    let sp = data
        .points()
        .find(|p| data.get_value(*p) == Some('S'))
        .unwrap();
    let mut beams = aoc::FxHashMap::default();
    beams.insert(sp, 1);
    loop {
        let mut new_beams: aoc::FxHashMap<aoc::Point, i64> = aoc::FxHashMap::default();
        for (bp, n) in &beams {
            let np = aoc::point_add(*bp, aoc::SOUTH);
            match data.get_value(np) {
                Some('.') => {
                    *new_beams.entry(np).or_default() += n;
                }
                Some('^') => {
                    let np1 = aoc::point_add(np, aoc::WEST);
                    if data.get_value(np1).is_some() {
                        *new_beams.entry(np1).or_default() += n;
                    }
                    let np2 = aoc::point_add(np, aoc::EAST);
                    if data.get_value(np2).is_some() {
                        *new_beams.entry(np2).or_default() += n;
                    }
                }
                _ => {}
            }
        }
        beams = new_beams;
        if beams.iter().any(|(p, _)| p[1] == (data.len() - 1) as i64) {
            break;
        }
    }
    beams.values().sum::<i64>()
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid(lines)
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
        assert_eq!(part1(&parse(&example())), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 40);
    }
}
