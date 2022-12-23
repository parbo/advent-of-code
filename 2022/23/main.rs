use aoc::{
    neighbors_incl_diagonals, point_add, Grid, Point, EAST, NORTH, NORTH_EAST, NORTH_WEST, SOUTH,
    SOUTH_EAST, SOUTH_WEST, WEST,
};
use std::{collections::HashMap, iter::*};

type Parsed = HashMap<Point, char>;

#[cfg(feature = "vis")]
mod vis {
    use super::*;

    pub struct Drawer {
        drawer: Box<dyn aoc::GridDrawer<HashMap<aoc::Point, char>, char>>,
        grids: Vec<HashMap<Point, char>>,
    }

    fn make_col(c: char) -> [u8; 3] {
        if c == '#' {
            [255, 255, 255]
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

        pub fn draw(&mut self, grid: &HashMap<Point, char>) {
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

fn solve(data: &Parsed, max: Option<usize>) -> (usize, usize) {
    let mut rules = vec![
        ([NORTH, NORTH_EAST, NORTH_WEST], NORTH),
        ([SOUTH, SOUTH_EAST, SOUTH_WEST], SOUTH),
        ([WEST, NORTH_WEST, SOUTH_WEST], WEST),
        ([EAST, NORTH_EAST, SOUTH_EAST], EAST),
    ];
    let mut g = data.clone();
    #[cfg(feature = "vis")]
    let mut drawer = vis::Drawer::new(&format!("vis/23/part{}", if max.is_none() { 2 } else { 1 }));
    let mut rounds = 0;
    loop {
        let mut proposed: HashMap<Point, Vec<Point>> = HashMap::new();
        for p in g.keys() {
            if neighbors_incl_diagonals(*p).any(|n| g.get_value(n).is_some()) {
                for (nb, d) in &rules {
                    if nb.iter().all(|n| g.get_value(point_add(*p, *n)).is_none()) {
                        proposed.entry(point_add(*p, *d)).or_default().push(*p);
                        break;
                    }
                }
            }
        }
        rounds += 1;
        if proposed.is_empty() {
            break;
        }
        for (to, from) in &proposed {
            if from.len() == 1 {
                let r = g.remove(&from[0]);
                assert!(r.is_some());
                g.insert(*to, '#');
            }
        }
        rules.rotate_left(1);
        #[cfg(feature = "vis")]
        drawer.draw(&g);
        if Some(rounds) == max {
            break;
        }
    }
    let empty = g
        .points()
        .filter_map(|p| {
            if g.get_value(p).is_none() {
                Some(true)
            } else {
                None
            }
        })
        .count();
    (empty, rounds)
}

fn part1(data: &Parsed) -> usize {
    solve(data, Some(10)).0
}
fn part2(data: &Parsed) -> usize {
    solve(data, None).1
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid_to_sparse(lines, |c| if c == '#' { Some(c) } else { None })
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 110);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 20);
    }
}
