use std::collections::HashSet;
use std::time::Instant;

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("target area: x={min_x}..{max_x}, y={min_y}..{max_y}")]
struct Area {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

type Parsed = Area;
type Answer = i64;

fn shoot(mut x: i64, mut y: i64, area: &Area) -> Option<(i64, aoc::Point)> {
    let mut p = [0, 0];
    let mut max_y = 0;
    loop {
        p[0] += x;
        p[1] += y;
        if x > 0 {
            x -= 1;
        } else if x < 0 {
            x += 1;
        }
        y -= 1;
        max_y = p[1].max(max_y);
        if aoc::inside_extent(p, ([area.min_x, area.min_y], [area.max_x, area.max_y])) {
            return Some((max_y, p));
        }
        if p[1] < area.min_y {
            break;
        }
        if x > 0 && p[0] > area.max_x {
            break;
        }
        if x < 0 && p[0] < area.min_x {
            break;
        }
    }
    None
}

fn part1(area: &Parsed) -> Answer {
    let mut max_y = 0;
    let mut y = 1;
    loop {
        let mut x = 1;
        loop {
            for (v_x, v_y) in [(x, y), (-x, y), (x, -y), (-x, -y)] {
		if let Some((m_y, _p)) = shoot(v_x, v_y, area) {
		    max_y = m_y.max(max_y);
		}
            }
            x += 1;
            if max_y > 0 && x > 1000 {
                break;
            }
        }
        y += 1;
        if max_y > 0 && y > 1000 {
            break;
        }
    }
    max_y
}

fn solve2(area: &Parsed) -> HashSet<(i64, i64)> {
    let mut found = HashSet::new();
    let mut y = 0;
    let mut any_found;
    loop {
        let mut x = 0;
        let mut any_found_y = false;
        loop {
            any_found = false;
            for (v_x, v_y) in [(x, y), (-x, y), (x, -y), (-x, -y)] {
                if let Some((_m_y, _p)) = shoot(v_x, v_y, area) {
                    if found.insert((v_x, v_y)) {
                        any_found = true;
                    }
                }
            }
            x += 1;
            if any_found {
                any_found_y = true;
            } else if x > 1000 {
                break;
            }
        }
        if any_found_y {
        } else if y > 1000 {
            break;
        }
        y += 1;
    }
    found
}

fn part2(area: &Parsed) -> Answer {
    let found = solve2(area);
    found.len() as Answer
}

fn parse(lines: &[String]) -> Parsed {
    lines[0].parse().unwrap()
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
        vec!["target area: x=20..30, y=-10..-5".into()]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 45);
    }

    #[test]
    fn test_part2() {
        let expected = vec![
            (23, -10),
            (25, -9),
            (27, -5),
            (29, -6),
            (22, -6),
            (21, -7),
            (9, 0),
            (27, -7),
            (24, -5),
            (25, -7),
            (26, -6),
            (25, -5),
            (6, 8),
            (11, -2),
            (20, -5),
            (29, -10),
            (6, 3),
            (28, -7),
            (8, 0),
            (30, -6),
            (29, -8),
            (20, -10),
            (6, 7),
            (6, 4),
            (6, 1),
            (14, -4),
            (21, -6),
            (26, -10),
            (7, -1),
            (7, 7),
            (8, -1),
            (21, -9),
            (6, 2),
            (20, -7),
            (30, -10),
            (14, -3),
            (20, -8),
            (13, -2),
            (7, 3),
            (28, -8),
            (29, -9),
            (15, -3),
            (22, -5),
            (26, -8),
            (25, -8),
            (25, -6),
            (15, -4),
            (9, -2),
            (15, -2),
            (12, -2),
            (28, -9),
            (12, -3),
            (24, -6),
            (23, -7),
            (25, -10),
            (7, 8),
            (11, -3),
            (26, -7),
            (7, 1),
            (23, -9),
            (6, 0),
            (22, -10),
            (27, -6),
            (8, 1),
            (22, -8),
            (13, -4),
            (7, 6),
            (28, -6),
            (11, -4),
            (12, -4),
            (26, -9),
            (7, 4),
            (24, -10),
            (23, -8),
            (30, -8),
            (7, 0),
            (9, -1),
            (10, -1),
            (26, -5),
            (22, -9),
            (6, 5),
            (7, 5),
            (23, -6),
            (28, -10),
            (10, -2),
            (11, -1),
            (20, -9),
            (14, -2),
            (29, -7),
            (13, -3),
            (23, -5),
            (24, -8),
            (27, -9),
            (30, -7),
            (28, -5),
            (21, -10),
            (7, 9),
            (6, 6),
            (21, -5),
            (27, -10),
            (7, 2),
            (30, -9),
            (21, -8),
            (22, -7),
            (24, -9),
            (20, -6),
            (6, 9),
            (29, -5),
            (8, -2),
            (27, -8),
            (30, -5),
            (24, -7),
        ];
        let parsed = parse(&example());
        let found = solve2(&parsed);
        for f in expected {
            assert!(found.contains(&f), "{:?} not found!", f);
        }
    }
}
