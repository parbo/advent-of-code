use std::iter::*;

use aoc::Grid;

type Parsed = (Vec<Vec<usize>>, Vec<Vec<usize>>, usize);

fn part1(data: &Parsed) -> i64 {
    let mut n = 0;
    for key in &data.1 {
        for lock in &data.0 {
            if key
                .iter()
                .zip(lock.iter())
                .map(|(a, b)| a + b)
                .all(|x| x <= data.2)
            {
                n += 1;
            }
        }
    }
    n
}

fn part2(_: &Parsed) -> i64 {
    0
}

fn get_rows(g: &Vec<Vec<char>>) -> Vec<usize> {
    (0..g[0].len())
        .map(|x| {
            (0..g.len())
                .filter(|y| g.get_value([x as i64, *y as i64]) == Some('#'))
                .count()
        })
        .collect()
}

fn parse(lines: &[String]) -> Parsed {
    let all: Vec<_> = aoc::split_by_empty_line(lines)
        .iter()
        .map(aoc::parse_grid)
        .collect();
    (
        all.iter()
            .filter(|g| g.get_value([0, 0]) == Some('#'))
            .map(get_rows)
            .collect(),
        all.iter()
            .filter(|g| g.get_value([0, 0]) == Some('.'))
            .map(get_rows)
            .collect(),
        all[0].len(),
    )
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
        assert_eq!(part1(&parse(&example())), 3);
    }
}
