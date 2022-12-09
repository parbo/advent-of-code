use std::collections::HashSet;
use std::iter::*;

#[cfg(feature = "vis")]
mod vis {
    use std::collections::HashMap;

    use super::*;
    use aoc::Grid;

    pub struct Drawer {
        drawer: Box<dyn aoc::GridDrawer<HashMap<aoc::Point, char>, char>>,
        ropes: Vec<Vec<aoc::Point>>,
    }

    fn make_col(c: char) -> [u8; 3] {
        if c == 't' {
            [127, 127, 127]
        } else if c != ' ' {
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
                ropes: vec![],
            }
        }

        pub fn draw(&mut self, rope: &[aoc::Point]) {
            self.ropes.push(rope.to_vec());
        }
    }

    impl Drop for Drawer {
        fn drop(&mut self) {
            let grids = self
                .ropes
                .iter()
                .map(|r| make_grid(r.as_slice()))
                .collect::<Vec<_>>();
            let extents = grids.iter().map(|g| g.extents()).collect::<Vec<_>>();
            let minx = extents.iter().map(|(minp, _)| minp[0]).min().unwrap();
            let maxx = extents.iter().map(|(_, maxp)| maxp[0]).max().unwrap();
            let miny = extents.iter().map(|(minp, _)| minp[1]).min().unwrap();
            let maxy = extents.iter().map(|(_, maxp)| maxp[1]).max().unwrap();
            let mut tails: HashMap<aoc::Point, char> = HashMap::new();
            for (i, grid) in grids.iter().enumerate() {
                let mut g = tails.clone();
                g.insert([minx, miny], ' ');
                g.insert([maxx, maxy], ' ');
                for (k, v) in grid {
                    g.insert(*k, *v);
                }
                tails.insert(self.ropes[i][9], 't');
                self.drawer.draw(&g);
            }
        }
    }

    fn make_grid(rope: &[aoc::Point]) -> HashMap<aoc::Point, char> {
        let mut grid: HashMap<aoc::Point, char> = HashMap::new();
        for (i, c) in ['9', '8', '7', '6', '5', '4', '3', '2', '1', 'H']
            .into_iter()
            .enumerate()
        {
            grid.insert(rope[i], c);
        }
        grid
    }
}

type ParsedItem = (String, i64);
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn follow(h: aoc::Point, mut t: aoc::Point) -> aoc::Point {
    if (h[0] - t[0]).abs() > 1 || (h[1] - t[1]).abs() > 1 {
        let dir = aoc::point_sub(h, t);
        t = aoc::point_add(t, [dir[0].signum(), dir[1].signum()]);
    }
    t
}

fn part1(data: &Parsed) -> Answer {
    let mut h = [0, 0];
    let mut t = [0, 0];
    let mut visited: HashSet<aoc::Point> = HashSet::new();
    visited.insert(t);
    for (mv, d) in data {
        let dir = aoc::DIRECTION_MAP.get(mv.as_str()).unwrap();
        for _ in 0..*d {
            h = aoc::point_add(h, *dir);
            t = follow(h, t);
            visited.insert(t);
        }
    }
    visited.len() as i64
}

fn part2(data: &Parsed) -> Answer {
    let mut rope = vec![[0, 0]; 10];
    let mut visited: HashSet<aoc::Point> = HashSet::new();
    #[cfg(feature = "vis")]
    let mut drawer = vis::Drawer::new("vis/9/part2");
    visited.insert(rope[9]);
    for (mv, d) in data {
        let dir = aoc::DIRECTION_MAP.get(mv.as_str()).unwrap();
        for _ in 0..*d {
            rope[0] = aoc::point_add(rope[0], *dir);
            for i in 1..10 {
                rope[i] = follow(rope[i - 1], rope[i]);
            }
            #[cfg(feature = "vis")]
            drawer.draw(&rope);
            visited.insert(rope[9]);
        }
    }
    visited.len() as i64
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let p = aoc::split_w(x);
            (p[0].to_string(), p[1].parse::<i64>().unwrap())
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
            "R 4".into(),
            "U 4".into(),
            "L 3".into(),
            "D 1".into(),
            "R 4".into(),
            "D 1".into(),
            "L 5".into(),
            "R 2".into(),
        ]
    }

    fn example2() -> Vec<String> {
        vec![
            "R 5".into(),
            "U 8".into(),
            "L 8".into(),
            "D 3".into(),
            "R 17".into(),
            "D 10".into(),
            "L 25".into(),
            "U 20".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 13);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(&parse(&example())), 1);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&parse(&example2())), 36);
    }

    #[test]
    fn test_follow() {
        assert_eq!(follow([0, 0], [0, 0]), [0, 0]);
        assert_eq!(follow([0, 1], [0, 0]), [0, 0]);
        assert_eq!(follow([1, 0], [0, 0]), [0, 0]);
        assert_eq!(follow([0, -1], [0, 0]), [0, 0]);
        assert_eq!(follow([-1, 0], [0, 0]), [0, 0]);
        assert_eq!(follow([0, 2], [0, 0]), [0, 1]);
        assert_eq!(follow([2, 0], [0, 0]), [1, 0]);
        assert_eq!(follow([0, -2], [0, 0]), [0, -1]);
        assert_eq!(follow([-2, 0], [0, 0]), [-1, 0]);
        assert_eq!(follow([1, -1], [0, 0]), [0, 0]);
    }
}
