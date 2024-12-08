use std::iter::*;

use aoc::Grid;

type ParsedItem = Vec<char>;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let mut unique: aoc::FxHashSet<aoc::Point> = aoc::FxHashSet::default();
    data.points()
        .filter(|p| data.get_value(*p).unwrap() != '.')
        .for_each(|p| {
            let v = data.get_value(p).unwrap();
            for pp in data
                .points()
                .filter(|pp| data.get_value(*pp).unwrap() == v && p != *pp)
            {
                let diff = aoc::point_sub(pp, p);
                let a = aoc::point_add(aoc::point_add(p, diff), diff);
                if data.get_value(a).is_some() {
                    unique.insert(a);
                }
                let diff = aoc::point_sub(p, pp);
                let a = aoc::point_add(aoc::point_add(pp, diff), diff);
                if data.get_value(a).is_some() {
                    unique.insert(a);
                }
            }
        });
    unique.len() as i64
}

fn part2(data: &Parsed) -> i64 {
    let mut unique: aoc::FxHashSet<aoc::Point> = aoc::FxHashSet::default();
    data.points()
        .filter(|p| data.get_value(*p).unwrap() != '.')
        .for_each(|p| {
            let v = data.get_value(p).unwrap();
            for pp in data
                .points()
                .filter(|pp| data.get_value(*pp).unwrap() == v && p != *pp)
            {
                let diff = aoc::point_sub(pp, p);
                let mut a = p;
                loop {
                    a = aoc::point_add(a, diff);
                    if data.get_value(a).is_some() {
                        unique.insert(a);
                    } else {
                        break;
                    }
                }
                let diff = aoc::point_sub(p, pp);
                let mut a = pp;
                loop {
                    a = aoc::point_add(a, diff);
                    if data.get_value(a).is_some() {
                        unique.insert(a);
                    } else {
                        break;
                    }
                }
            }
        });
    unique.len() as i64
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
        vec![
            "............".into(),
            "........0...".into(),
            ".....0......".into(),
            ".......0....".into(),
            "....0.......".into(),
            "......A.....".into(),
            "............".into(),
            "............".into(),
            "........A...".into(),
            ".........A..".into(),
            "............".into(),
            "............".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 34);
    }
}
