use std::iter::*;

use aoc::Grid;

type Parsed = Vec<Vec<u32>>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    let mut visible = 0;
    'outer: for p in data.points() {
        let th = data.get_value(p).unwrap();
        for d in aoc::DIRECTIONS {
            let mut pos = p;
            loop {
                pos = aoc::point_add(pos, d);
                if let Some(oth) = data.get_value(pos) {
                    if oth >= th {
                        break;
                    }
                } else {
                    visible += 1;
                    continue 'outer;
                }
            }
        }
    }
    visible
}

fn part2(data: &Parsed) -> Answer {
    let mut scores = vec![];
    for p in data.points() {
        let th = data.get_value(p).unwrap();
        let mut score = 1;
        for d in aoc::DIRECTIONS {
            let mut pos = p;
            let mut dist = 0;
            loop {
                pos = aoc::point_add(pos, d);
                if let Some(oth) = data.get_value(pos) {
                    dist += 1;
                    if oth >= th {
                        score *= dist;
                        break;
                    }
                } else {
                    score *= dist;
                    break;
                }
            }
        }
        scores.push(score);
    }
    *scores.iter().max().unwrap() as i64
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid_to(lines, |x| x.to_digit(10).unwrap())
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "30373".into(),
            "25512".into(),
            "65332".into(),
            "33549".into(),
            "35390".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 8);
    }
}
