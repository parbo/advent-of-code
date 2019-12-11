use aoc;
use intcode;
use std::iter::*;
use std::collections::HashMap;
use pancurses::*;

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left
}

fn draw(window: &Window, hull: &HashMap<(i128, i128), i128>, robot: ((i128, i128), Dir)) {
    window.clear();
    for ((x, y), col) in hull {
	let ch = match col {
            1 => '█',
	    _ => ' ',
	};
        window.mvaddch(*y as i32, *x as i32, ch);
    }
    let rch = match robot.1 {
        Dir::Up => '^',
        Dir::Right => '>',
        Dir::Down => 'v',
        Dir::Left => '<',
    };
    window.mvaddch((robot.0).1 as i32, (robot.0).0 as i32, rch);
    let _ = window.getch();
    window.refresh();
}

fn paint(numbers: &Vec<i128>, color: i128, window: Option<&Window>) -> HashMap<(i128, i128), i128> {
    let mut m = intcode::Machine::new(&numbers);
    let mut current_color;
    let mut current_dir = Dir::Up;
    let mut x = 0;
    let mut y = 0;
    let mut hull = HashMap::new();
    hull.insert((x, y), color);
    loop {
        current_color = *hull.get(&(x, y)).unwrap_or(&0);
        m.add_input(current_color);
        let color = match m.run_to_next_output() {
            Some(c) => c,
            None => break hull,
        };
        let turn = match m.run_to_next_output() {
            Some(t) => t,
            None => break hull,
        };
        hull.insert((x, y), color);
        current_dir = match current_dir {
            Dir::Up => if turn == 0 { Dir::Left } else { Dir::Right },
            Dir::Right => if turn == 0 { Dir::Up } else { Dir::Down },
            Dir::Down => if turn == 0 { Dir::Right } else { Dir::Left },
            Dir::Left => if turn == 0 { Dir::Down } else { Dir::Up },
        };
        match current_dir {
            Dir::Up => y -= 1,
            Dir::Right => x += 1,
            Dir::Down => y += 1,
            Dir::Left => x -= 1,
        }
	if let Some(w) = window {
	    draw(w, &hull, ((x, y), current_dir));
	}
    }
}

fn part1(numbers: &Vec<i128>) -> i128 {
    let hull = paint(numbers, 0, None);
    hull.iter().count() as i128
}

fn part2(numbers: &Vec<i128>) -> i128 {
    let window = initscr();
    nl();
    noecho();
    curs_set(0);
    window.keypad(true);
    window.scrollok(true);
    window.timeout(20);
    let hull = paint(numbers, 1, Some(&window));
    let min_x = hull.iter().map(|p| (p.0).0).min().unwrap();
    let min_y = hull.iter().map(|p| (p.0).1).min().unwrap();
    let max_x = hull.iter().map(|p| (p.0).0).max().unwrap();
    let max_y = hull.iter().map(|p| (p.0).1).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = match hull.get(&(x, y)) {
                Some(c) => *c,
                None => 0,
            };
            if c == 0 {
                print!(" ");
            } else {
                print!("█");
            }
        }
        println!();
    }
    0
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = aoc::parse_intcode(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

