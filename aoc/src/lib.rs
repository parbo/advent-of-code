use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::*;
use std::path::Path;
use pancurses;

pub use num::integer::*;
pub use serde_scan::scan;
pub use serde_scan::from_str;

pub trait GridDrawer {
    fn draw(&mut self, area: &HashMap<(i128, i128), i128>);
}

pub struct NopGridDrawer {}

impl GridDrawer for NopGridDrawer {
    fn draw(&mut self, _: &HashMap<(i128, i128), i128>) {}
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

impl<F> GridDrawer for PrintGridDrawer<F>
where
    F: Fn(i128) -> char,
{
    fn draw(&mut self, area: &HashMap<(i128, i128), i128>) {
        let min_x = area.iter().map(|p| (p.0).0).min().unwrap();
        let min_y = area.iter().map(|p| (p.0).1).min().unwrap();
        let max_x = area.iter().map(|p| (p.0).0).max().unwrap();
        let max_y = area.iter().map(|p| (p.0).1).max().unwrap();
	for y in min_y..=max_y {
	    for x in min_x..=max_x {
		let ch = if let Some(x) = area.get(&(x, y)) {
		    self.to_char(*x)
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

impl<F> GridDrawer for CursesGridDrawer<F>
where
    F: Fn(i128) -> char,
{
    fn draw(&mut self, area: &HashMap<(i128, i128), i128>) {
        self.window.clear();
        let min_x = area.iter().map(|p| (p.0).0).min().unwrap();
        let min_y = area.iter().map(|p| (p.0).1).min().unwrap();
        for ((x, y), col) in area {
            let ch = self.to_char(*col);
            self.window
                .mvaddch((*y - min_y) as i32, (*x - min_x) as i32, ch);
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
