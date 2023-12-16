use std::{
    collections::{HashMap, VecDeque},
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

        pub fn draw(&mut self, _grid: &aoc::FxHashMap<aoc::Point, char>) {
            #[cfg(feature = "vis")]
            self.grids.push(_grid.into());
        }
    }

    impl Drop for Drawer {
        fn drop(&mut self) {
            if self.grids.is_empty() {
                return;
            }
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
    let mut todo = VecDeque::from([(start.0, start.1, 0)]);
    let mut energized = aoc::FxHashMap::default();
    energized.insert(start.1, '.');
    let mut seen = aoc::FxHashSet::default();
    let mut last_t = 0;
    while let Some((dir, p, t)) = todo.pop_back() {
        if seen.contains(&(dir, p)) {
            continue;
        }
        seen.insert((dir, p));
        let v = data.get_value(p);
        if v.is_none() {
            continue;
        }
        energized.insert(p, '#');
        if t != last_t {
            drawer.draw(&energized);
            last_t = t;
        }
        match v {
            Some('.') => {
                todo.push_front((dir, aoc::point_add(p, dir), t + 1));
            }
            Some('-') => {
                if dir == aoc::EAST || dir == aoc::WEST {
                    todo.push_front((dir, aoc::point_add(p, dir), t + 1));
                } else {
                    for dir in [aoc::EAST, aoc::WEST] {
                        todo.push_front((dir, aoc::point_add(p, dir), t + 1));
                    }
                }
            }
            Some('|') => {
                if dir == aoc::NORTH || dir == aoc::SOUTH {
                    todo.push_front((dir, aoc::point_add(p, dir), t + 1));
                } else {
                    for dir in [aoc::NORTH, aoc::SOUTH] {
                        todo.push_front((dir, aoc::point_add(p, dir), t + 1));
                    }
                }
            }
            Some('/') => {
                if dir == aoc::EAST || dir == aoc::WEST {
                    let dir = *aoc::DIRECTION_ROTATE_LEFT.get(&dir).unwrap();
                    todo.push_front((dir, aoc::point_add(p, dir), t + 1));
                } else {
                    let dir = *aoc::DIRECTION_ROTATE_RIGHT.get(&dir).unwrap();
                    todo.push_front((dir, aoc::point_add(p, dir), t + 1));
                }
            }
            Some('\\') => {
                if dir == aoc::EAST || dir == aoc::WEST {
                    let dir = *aoc::DIRECTION_ROTATE_RIGHT.get(&dir).unwrap();
                    todo.push_front((dir, aoc::point_add(p, dir), t + 1));
                } else {
                    let dir = *aoc::DIRECTION_ROTATE_LEFT.get(&dir).unwrap();
                    todo.push_front((dir, aoc::point_add(p, dir), t + 1));
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
