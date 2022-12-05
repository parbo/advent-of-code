use std::{collections::VecDeque, iter::*};

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("move {num} from {from} to {to}")]
struct Move {
    num: usize,
    from: usize,
    to: usize,
}

type Parsed = (Vec<VecDeque<char>>, Vec<Move>);
type Answer = String;

#[cfg(feature = "vis")]
mod vis {
    use super::*;
    use aoc::Grid;
    use aoc::GridDrawer;

    pub struct Drawer {
        drawer: Box<aoc::BitmapGridDrawer<fn([u8; 3]) -> [u8; 3], Vec<Vec<[u8; 3]>>, [u8; 3]>>,
        stacks: Vec<Vec<VecDeque<char>>>,
    }

    impl Drawer {
        pub fn new(name: &str) -> Drawer {
            Drawer {
                drawer: Box::new(aoc::BitmapGridDrawer::new(|x| x, name)),
                stacks: vec![],
            }
        }

        pub fn draw(&mut self, stacks: &Vec<VecDeque<char>>) {
            self.stacks.push(stacks.clone());
        }
    }

    impl Drop for Drawer {
        fn drop(&mut self) {
            let w = self.stacks[0].len();
            let h = self
                .stacks
                .iter()
                .map(|x| x.iter().map(|x| x.len()).max().unwrap())
                .max()
                .unwrap();
            let xs = aoc::SMALLFONT.glyph_size().0 + 1;
            let ys = aoc::SMALLFONT.glyph_size().1 + 1;
            for stacks in &self.stacks {
                let mut grid = vec![vec![[0, 0, 0]; w * xs as usize]; h * ys as usize];
                let mut x = 0;
                for s in stacks {
                    let mut y = (h - 1) as i64 * ys as i64;
                    for c in s {
                        grid.text_ch(*c, [x, y], [255, 255, 255]);
                        y -= ys as i64;
                    }
                    x += xs as i64;
                }
                self.drawer.draw(&grid);
            }
        }
    }
}

fn part1(data: &Parsed) -> Answer {
    let mut stacks = data.0.clone();
    #[cfg(feature = "vis")]
    let mut drawer = vis::Drawer::new("vis/5/part1");
    for mv in &data.1 {
        for _i in 0..mv.num {
            let c = stacks[mv.from - 1].pop_front().unwrap();
            stacks[mv.to - 1].push_front(c);
            #[cfg(feature = "vis")]
            drawer.draw(&stacks);
        }
    }
    stacks.iter().map(|x| x.front().unwrap()).copied().collect()
}

fn part2(data: &Parsed) -> Answer {
    let mut stacks = data.0.clone();
    #[cfg(feature = "vis")]
    let mut drawer = vis::Drawer::new("vis/5/part2");
    for mv in &data.1 {
        let mut tmp = VecDeque::new();
        for _i in 0..mv.num {
            let c = stacks[mv.from - 1].pop_front().unwrap();
            tmp.push_front(c);
        }
        for x in tmp {
            stacks[mv.to - 1].push_front(x);
        }
        #[cfg(feature = "vis")]
        drawer.draw(&stacks);
    }
    stacks.iter().map(|x| x.front().unwrap()).copied().collect()
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    let cols = (parts[0].iter().map(|x| x.len()).max().unwrap() + 1) / 4;
    let mut stacks = vec![VecDeque::new(); cols];
    for line in &parts[0] {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let c = line.chars().nth(1 + 4 * i).unwrap();
            if c != ' ' && !c.is_ascii_digit() {
                stack.push_back(c);
            }
        }
    }
    let moves = parts[1].iter().map(|x| x.parse().unwrap()).collect();
    (stacks, moves)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
