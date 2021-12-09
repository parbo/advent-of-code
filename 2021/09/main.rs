use aoc::{Grid, GridDrawer};
use std::iter::*;

type Parsed = Vec<Vec<i64>>;
type Answer = i64;

fn part1(map: &Parsed) -> Answer {
    let mut low_points = vec![];
    'outer: for p in map.points() {
        let ph = map.get_value(p).unwrap();
        for d in aoc::DIRECTIONS {
            if let Some(h) = map.get_value(aoc::point_add(p, d)) {
                if ph >= h {
                    continue 'outer;
                }
            }
        }
        low_points.push(p);
    }
    low_points
        .iter()
        .map(|p| map.get_value(*p).unwrap())
        .map(|h| h + 1)
        .sum::<i64>()
}

fn part2(m: &Parsed, draw: bool) -> Answer {
    let mut map = m.clone();
    let mut basins = vec![];
    let ([min_x, min_y], [max_x, max_y]) = map.extents();
    let mut gd = aoc::BitmapGridDrawer::new(
        |x| match x {
            -1i64 => [0x79, 0xa2, 0xd8],
            9 => [0xff, 0xff, 0x66],
            x => [0, (x * 0x10 + 0x19) as u8, 0],
        },
        "ppm/day09",
    );
    for pos in map.points() {
        let value = map.get_value(pos).unwrap();
        if value == 9 && value == -1 {
            continue;
        }
        let mut num = 0;
        let mut todo = vec![];
        todo.push(pos);
        while let Some(p) = todo.pop() {
            if let Some(curr) = map.get_value(p) {
                if curr != 9 && curr != -1 {
                    num += 1;
                    map.set_value(p, -1);
                    if p[0] > min_x {
                        todo.push([p[0] - 1, p[1]]);
                    }
                    if p[0] < max_x {
                        todo.push([p[0] + 1, p[1]]);
                    }
                    if p[1] > min_y {
                        todo.push([p[0], p[1] - 1]);
                    }
                    if p[1] < max_y {
                        todo.push([p[0], p[1] + 1]);
                    }
		    if draw {
			gd.draw(&map);
			gd.save_image();
		    }
                }
            }
        }
        basins.push(num);
    }
    let mut d = aoc::PrintGridDrawer::new(|i| {
        if i == -1 {
            '*'
        } else {
            format!("{}", i).chars().next().unwrap()
        }
    });
    d.draw(&map);
    basins.sort();
    basins.iter().rev().take(3).product()
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid_to(lines, |x| x.to_digit(10).unwrap().into())
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else if part == 2 {
	part2(&parsed, false)
    } else {
        part2(&parsed, true)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse(&vec![
                "2199943210".into(),
                "3987894921".into(),
                "9856789892".into(),
                "8767896789".into(),
                "9899965678".into()
            ])),
            1134
        );
    }
}
