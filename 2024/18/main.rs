use std::iter::*;

use aoc::Grid;

#[cfg(feature = "vis")]
use aoc::GridDrawer;

type ParsedItem = aoc::Point;
type Parsed = Vec<ParsedItem>;

fn solve(data: &Parsed, num: usize, w: usize, h: usize) -> Option<(i64, Vec<aoc::Point>)> {
    let mut g = vec![vec!['.'; w + 1]; h + 1];
    for p in data.iter().take(num) {
        g.set_value(*p, '#');
    }
    let a: aoc::Point = [0, 0];
    let f: aoc::Point = [w as i64, h as i64];

    aoc::astar_grid(&g, |_p, c| *c != '#', |_p1, _c1, _p2, _c2| Some(1), a, f)
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 1024, 70, 70).unwrap().0
}

fn part2(data: &Parsed) -> i64 {
    #[cfg(feature = "vis")]
    let mut gd = aoc::make_bitmap_text_grid_drawer(
        |x| match x {
            '#' => (x, [0xff, 0xff, 0xff]),
            '*' => (x, [0xff, 0x0, 0]),
            _ => (x, [0, 0x5f, 0]),
        },
        "vis/18/day18",
    );
    let mut lastp = vec![];
    for i in 1..=data.len() {
        if !lastp.is_empty() && !lastp.contains(&data[i - 1]) {
            // safe
            continue;
        }
        let s = solve(data, i, 70, 70);
        if let Some((_, path)) = &s {
            lastp = path.clone();
            #[cfg(feature = "vis")]
            {
                let mut g = vec![vec!['.'; 71]; 71];
                for p in data.iter().take(i) {
                    g.set_value(*p, '#');
                }
                for p in path {
                    g.set_value(*p, '*');
                }
                gd.draw(&g);
            }
        }
        if s.is_none() {
            println!("{},{}", data[i - 1][0], data[i - 1][1]);
            break;
        }
    }
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let p = aoc::split_ch(x, ',');
            [p[0].parse().unwrap(), p[1].parse().unwrap()]
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
        assert_eq!(solve(&parse(&example()), 12, 6, 6), 22);
    }
}
