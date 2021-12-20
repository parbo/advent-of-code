use aoc::*;
use std::collections::HashMap;
use std::iter::*;
use std::time::Instant;

type Parsed = (Vec<char>, HashMap<Point, char>);
type Answer = i64;

pub fn kernel(p: Point) -> impl Iterator<Item = Point> {
    let mut diter = [
        NORTH_WEST,
        NORTH,
        NORTH_EAST,
        WEST,
        [0, 0],
        EAST,
        SOUTH_WEST,
        SOUTH,
        SOUTH_EAST,
    ]
    .iter();
    from_fn(move || diter.next().map(|d| point_add(p, *d)))
}

fn part1(map: &Parsed) -> Answer {
    let mut g = map.1.clone();
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    gd.draw(&g);
    println!();
    for _i in 0..2 {
        let mut new_g = g.clone();
        let ([min_x, min_y], [max_x, max_y]) = g.extents();
        for x in (min_x - 3)..=(max_x + 3) {
            for y in (min_y - 3)..=(max_y + 3) {
                let p = [x, y];
                let mut ix = 0;
                for pp in kernel(p) {
                    let v = if let Some(x) = g.get_value(pp) {
                        if x == '#' {
                            1
                        } else {
                            0
                        }
                    } else {
                        0
                    };
                    ix <<= 1;
                    ix += v;
                }
                new_g.set_value(p, map.0[ix]);
            }
        }
        g = new_g;
        gd.draw(&g);
        println!();
    }
    g.iter().filter(|(_p, x)| **x == '#').count() as Answer
}

fn part2(_: &Parsed) -> Answer {
    0
}

fn parse(lines: &[String]) -> Parsed {
    let sections = aoc::split_by_empty_line(lines);
    let enhance = sections[0][0].chars().collect();
    let grid = aoc::parse_grid_to_sparse(&sections[1], |x| if x == '#' { Some(x) } else { None });
    (enhance, grid)
}

fn main() {
    let start_time = Instant::now();
    let (part, lines) = aoc::read_lines();
    let io_time = Instant::now();
    let parsed = parse(&lines);
    let parse_time = Instant::now();
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    let done_time = Instant::now();
    println!(
        "read: {:?}, parse: {:?}, solve: {:?}\n",
        io_time.duration_since(start_time),
        parse_time.duration_since(io_time),
        done_time.duration_since(parse_time)
    );
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        include_str!("sample.txt")
            .lines()
            .map(|x| x.into())
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 35);
    }
}
