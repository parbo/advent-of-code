extern crate pancurses;

use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;
use pancurses::*;

#[derive(Debug)]
enum Turn {
    Left,
    Straight,
    Right
}

#[derive(Debug)]
struct Car {
    dir: char,
    pos: (usize, usize),
    next_turn: Turn
}

struct CartGrid {
    cars: Vec<Car>,
    grid: Vec<Vec<char>>,
    window: Window,
    xoffs: i32,
    yoffs: i32
}

impl CartGrid {
    fn new(lines: &[String]) -> CartGrid {
        let mut grid = vec![];
        let mut max_w = 0;
        let mut cars = vec![];
        let mut m = HashMap::new();
        m.insert('<', '-');
        m.insert('>', '-');
        m.insert('v', '|');
        m.insert('^', '|');
        for (row, line) in lines.iter().enumerate() {
            let row_cars : Vec<Car>= line.chars().enumerate().filter(|(_, c)| ['<', '>', 'v', '^'].contains(&c)).map(|(col, c)| Car {dir: c, pos: (col, row), next_turn: Turn::Left }).collect();
            cars.extend(row_cars);
            let v : Vec<char> = line.chars().map(|c| *m.get(&c).unwrap_or(&c)).collect();
            max_w = std::cmp::max(v.len(), max_w);
            grid.push(v);
        }
        for row in &mut grid {
            row.resize(max_w, ' ');
        }
        let window = initscr();
        nl();
        noecho();
        curs_set(0);
        window.keypad(true);
        if has_colors() {
            let mut bg = COLOR_BLACK;
            start_color();
            if use_default_colors() == OK {
                bg = -1;
            }
            init_pair(1, COLOR_WHITE, bg);
            init_pair(2, COLOR_BLACK, COLOR_RED);
        }
        window.scrollok(true);
        window.timeout(20);
        CartGrid { cars: cars, grid: grid, window: window, xoffs: 0, yoffs: 0 }
    }

    fn draw_cars(&self) {
        for car in &self.cars {
            self.window.mvaddch(self.yoffs + car.pos.1 as i32, self.xoffs + car.pos.0 as i32, car.dir);
        }
    }

    fn draw_grid(&self) {
        for y in 0..self.grid.len() {
            let row_len = self.grid[y].len();
            for x in 0..row_len {
                let col = self.grid[y][x];
                let c = match col {
                    '-' => '─',
                    '|' => '│',
                    '+' => '┼',
                    '/' => {
                        if x > 0 && (self.grid[y][x-1] == '-' || self.grid[y][x-1] == '+') {
                            '╯'
                        } else {
                            '╭'
                        }
                    },
                    '\\' => {
                        if x + 1 < row_len && (self.grid[y][x+1] == '-' || self.grid[y][x+1] == '+') {
                            '╰'
                        } else {
                            '╮'
                        }
                    },
                    _ => col
                };
                self.window.mvaddch(self.yoffs + y as i32, self.xoffs + x as i32, c);
            }
        }
    }

    fn draw(&mut self) {
        self.window.clear();
        if has_colors() {
            self.window.attrset(ColorPair(1));
        }
        self.draw_grid();
        if has_colors() {
            self.window.attroff(ColorPair(1));
            self.window.attrset(ColorPair(2));
        }
        self.draw_cars();
        if has_colors() {
            self.window.attroff(ColorPair(2));
        }
        let c = self.window.getch();
        match c {
            Some(Input::KeyLeft) => self.xoffs += 1,
            Some(Input::KeyRight) => self.xoffs -= 1,
            Some(Input::KeyUp) => self.yoffs += 1,
            Some(Input::KeyDown) => self.yoffs -= 1,
            Some(Input::KeyResize) => {
                resize_term(0, 0);
            }
            _ => {}
        }
        self.window.refresh();
    }

    fn tick(&mut self) {
        self.cars.sort_by(|a, b| a.pos.cmp(&b.pos));
        let mut i = 0;
        loop {
            if self.cars.len() == 0 {
                break;
            }
            {
                let c = &mut self.cars[i];
                // move
                match c.dir {
                    '>' => c.pos = (c.pos.0 + 1, c.pos.1),
                    '<' => c.pos = (c.pos.0 - 1, c.pos.1),
                    '^' => c.pos = (c.pos.0, c.pos.1 - 1),
                    'v' => c.pos = (c.pos.0, c.pos.1 + 1),
                    _ => panic!()
                }
                // maybe turn
                if self.grid[c.pos.1][c.pos.0] == '/' {
                    c.dir = match c.dir {
                        '>' => '^',
                        '<' => 'v',
                        '^' => '>',
                        'v' => '<',
                        _ => panic!()
                    }
                } else if self.grid[c.pos.1][c.pos.0] == '\\' {
                    c.dir = match c.dir {
                        '>' => 'v',
                        '<' => '^',
                        '^' => '<',
                        'v' => '>',
                        _ => panic!()
                    }
                } else if self.grid[c.pos.1][c.pos.0] == '+' {
                    c.dir = match c.dir {
                        '>' => match c.next_turn {
                            Turn::Left => '^',
                            Turn::Right => 'v',
                            Turn::Straight => c.dir
                        },
                        '<' => match c.next_turn {
                            Turn::Left => 'v',
                            Turn::Right => '^',
                            Turn::Straight => c.dir
                        },
                        '^' => match c.next_turn {
                            Turn::Left => '<',
                            Turn::Right => '>',
                            Turn::Straight => c.dir
                        },
                        'v' => match c.next_turn {
                            Turn::Left => '>',
                            Turn::Right => '<',
                            Turn::Straight => c.dir
                        },
                        _ => panic!()
                    };
                    match c.next_turn {
                        Turn::Left => c.next_turn = Turn::Straight,
                        Turn::Straight => c.next_turn = Turn::Right,
                        Turn::Right => c.next_turn = Turn::Left
                    }
                }
            }
            match self.is_crash() {
                Some(pos) => {
                    let last = std::cmp::max(pos.0, pos.1);
                    let first = std::cmp::min(pos.0, pos.1);
                    assert!(first == i || last == i);
                    self.cars.remove(last);
                    self.cars.remove(first);
                    // move back the loop pos on crash
                    let mut count = 0;
                    if first <= i {
                        count += 1;
                    }
                    if last <= i {
                        count += 1;
                    }
                    i -= std::cmp::min(count, i);
                }
                _ => {}
            }
            i += 1;
            if i >= self.cars.len() {
                break;
            }
        }
    }

    fn is_crash(&self) -> Option<(usize, usize)> {
        for i in 0..self.cars.len() {
            for j in (i + 1)..self.cars.len() {
                if self.cars[i].pos == self.cars[j].pos {
                    return Some((i, j));
                }
            }
        }
        return None;
    }

    fn cars(&self) -> &[Car] {
        &self.cars
    }
}

fn solve(path: &Path) -> (usize, usize) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut cart_grid = CartGrid::new(&lines);
    loop {
        cart_grid.draw();
        cart_grid.tick();
        if cart_grid.cars().len() <= 1 {
            cart_grid.tick();
            break;
        }
    }
    endwin();
    cart_grid.cars()[0].pos
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{:?}", result);
}
