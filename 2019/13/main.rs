use pancurses::*;
use std::collections::HashMap;
use std::iter::*;

type Parsed = Vec<i128>;

// 0 is an empty tile. No game object appears in this tile.
// 1 is a wall tile. Walls are indestructible barriers.
// 2 is a block tile. Blocks can be broken by the ball.
// 3 is a horizontal paddle tile. The paddle is indestructible.
// 4 is a ball tile. The ball moves diagonally and bounces off objects.

fn part1(program: &Parsed) -> i128 {
    let mut m = intcode::Machine::new(program);
    let mut blocks = HashMap::new();
    loop {
        let x = m.run_to_next_output();
        let y = m.run_to_next_output();
        let tile = m.run_to_next_output();
        if let (Some(x), Some(y), Some(tile)) = (x, y, tile) {
            blocks.insert((x, y), tile);
        } else {
            break;
        }
    }
    blocks.values().filter(|v| **v == 2).count() as i128
}

fn draw(window: &Window, x: i128, y: i128, v: i128, score: i128) -> Option<Input> {
    let ch = match v {
        0 => ' ',
        1 => '#',
        2 => 'B',
        3 => '-',
        4 => 'o',
        _ => ' ',
    };
    window.mvaddch(y as i32, x as i32, ch);
    window.mvprintw(0, 2, score.to_string());
    window.refresh();
    if v == 4 {
        window.getch()
    } else {
        None
    }
}

fn part2(program: &Parsed) -> i128 {
    let window = initscr();
    nl();
    noecho();
    curs_set(0);
    window.keypad(true);
    window.scrollok(true);
    window.timeout(5);
    let mut m = intcode::Machine::new(program);
    *m.memory_mut().get_mut(0).unwrap() = 2;
    let mut score = -1;
    let mut paddle = (0, 0);
    let mut ball = (0, 0);
    let mut dir = 0;
    let mut skip = false;
    loop {
        let state = m.run_to_next_io();
        match state {
            intcode::State::Output => {
                let outputs = m.drain_output();
                let x = outputs[0];
                let y = outputs[1];
                let t = outputs[2];
                if x == -1 && y == 0 {
                    score = t;
                } else {
                    if t == 3 {
                        paddle = (x, y);
                    } else if t == 4 {
                        ball = (x, y);
                    }
                    dir = (ball.0 - paddle.0).signum();
                    if !skip {
                        let x = draw(&window, x, y, t, score);
                        match x {
                            Some(pancurses::Input::Character(' ')) => skip = true,
                            None => {}
                            _ => break,
                        }
                    }
                }
            }
            intcode::State::Input => m.add_input(dir),
            intcode::State::Halted => break,
            _ => panic!(),
        }
    }
    score
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = intcode::parse_intcode(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}
