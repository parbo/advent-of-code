use std::{
    collections::{HashMap, HashSet},
    iter::*,
};

use aoc::Grid;

type Parsed = Vec<Vec<char>>;

mod vis {
    use super::*;

    pub struct Drawer {
        drawer: Box<dyn aoc::GridDrawer<HashMap<aoc::Point, char>, char>>,
        grids: Vec<HashMap<aoc::Point, char>>,
    }

    fn make_col(c: char) -> [u8; 3] {
        if c == '#' {
            [255, 255, 255]
        } else if c == '.' {
            [255, 0, 0]
        } else {
            [0, 0, 0]
        }
    }

    impl Drawer {
        pub fn new(name: &str) -> Drawer {
            let mut drawer = aoc::BitmapGridDrawer::new(make_col, name);
            drawer.set_bg([0, 0, 0]);
            Drawer {
                drawer: Box::new(drawer),
                grids: vec![],
            }
        }

        pub fn draw(&mut self, grid: &HashMap<aoc::Point, char>) {
            self.grids.push(grid.clone());
        }
    }

    impl Drop for Drawer {
        fn drop(&mut self) {
            let extents = self.grids.iter().map(|g| g.extents()).collect::<Vec<_>>();
            let minx = extents.iter().map(|(minp, _)| minp[0]).min().unwrap();
            let maxx = extents.iter().map(|(_, maxp)| maxp[0]).max().unwrap();
            let miny = extents.iter().map(|(minp, _)| minp[1]).min().unwrap();
            let maxy = extents.iter().map(|(_, maxp)| maxp[1]).max().unwrap();
            for grid in &self.grids {
                let mut g = grid.clone();
                g.insert([minx, miny], ' ');
                g.insert([maxx, maxy], ' ');
                self.drawer.draw(&g);
            }
        }
    }
}

fn solve(data: &Parsed, start: (aoc::Point, aoc::Point), drawer: &mut vis::Drawer) -> i64 {
    let mut todo = vec![start];
    let mut energized = HashMap::from([(start.1, '.')]);
    let mut seen = HashSet::new();
    while let Some((dir, p)) = todo.pop() {
        if seen.contains(&(dir, p)) {
            continue;
        }
        seen.insert((dir, p));
        let v = data.get_value(p);
        if v.is_none() {
            continue;
        }
        energized.insert(p, '#');
        #[cfg(feature = "vis")]
        drawer.draw(&energized);
        match v {
            Some('.') => {
                todo.push((dir, aoc::point_add(p, dir)));
            }
            Some('-') => {
                if dir == aoc::EAST || dir == aoc::WEST {
                    todo.push((dir, aoc::point_add(p, dir)));
                } else {
                    for dir in [aoc::EAST, aoc::WEST] {
                        todo.push((dir, aoc::point_add(p, dir)));
                    }
                }
            }
            Some('|') => {
                if dir == aoc::NORTH || dir == aoc::SOUTH {
                    todo.push((dir, aoc::point_add(p, dir)));
                } else {
                    for dir in [aoc::NORTH, aoc::SOUTH] {
                        todo.push((dir, aoc::point_add(p, dir)));
                    }
                }
            }
            Some('/') => {
                if dir == aoc::EAST || dir == aoc::WEST {
                    let dir = *aoc::DIRECTION_ROTATE_LEFT.get(&dir).unwrap();
                    todo.push((dir, aoc::point_add(p, dir)));
                } else {
                    let dir = *aoc::DIRECTION_ROTATE_RIGHT.get(&dir).unwrap();
                    todo.push((dir, aoc::point_add(p, dir)));
                }
            }
            Some('\\') => {
                if dir == aoc::EAST || dir == aoc::WEST {
                    let dir = *aoc::DIRECTION_ROTATE_RIGHT.get(&dir).unwrap();
                    todo.push((dir, aoc::point_add(p, dir)));
                } else {
                    let dir = *aoc::DIRECTION_ROTATE_LEFT.get(&dir).unwrap();
                    todo.push((dir, aoc::point_add(p, dir)));
                }
            }
            _ => {}
        }
    }
    energized.len() as i64
}

fn part1(data: &Parsed) -> i64 {
    let mut drawer = vis::Drawer::new("vis/16/part1");
    solve(data, (aoc::EAST, [0, 0]), &mut drawer)
}

fn part2(data: &Parsed) -> i64 {
    let mut drawer = vis::Drawer::new("vis/16/part2");
    let ([min_x, min_y], [max_x, max_y]) = data.extents();
    let edges = (min_x..=max_x)
        .map(|x| (aoc::SOUTH, [x, 0]))
        .chain((min_x..=max_x).map(|x| (aoc::NORTH, [x, max_y])))
        .chain((min_y..=max_y).map(|y| (aoc::EAST, [0, y])))
        .chain((min_y..=max_y).map(|y| (aoc::WEST, [max_x, y])));
    edges
        .map(|start| solve(data, start, &mut drawer))
        .max()
        .unwrap()
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            ".|...\\....".into(),
            "|.-.\\.....".into(),
            ".....|-...".into(),
            "........|.".into(),
            "..........".into(),
            ".........\\".into(),
            "..../.\\\\..".into(),
            ".-.-/..|..".into(),
            ".|....-|.\\".into(),
            "..//.|....".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 46);
    }
}
