use aoc::{Grid, GridDrawer};
use std::collections::HashMap;
use std::time::Instant;

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("target area: x={min_x}..{max_x}, y={min_y}..{max_y}")]
struct Area {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

type Answer = i64;

#[allow(clippy::comparison_chain)]
fn shoot(mut x: i64, mut y: i64, area: &Area) -> (Option<i64>, Vec<aoc::Point>) {
    let mut p = [0, 0];
    let mut max_y = 0;
    let mut path = vec![];
    loop {
        p[0] += x;
        p[1] += y;
        if x > 0 {
            x -= 1;
        } else if x < 0 {
            x += 1;
        }
        y -= 1;
        path.push(p);
        max_y = p[1].max(max_y);
        if aoc::inside_extent(p, ([area.min_x, area.min_y], [area.max_x, area.max_y])) {
            return (Some(max_y), path);
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
    (None, path)
}

fn get_bounds(area: &Area) -> (i64, i64, i64, i64) {
    let min_dist_x = if area.min_x > 0 {
        area.min_x
    } else {
        area.max_x
    };
    let max_dist_x = if area.max_x > 0 {
        area.max_x
    } else {
        area.min_x
    };
    // Solve n*(n+1)/2 = dist
    let min_vx = ((1.0 + (1.0 + 8.0 * (min_dist_x as f64)).sqrt()) / 2.0).floor() as i64;
    let max_vx = if area.min_x > 0 {
        max_dist_x
    } else {
        -max_dist_x
    };
    let min_vy = area.min_y;
    // Is this correct even if area.min_y is > 0?
    let max_vy = -area.min_y;
    (min_vx, min_vy, max_vx, max_vy)
}

fn part1(area: &Area) -> Answer {
    let mut max_y = 0;
    let (min_vx, min_vy, max_vx, max_vy) = get_bounds(area);
    for v_y in min_vy..=max_vy {
        for v_x in min_vx..=max_vx {
            if let Some(m_y) = shoot(v_x, v_y, area).0 {
                max_y = m_y.max(max_y);
            }
        }
    }
    max_y
}

fn solve2(area: &Area) -> HashMap<(i64, i64), Vec<aoc::Point>> {
    let mut found = HashMap::new();
    let (min_vx, min_vy, max_vx, max_vy) = get_bounds(area);
    for v_y in min_vy..=max_vy {
        for v_x in min_vx..=max_vx {
            let r = shoot(v_x, v_y, area);
            if let Some(_m_y) = r.0 {
                found.insert((v_x, v_y), r.1);
            }
        }
    }
    found
}

fn part2(area: &Area) -> Answer {
    let found = solve2(area);
    found.len() as Answer
}

fn draw(area: &Area) -> Answer {
    // Flip y coord inside this to make it the right way
    let found = solve2(area);
    let mut ext = (
        [0.min(area.min_x), -(0.min(area.min_y))],
        [0.max(area.max_x), -(0.max(area.max_y))],
    );
    for p in found.values() {
        let min_x = p.iter().map(|p| p[0]).min().unwrap();
        let max_x = p.iter().map(|p| p[0]).max().unwrap();
        let min_y = p.iter().map(|p| p[1]).min().unwrap();
        let max_y = p.iter().map(|p| p[1]).max().unwrap();
        ext.0[0] = ext.0[0].min(min_x.clamp(-200, 200));
        ext.0[1] = ext.0[1].min(min_y.clamp(-200, 200));
        ext.1[0] = ext.1[0].max(max_x.clamp(-200, 200));
        ext.1[1] = ext.1[1].max(max_y.clamp(-200, 200));
    }
    if ((ext.0[0] - ext.1[0]).abs() + 1) % 2 != 0 {
        ext.1[0] += 1;
    }
    if ((ext.0[1] - ext.1[1]).abs() + 1) % 2 != 0 {
        ext.1[1] += 1;
    }
    let mut gd = aoc::BitmapGridDrawer::new(
        |x| match x {
            'S' | 'T' => [0xff, 0xff, 0xff],
            '*' => [0xff, 0xff, 0xff],
            't' => [0xff, 0xff, 0x66],
            '#' => [0x00, 0x99, 0x00],
            _ => panic!(),
        },
        "ppm/day17",
    );
    gd.set_rect(ext);
    gd.set_bg([0x0f, 0x0f, 0x23]);
    let (min_vx, min_vy, max_vx, max_vy) = get_bounds(area);
    for v_y in min_vy..=max_vy {
        for v_x in min_vx..=max_vx {
            let mut g = HashMap::new();
            g.set_value([0, 0], 'S');
            let (ok, p) = if let Some(p) = found.get(&(v_x, v_y)) {
                (true, p.clone())
            } else {
                (false, shoot(v_x, v_y, area).1)
            };
            for x in area.min_x..=area.max_x {
                for y in area.min_y..=area.max_y {
                    g.set_value([x, -y], if ok { 'T' } else { 't' });
                }
            }
            let mut last_p = [0, 0];
            for pp in p {
                g.line(
                    [pp[0], -pp[1]],
                    [last_p[0], -last_p[1]],
                    if ok { '*' } else { '#' },
                );
                last_p = pp;
            }
            gd.draw(&g);
            gd.save_image();
        }
    }
    found.len() as Answer
}

fn parse(lines: &[String]) -> Area {
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
    } else if part == 2 {
        part2(&parsed)
    } else {
        draw(&parsed)
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
            assert!(found.contains_key(&f), "{:?} not found!", f);
        }
    }
}
