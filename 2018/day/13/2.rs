#[macro_use]
extern crate lazy_static;
extern crate pancurses;

use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;
use pancurses::*;

lazy_static! {
    static ref SPRITES: HashMap<char, char> = {
        let mut map = HashMap::new();
        map.insert('-', '═');
	map.insert('|', '║');
	map.insert('+', '╬');
        map
    };
    static ref COLORS: HashMap<char, u8> = {
        let mut map = HashMap::new();
        map.insert('<', 2);
        map.insert('>', 2);
        map.insert('^', 2);
        map.insert('v', 2);
        map
    };
}

fn sprite(s: char) -> char {
    *SPRITES.get(&s).unwrap_or(&s)
}

fn draw(window: &Window, x: usize, y: usize, c: char) {
    if has_colors() {
        let color = COLORS.get(&c).unwrap_or(&1);
        window.attrset(ColorPair(*color));
    }
    window.mvaddch(y as i32, x as i32, sprite(c));
    if has_colors() {
        let color = COLORS.get(&c).unwrap_or(&1);
        window.attroff(ColorPair(*color));
    }
}

fn draw_grid(window: &Window, cars: &Vec<Car>, grid: &Vec<Vec<char>>) {
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let mut car = None;
            for c in cars {
                if (x, y) == c.pos {
                    car = Some(c.dir);
                    break;
                }
            }
            if !car.is_none() {
                let c = car.unwrap();
                draw(window, x, y, c);
            } else {
                draw(window, x, y, *col);
            }
        }
    }
}

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

fn tick(cars: &mut Vec<Car>, grid: &Vec<Vec<char>>) {
    cars.sort_by(|a, b| a.pos.cmp(&b.pos));
    let mut i = 0;
    loop {
        if cars.len() == 0 {
            break;
        }
        {
            let c = &mut cars[i];
            // move
            match c.dir {
                '>' => c.pos = (c.pos.0 + 1, c.pos.1),
                '<' => c.pos = (c.pos.0 - 1, c.pos.1),
                '^' => c.pos = (c.pos.0, c.pos.1 - 1),
                'v' => c.pos = (c.pos.0, c.pos.1 + 1),
                _ => panic!()
            }
            // maybe turn
            if grid[c.pos.1][c.pos.0] == '/' {
                c.dir = match c.dir {
                    '>' => '^',
                    '<' => 'v',
                    '^' => '>',
                    'v' => '<',
                    _ => panic!()
                }
            } else if grid[c.pos.1][c.pos.0] == '\\' {
                c.dir = match c.dir {
                    '>' => 'v',
                    '<' => '^',
                    '^' => '<',
                    'v' => '>',
                    _ => panic!()
                }
            } else if grid[c.pos.1][c.pos.0] == '+' {
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
        match is_crash(&cars) {
            Some(pos) => {
                let last = std::cmp::max(pos.0, pos.1);
                let first = std::cmp::min(pos.0, pos.1);
                assert!(first == i || last == i);
                cars.remove(last);
                cars.remove(first);
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
        if i >= cars.len() {
            break;
        }
    }
}

fn is_crash(cars: &Vec<Car>) -> Option<(usize, usize)> {
    for i in 0..cars.len() {
        for j in (i + 1)..cars.len() {
            if cars[i].pos == cars[j].pos {
                return Some((i, j));
            }
        }
    }
    return None;
}

fn parse(path: &Path) -> (Vec<Car>, Vec<Vec<char>>) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
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
    (cars, grid)
}

fn solve(path: &Path, sleep: u64) -> (usize, usize) {
    let (mut cars, grid) = parse(path);
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
    window.timeout(sleep as i32);
    loop {
        window.clear();
        draw_grid(&window, &cars, &grid);
        tick(&mut cars, &grid);
        if cars.len() <= 1 {
            tick(&mut cars, &grid);
            break;
        }
        let _ = window.getch();
        window.refresh();
    }
    endwin();
    cars[0].pos
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let sleep_str = &args[2];
    let sleep = sleep_str.parse::<u64>().unwrap();

    let result = solve(Path::new(&filename), sleep);
    println!("{:?}", result);
}
