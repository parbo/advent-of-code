use pancurses;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::*;
use std::path::Path;

pub use num::integer::*;
pub use serde_scan::from_str;
pub use serde_scan::scan;

pub trait Grid {
    fn get_value(&self, pos: (i128, i128)) -> Option<i128>;
    fn extents(&self) -> ((i128, i128), (i128, i128));
}

impl Grid for HashMap<(i128, i128), i128> {
    fn get_value(&self, pos: (i128, i128)) -> Option<i128> {
        if let Some(x) = self.get(&pos) {
            Some(*x)
        } else {
            None
        }
    }
    fn extents(&self) -> ((i128, i128), (i128, i128)) {
        let min_x = self.iter().map(|p| (p.0).0).min().unwrap();
        let min_y = self.iter().map(|p| (p.0).1).min().unwrap();
        let max_x = self.iter().map(|p| (p.0).0).max().unwrap();
        let max_y = self.iter().map(|p| (p.0).1).max().unwrap();
        ((min_x, max_x), (min_y, max_y))
    }
}

impl Grid for Vec<Vec<i128>> {
    fn get_value(&self, pos: (i128, i128)) -> Option<i128> {
        let (x, y) = pos;
        if let Some(line) = self.get(y as usize) {
            if let Some(c) = line.get(x as usize) {
                return Some(*c);
            }
        }
        None
    }
    fn extents(&self) -> ((i128, i128), (i128, i128)) {
        if self.len() > 0 {
            if self[0].len() > 0 {
                return (
                    (0, (self[0].len() - 1) as i128),
                    (0, (self.len() - 1) as i128),
                );
            }
        }
        ((0, 0), (0, 0))
    }
}

pub trait GridDrawer<G>
where
    G: Grid,
{
    fn draw(&mut self, area: &G);
}

pub struct NopGridDrawer {}

impl<G> GridDrawer<G> for NopGridDrawer
where
    G: Grid,
{
    fn draw(&mut self, _: &G) {}
}

pub struct PrintGridDrawer<F>
where
    F: Fn(i128) -> char,
{
    to_ch: F,
}

impl<F> PrintGridDrawer<F>
where
    F: Fn(i128) -> char,
{
    pub fn new(to_ch: F) -> PrintGridDrawer<F> {
        PrintGridDrawer { to_ch }
    }

    fn to_char(&self, col: i128) -> char {
        (self.to_ch)(col)
    }
}

impl<F, G> GridDrawer<G> for PrintGridDrawer<F>
where
    F: Fn(i128) -> char,
    G: Grid,
{
    fn draw(&mut self, area: &G) {
        let ((min_x, max_x), (min_y, max_y)) = area.extents();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let ch = if let Some(x) = area.get_value((x, y)) {
                    self.to_char(x)
                } else {
                    ' '
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

pub struct CursesGridDrawer<F>
where
    F: Fn(i128) -> char,
{
    window: pancurses::Window,
    to_ch: F,
}

impl<F> CursesGridDrawer<F>
where
    F: Fn(i128) -> char,
{
    pub fn new(to_ch: F) -> CursesGridDrawer<F> {
        let window = pancurses::initscr();
        pancurses::nl();
        pancurses::noecho();
        pancurses::curs_set(0);
        window.keypad(true);
        window.scrollok(true);
        window.timeout(16);
        CursesGridDrawer { window, to_ch }
    }

    fn to_char(&self, col: i128) -> char {
        (self.to_ch)(col)
    }
}

impl<F> Drop for CursesGridDrawer<F>
where
    F: Fn(i128) -> char,
{
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

impl<F, G> GridDrawer<G> for CursesGridDrawer<F>
where
    F: Fn(i128) -> char,
    G: Grid,
{
    fn draw(&mut self, area: &G) {
        self.window.clear();
        let ((min_x, max_x), (min_y, max_y)) = area.extents();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let ch = if let Some(x) = area.get_value((x, y)) {
                    self.to_char(x)
                } else {
                    ' '
                };
                self.window
                    .mvaddch((y - min_y) as i32, (x - min_x) as i32, ch);
            }
        }
        if let Some(pancurses::Input::Character(c)) = self.window.getch() {
            if c == 'q' {
                pancurses::endwin();
                std::process::exit(0);
            }
        }
        self.window.refresh();
    }
}

// TODO: improve and generalize
pub struct Tree {
    things: HashSet<String>,
    children: HashMap<String, Vec<String>>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            things: HashSet::new(),
            children: HashMap::new(),
        }
    }

    pub fn things(&self) -> impl Iterator<Item = &String> {
        self.things.iter()
    }

    pub fn insert(&mut self, parent: &str, child: &str) {
        self.things.insert(parent.to_string());
        self.things.insert(child.to_string());
        self.children
            .entry(parent.to_string())
            .or_insert(Vec::new())
            .push(child.to_string());
    }

    pub fn depth_from_to(&self, from: &str, to: &str) -> Option<i64> {
        self.depth_from_to_recursive(from, to, 0)
    }

    fn depth_from_to_recursive(&self, from: &str, to: &str, depth: i64) -> Option<i64> {
        if from == to {
            return Some(depth);
        }
        if let Some(v) = self.children.get(from) {
            for t in v {
                if let Some(x) = self.depth_from_to_recursive(t, to, depth + 1) {
                    return Some(x);
                }
            }
        }
        return None;
    }
}

pub fn read_lines() -> (i32, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();
    let filename = &args[2];

    let input = File::open(Path::new(filename)).unwrap();
    let buffered = BufReader::new(input);
    (
        part,
        buffered
            .lines()
            .filter_map(Result::ok)
            .map(|x| x.trim().to_string())
            .collect(),
    )
}

pub fn parse_intcode(lines: &Vec<String>) -> Vec<i128> {
    let result: Vec<i128> = lines[0]
        .split(|c| c == ',')
        .map(|s| s.trim())
        .map(|v| v.parse::<i128>().unwrap())
        .collect();
    result
}
