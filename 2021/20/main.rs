use aoc::{BitmapSpriteGridDrawer, Grid, GridDrawer, Point};
use std::collections::HashMap;
use std::iter::*;

type Parsed = (Vec<char>, HashMap<Point, char>);
type Answer = i64;

pub fn kernel(p: Point) -> impl Iterator<Item = Point> {
    let mut diter = [
        aoc::NORTH_WEST,
        aoc::NORTH,
        aoc::NORTH_EAST,
        aoc::WEST,
        [0, 0],
        aoc::EAST,
        aoc::SOUTH_WEST,
        aoc::SOUTH,
        aoc::SOUTH_EAST,
    ]
    .iter();
    from_fn(move || diter.next().map(|d| aoc::point_add(p, *d)))
}

fn solve(map: &Parsed, iterations: usize, draw: bool) -> Answer {
    let mut inf_c = '.';
    let mut g = map.1.clone();
    let mut all_g = vec![(g.clone(), inf_c)];
    let mut max_ext = g.extents();
    for _i in 0..iterations {
        let mut new_g = HashMap::new();
        let ext = g.extents();
        let ([min_x, min_y], [max_x, max_y]) = ext;
        max_ext.0[0] = min_x.min(max_ext.0[0]);
        max_ext.0[1] = min_y.min(max_ext.0[1]);
        max_ext.1[0] = max_x.max(max_ext.1[0]);
        max_ext.1[1] = max_y.max(max_ext.1[1]);
        // First do the points inside the extent
        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                let p = [x, y];
                let mut ix = 0;
                for pp in kernel(p) {
                    let c = if aoc::inside_extent(pp, ext) {
                        if let Some(x) = g.get_value(pp) {
                            x
                        } else {
                            '.'
                        }
                    } else {
                        inf_c
                    };
                    let v = (c == '#') as usize;
                    ix <<= 1;
                    ix += v;
                }
                if map.0[ix] == '#' {
                    new_g.set_value(p, '#');
                }
            }
        }
        // Update inf_c
        if inf_c == '#' {
            inf_c = map.0[511];
        } else {
            inf_c = map.0[0];
        }
        g = new_g;
        if draw {
            all_g.push((g.clone(), inf_c));
        }
    }
    if draw {
        let mut gd = BitmapSpriteGridDrawer::new(
            (1, 1),
            |c| {
                if c == '#' {
                    vec![[0xff, 0xff, 0xff]; 1]
                } else {
                    vec![[0, 0, 0]; 1]
                }
            },
            "ppm/day20",
        );
        gd.set_rect(max_ext);
        gd.set_unset('.');
        for (gg, ic) in all_g {
            let mut ggg = HashMap::new();
            let ext = gg.extents();
            for x in max_ext.0[0]..=max_ext.1[0] {
                for y in max_ext.0[1]..=max_ext.1[1] {
                    let p = [x, y];
                    if aoc::inside_extent(p, ext) {
                        if let Some(v) = gg.get_value(p) {
                            ggg.set_value(p, v)
                        }
                    } else {
                        ggg.set_value(p, ic)
                    }
                }
            }
            gd.draw(&ggg);
            gd.save_image();
        }
    }
    g.iter().filter(|(_p, x)| **x == '#').count() as Answer
}

fn part1(map: &Parsed) -> Answer {
    let draw = cfg!(feature = "vis");
    solve(map, 2, draw)
}

fn part2(map: &Parsed) -> Answer {
    let draw = cfg!(feature = "vis");
    solve(map, 50, draw)
}

fn parse(lines: &[String]) -> Parsed {
    let sections = aoc::split_by_empty_line(lines);
    let enhance = sections[0][0].chars().collect();
    let grid = aoc::parse_grid_to_sparse(&sections[1], |x| if x == '#' { Some(x) } else { None });
    (enhance, grid)
}

fn main() {
    aoc::run_main(parse, part1, part2);
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
