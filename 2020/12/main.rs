use aoc::*;
use std::collections::HashMap;
use std::iter::*;

type Parsed = Vec<(char, i64)>;

fn draw(orig_path: &[(Point, Point)], s: &str, scale: i64) {
    let mut minx = 0;
    let mut miny = 0;
    let mut maxx = 0;
    let mut maxy = 0;
    // scale it
    let mut scaled_path = vec![];
    for (ship, wp) in orig_path {
        scaled_path.push((
            [ship[0] / scale, ship[1] / scale],
            [wp[0] / scale, wp[1] / scale],
        ));
    }
    // find min/max
    for (ship, wp) in &scaled_path {
        minx = ship[0].min(minx);
        miny = ship[1].min(miny);
        maxx = ship[0].max(maxx);
        maxy = ship[1].max(maxy);
        let p = point_add(*ship, *wp);
        minx = p[0].min(minx);
        miny = p[1].min(miny);
        maxx = p[0].max(maxx);
        maxy = p[1].max(maxy);
    }
    // extend paths by connecting with lines
    let mut path = vec![];
    for i in 0..(scaled_path.len() - 1) {
        let (s0, w0) = scaled_path[i];
        let (s1, w1) = scaled_path[i + 1];
        let s_line = plot_line(s0, s1);
        let w_line = plot_line(w0, w1);
        let ms = s_line.len().max(w_line.len());
        for j in 0..ms {
            let s = if j < s_line.len() {
                s_line[j]
            } else {
                s_line[s_line.len() - 1]
            };
            let w = if j < w_line.len() {
                w_line[j]
            } else {
                w_line[w_line.len() - 1]
            };
            path.push((s, w));
        }
    }
    let mut g = HashMap::new();
    g.insert([minx, miny], '.');
    g.insert([maxx, maxy], '.');
    let mut gd = aoc::BitmapGridDrawer::new(
        |x| match x {
            '#' => [100, 0, 0],
            _ => [255, 255, 255],
        },
        s,
    );
    let f = path.len();
    println!("{} frames", path.len());
    for (i, (ship, wp)) in path.iter().enumerate() {
        *g.entry(*ship).or_insert('#') = '#';
        // Draw max 1000 frames
        if (f > 1000 && i % (f / 1000) == 0) || i + 1 == f {
            // Center rect around ship
            let r = (
                [ship[0] - 500, ship[1] - 500],
                [ship[0] + 500, ship[1] + 500],
            );
            gd.set_rect(r);
            gd.draw_grid(&g);
            // Fatten up the ship
            for y in -4..=4 {
                for x in -4..=4 {
                    gd.put_pixel(point_add([x, y], [500, 500]), [255, 0, 0]);
                }
            }
            // Draw the wp as a plus and also offset it a bit
            let pp = point_mul(*wp, 16);
            for y in -2..=2 {
                gd.put_pixel(point_add(point_add([0, y], pp), [500, 500]), [0, 0, 255]);
            }
            for x in -2..=2 {
                gd.put_pixel(point_add(point_add([x, 0], pp), [500, 500]), [0, 0, 255]);
            }
            gd.save_image();
        }
    }
}

fn part1(moves: &Parsed) -> i64 {
    let mut curr = [0, 0];
    let mut facing = EAST;
    let mut path = vec![(curr, facing)];
    for (d, steps) in moves {
        match d {
            'E' => curr = point_add(curr, point_mul(EAST, *steps)),
            'W' => curr = point_add(curr, point_mul(WEST, *steps)),
            'N' => curr = point_add(curr, point_mul(NORTH, *steps)),
            'S' => curr = point_add(curr, point_mul(SOUTH, *steps)),
            'F' => curr = point_add(curr, point_mul(facing, *steps)),
            'L' => {
                for _ in (0..*steps).step_by(90) {
                    facing = [facing[1], -facing[0]]
                }
            }
            'R' => {
                for _ in (0..*steps).step_by(90) {
                    facing = [-facing[1], facing[0]]
                }
            }
            _ => panic!(),
        }
        path.push((curr, facing));
    }
    if cfg!(feature = "vis") {
        draw(&path, "ppm/day12/part1", 1);
    }
    curr[0].abs() + curr[1].abs()
}

fn part2(moves: &Parsed) -> i64 {
    let mut waypoint = [10, -1];
    let mut ship = [0, 0];
    let mut path = vec![(ship, waypoint)];
    for (d, steps) in moves {
        match d {
            'E' => waypoint = point_add(waypoint, point_mul(EAST, *steps)),
            'W' => waypoint = point_add(waypoint, point_mul(WEST, *steps)),
            'N' => waypoint = point_add(waypoint, point_mul(NORTH, *steps)),
            'S' => waypoint = point_add(waypoint, point_mul(SOUTH, *steps)),
            'F' => ship = point_add(ship, point_mul(waypoint, *steps)),
            'L' => {
                for _ in (0..*steps).step_by(90) {
                    waypoint = [waypoint[1], -waypoint[0]]
                }
            }
            'R' => {
                for _ in (0..*steps).step_by(90) {
                    waypoint = [-waypoint[1], waypoint[0]]
                }
            }
            _ => panic!(),
        }
        path.push((ship, waypoint));
    }
    if cfg!(feature = "vis") {
        draw(&path, "ppm/day12/part2", 16);
    }
    ship[0].abs() + ship[1].abs()
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| (x.chars().next().unwrap(), x[1..].parse().unwrap()))
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&vec![('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)]),
            286
        );
    }
}
