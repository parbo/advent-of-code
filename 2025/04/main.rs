use std::iter::*;

use aoc::Grid;

type Parsed = Vec<Vec<char>>;

fn part1(data: &Parsed) -> i64 {
    let mut ans = 0;
    for p in data.points() {
        let mut num = 0;
        if data.get_value(p) == Some('.') {
            continue;
        }
        for nb in aoc::neighbors_incl_diagonals(p) {
            if let Some(v) = data.get_value(nb) {
                if v == '@' {
                    num += 1;
                }
            }
        }
        if num < 4 {
            ans += 1;
        }
    }
    ans
}

fn part2(data: &Parsed) -> i64 {
    let mut ans = 0;
    let mut g = data.clone();
    loop {
        let mut to_remove = vec![];
        for p in g.points() {
            let mut num = 0;
            if g.get_value(p) == Some('.') {
                continue;
            }
            for nb in aoc::neighbors_incl_diagonals(p) {
                if let Some(v) = g.get_value(nb) {
                    if v == '@' {
                        num += 1;
                    }
                }
            }
            if num < 4 {
                ans += 1;
                to_remove.push(p);
            }
        }
        if to_remove.is_empty() {
            break;
        }
        for p in to_remove {
            g.set_value(p, '.');
        }
    }
    ans
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
        assert_eq!(part1(&parse(&example())), 13);
    }
}
