use std::{collections::HashMap, iter::*};

use aoc::{Grid, Point};

type Parsed = Vec<Vec<Point>>;
type Answer = i64;

#[cfg(feature = "vis")]
mod vis {
    use super::*;

    pub struct Drawer {
        drawer: Box<dyn aoc::GridDrawer<HashMap<aoc::Point, char>, char>>,
        grids: Vec<HashMap<Point, char>>,
        paths: Vec<Vec<Point>>,
        fast: bool,
    }

    fn make_col(c: char) -> [u8; 3] {
        if c == '#' {
            [127, 127, 127]
        } else if c == 'o' {
            [255, 255, 0]
        } else if c == '~' {
            [255, 0, 0]
        } else {
            [0, 0, 0]
        }
    }

    impl Drawer {
        pub fn new(name: &str, fast: bool) -> Drawer {
            let mut drawer = aoc::BitmapGridDrawer::new(make_col, name);
            drawer.set_bg([0, 0, 0]);
            Drawer {
                drawer: Box::new(drawer),
                grids: vec![],
                paths: vec![],
                fast,
            }
        }

        pub fn draw(&mut self, grid: &HashMap<Point, char>, path: &[aoc::Point]) {
            self.grids.push(grid.clone());
            self.paths.push(path.to_vec());
        }
    }

    impl Drop for Drawer {
        fn drop(&mut self) {
            let extents = self.grids.iter().map(|g| g.extents()).collect::<Vec<_>>();
            let minx = extents.iter().map(|(minp, _)| minp[0]).min().unwrap();
            let maxx = extents.iter().map(|(_, maxp)| maxp[0]).max().unwrap();
            let miny = extents.iter().map(|(minp, _)| minp[1]).min().unwrap();
            let maxy = extents.iter().map(|(_, maxp)| maxp[1]).max().unwrap();
            let mut last_path = vec![];
            for (grid, path) in zip(&self.grids, &self.paths) {
                let draw = !self.fast || path.len().abs_diff(last_path.len()) > 2;
                last_path = path.clone();
                if draw {
                    let mut g = grid.clone();
                    g.insert([minx, miny], ' ');
                    g.insert([maxx, maxy], ' ');
                    for p in path {
                        g.insert(*p, '~');
                    }
                    self.drawer.draw(&g);
                }
            }
        }
    }
}

fn solve(data: &Parsed, floor: bool) -> Answer {
    let mut grid = HashMap::new();
    for wall in data {
        for p in wall.windows(2) {
            grid.line(p[0], p[1], '#');
        }
    }
    let ([_, _], [_, maxy]) = grid.extents();
    #[cfg(feature = "vis")]
    let mut drawer = vis::Drawer::new(&format!("vis/14/part{}", if floor { 2 } else { 1 }), floor);
    let mut grains = 0;
    let mut s = [500, 0];
    grid.insert(s, '+');
    let mut path = vec![];
    'outer: loop {
        for d in [aoc::SOUTH, aoc::SOUTH_WEST, aoc::SOUTH_EAST] {
            let p = aoc::point_add(s, d);
            let mut c = grid.get_value(p).unwrap_or(' ');
            if floor && p[1] == maxy + 2 {
                c = '#';
            }
            if c != '#' && c != 'o' {
                s = p;
                path.push(s);
                if !floor && s[1] > maxy {
                    break 'outer;
                } else {
                    continue 'outer;
                }
            }
        }
        grid.set_value(s, 'o');
        #[cfg(feature = "vis")]
        drawer.draw(&grid, &path);
        path.pop();
        let p = path.pop().unwrap_or([500, 0]);
        grains += 1;
        if s == [500, 0] {
            break;
        }
        s = p;
    }
    grains
}

fn part1(data: &Parsed) -> Answer {
    solve(data, false)
}

fn part2(data: &Parsed) -> Answer {
    solve(data, true)
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            aoc::split_str(x, "->")
                .iter()
                .map(|x| {
                    let p = aoc::split_ch(x, ',');
                    [p[0].parse().unwrap(), p[1].parse().unwrap()]
                })
                .collect()
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
        vec![
            "498,4 -> 498,6 -> 496,6".into(),
            "503,4 -> 502,4 -> 502,9 -> 494,9".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 24);
    }
}
