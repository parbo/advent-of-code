use std::iter::*;

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("Button {n}: X+{x}, Y+{y}")]
struct Button {
    n: char,
    x: i64,
    y: i64,
}

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("Prize: X={x}, Y={y}")]
struct Prize {
    x: i64,
    y: i64,
}

type ParsedItem = (Button, Button, Prize);
type Parsed = Vec<ParsedItem>;

fn solve(x: i64, y: i64, ax: i64, ay: i64, bx: i64, by: i64) -> Option<i64> {
    let ma = [[ax, bx], [ay, by]];
    let det = ma[0][0] * ma[1][1] - ma[0][1] * ma[1][0];
    let inv = [[ma[1][1], -ma[0][1]], [-ma[1][0], ma[0][0]]];
    let numa = inv[0][0] * x + inv[0][1] * y;
    let numb = inv[1][0] * x + inv[1][1] * y;
    if numa % det == 0 && numb % det == 0 {
        Some(3 * numa / det + numb / det)
    } else {
        None
    }
}

fn part1(data: &Parsed) -> i64 {
    let mut cost = 0;
    for (a, b, p) in data {
        cost += solve(p.x, p.y, a.x, a.y, b.x, b.y).unwrap_or(0);
    }
    cost
}

fn part2(data: &Parsed) -> i64 {
    let mut cost = 0;
    for (a, b, p) in data {
        cost += solve(
            10000000000000 + p.x,
            10000000000000 + p.y,
            a.x,
            a.y,
            b.x,
            b.y,
        )
        .unwrap_or(0);
    }
    cost
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    parts
        .iter()
        .map(|x| {
            (
                x[0].parse::<Button>().unwrap(),
                x[1].parse::<Button>().unwrap(),
                x[2].parse::<Prize>().unwrap(),
            )
        })
        .collect()
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
        assert_eq!(part1(&parse(&example())), 480);
    }
}
