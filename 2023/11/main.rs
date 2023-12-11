use aoc::Grid;
use itertools::Itertools;
use std::iter::*;

type Parsed = Vec<Vec<char>>;

fn solve(data: &Parsed, n: i64) -> i64 {
    let empty_rows = (0..data.len())
        .filter(|y| {
            (0..data[0].len()).all(|x| data.get_value([x as i64, *y as i64]).unwrap() == '.')
        })
        .collect::<Vec<_>>();
    let empty_columns = (0..data[0].len())
        .filter(|x| (0..data.len()).all(|y| data.get_value([*x as i64, y as i64]).unwrap() == '.'))
        .collect::<Vec<_>>();
    let mut galaxies = data
        .points()
        .filter(|p| data.get_value(*p).unwrap() == '#')
        .collect::<Vec<_>>();
    for row in empty_rows.iter().rev() {
        for g in &mut galaxies {
            if g[1] >= *row as i64 {
                g[1] += n - 1;
            }
        }
    }
    for column in empty_columns.iter().rev() {
        for g in &mut galaxies {
            if g[0] >= *column as i64 {
                g[0] += n - 1;
            }
        }
    }
    galaxies
        .iter()
        .permutations(2)
        .unique()
        .map(|x| aoc::manhattan(*x[0], *x[1]))
        .sum::<i64>()
        / 2
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 2)
}

fn part2(data: &Parsed) -> i64 {
    solve(data, 1000000)
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "...#......".into(),
            ".......#..".into(),
            "#.........".into(),
            "..........".into(),
            "......#...".into(),
            ".#........".into(),
            ".........#".into(),
            "..........".into(),
            ".......#..".into(),
            "#...#.....".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 374);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(solve(&parse(&example()), 2), 374);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(solve(&parse(&example()), 10), 1030);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(solve(&parse(&example()), 100), 8410);
    }
}
