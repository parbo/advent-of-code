use aoc::Grid;

type Parsed = Vec<Vec<char>>;

fn solve(data: &Parsed, threshold: i64, cheats: i64) -> i64 {
    let s = data
        .points()
        .find(|p| data.get_value(*p) == Some('S'))
        .unwrap();
    let e = data
        .points()
        .find(|p| data.get_value(*p) == Some('E'))
        .unwrap();
    let path = aoc::dijkstra_grid(data, |_, c| *c != '#', |_, _, _, _| Some(1), s, e)
        .unwrap()
        .1;
    let mut num = 0;
    for (i, p1) in path.iter().enumerate() {
        for (j, p2) in path[i..].iter().enumerate() {
            let d = aoc::manhattan(*p1, *p2);
            if d <= cheats && j as i64 - d >= threshold {
                num += 1;
            }
        }
    }
    num
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 100, 2)
}

fn part2(data: &Parsed) -> i64 {
    solve(data, 100, 20)
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
        assert_eq!(solve(&parse(&example()), 1, 2), 44);
    }
}
