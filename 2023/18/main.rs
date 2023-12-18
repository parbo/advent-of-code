use std::iter::*;

type DataItem = (aoc::Point, i64);
type Data = Vec<DataItem>;
type Parsed = Vec<String>;

fn parse1(lines: &[String]) -> Data {
    lines
        .iter()
        .map(|s| {
            let parts = aoc::split_w(s);
            let dir = match parts[0] {
                "L" => aoc::WEST,
                "R" => aoc::EAST,
                "U" => aoc::NORTH,
                "D" => aoc::SOUTH,
                _ => panic!(),
            };
            let num = parts[1].parse::<i64>().unwrap();
            (dir, num)
        })
        .collect()
}

fn parse2(lines: &[String]) -> Data {
    lines
        .iter()
        .map(|s| {
            let parts = aoc::split_w(s);
            let num = i64::from_str_radix(&parts[2][2..7], 16).unwrap();
            let dir = match parts[2].chars().nth(7).unwrap() {
                '0' => aoc::EAST,
                '1' => aoc::SOUTH,
                '2' => aoc::WEST,
                '3' => aoc::NORTH,
                _ => panic!(),
            };
            (dir, num)
        })
        .collect()
}

fn solve(data: &Data) -> i64 {
    let mut pos = [0, 0];
    let mut vertices = vec![pos];
    let mut garea = 0;
    for d in data {
        for _ in 0..d.1 {
            pos = aoc::point_add(pos, d.0);
            vertices.push(pos);
            garea += 1;
        }
    }

    //shoelace formula
    let mut area = 0;
    let n = vertices.len() as i64;
    for p in vertices.windows(2) {
        area += p[0][1] * p[1][0];
        area -= p[0][0] * p[1][1];
    }
    let area = i64::abs(area) / 2;
    let area = area - (n / 2) + 1;
    garea + area
}

fn part1(lines: &Parsed) -> i64 {
    solve(&parse1(lines))
}

fn part2(lines: &Parsed) -> i64 {
    solve(&parse2(lines))
}

fn parse(lines: &[String]) -> Parsed {
    lines.to_vec()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn example() -> Vec<String> {
    //     let s = include_str!("example.txt");
    //     s.lines().map(|x| x.to_string()).collect()
    // }

    // fn example() -> Vec<String> {
    // 	   vec![
    //         "0".into()
    //     ]
    // }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&parse(&example())), 0);
    // }
}
