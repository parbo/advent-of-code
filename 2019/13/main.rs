use aoc;
// use intcode;
use std::iter::*;
use std::collections::HashMap;
use pancurses::*;

// 0 is an empty tile. No game object appears in this tile.
// 1 is a wall tile. Walls are indestructible barriers.
// 2 is a block tile. Blocks can be broken by the ball.
// 3 is a horizontal paddle tile. The paddle is indestructible.
// 4 is a ball tile. The ball moves diagonally and bounces off objects.

fn part1(program: &Vec<i128>) -> i128 {
    let mut m = intcode::Machine::new(&program);
    let mut blocks = HashMap::new();
    loop {
	let x = m.run_to_next_output();
	let y = m.run_to_next_output();
	let tile = m.run_to_next_output();
	if x.is_some() && y.is_some() && tile.is_some() {
	    blocks.insert((x.unwrap(), y.unwrap()), tile.unwrap());
	} else {
	    break;
	}
    }
    blocks.iter().map(|(_k, v)| *v).filter(|v| *v == 2).count() as i128
}

fn draw(window: &Window, hull: &HashMap<(i128, i128), i128>) {
    window.clear();
    for ((x, y), col) in hull {
	let ch = match col {
            0 => ' ',
            1 => '#',
            2 => 'B',
            3 => '-',
            4 => 'o',
	    _ => ' ',
	};
        window.mvaddch(*y as i32, *x as i32, ch);
    }
    let _ = window.getch();
    window.refresh();
}

fn print_screen(screen: &HashMap<(i128, i128), i128>) {
    let min_x = screen.iter().map(|p| (p.0).0).min().unwrap();
    let min_y = screen.iter().map(|p| (p.0).1).min().unwrap();
    let max_x = screen.iter().map(|p| (p.0).0).max().unwrap();
    let max_y = screen.iter().map(|p| (p.0).1).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match screen.get(&(x, y)) {
                Some(0) => print!(" "),
                Some(1) => print!("#"),
                Some(2) => print!("B"),
                Some(3) => print!("-"),
                Some(4) => print!("o"),
		None => print!(" "),
                _ => panic!(),
            }
        }
        println!();
    }
}

fn part2(program: &Vec<i128>) -> i128 {
    let window = initscr();
    nl();
    noecho();
    curs_set(0);
    window.keypad(true);
    window.scrollok(true);
    window.timeout(20);
    let mut m = intcode::Machine::new(&program);
    *m.memory_mut().get_mut(0).unwrap() = 2;
    let mut screen = HashMap::new();
    let mut score = -1;
    let mut paddle = (0, 0);
    let mut ball = (0, 0);
    let mut dir = 0;
//    m.add_input(dir);
    loop {
	let x = m.run_to_next_output();
	let y = m.run_to_next_output();
	let tile = m.run_to_next_output();
	if x.is_some() && y.is_some() && tile.is_some() {
//	    println!("{:?}, {:?}, {:?}", x, y, tile);
	    let t = tile.unwrap();
	    if x.unwrap() == -1 && y.unwrap() == 0  {
		if score == -1 {
		    // first screen complete
		    //		    print_screen(&screen);
		    draw(&window, &screen);
//		    m.add_input(dir);
		}
//		println!("score: {} -> {}", score, t);
		score = t;
	    } else {
		screen.insert((x.unwrap(), y.unwrap()), t);
		if t == 3 {
		    paddle = (x.unwrap(), y.unwrap());
		    if ball.0 - paddle.0 > 0 {
			dir = 1;
		    } else if ball.0 - paddle.0 == 0 {
			dir = 0;
		    } else {
			dir = -1;
		    };
		} else if t == 4 {
		    ball = (x.unwrap(), y.unwrap());
		    if ball.0 - paddle.0 > 0 {
			dir = 1;
		    } else if ball.0 - paddle.0 == 0 {
			dir = 0;
		    } else {
			dir = -1;
		    };
		    if score >= 0 {
			//			print_screen(&screen);
			draw(&window, &screen);
		    }
//		    println!("{}", dir);
		    m.add_input(dir);
		}
	    }
	} else {
	    break;
	}
    }
    score
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

#[cfg(test)]
mod tests {
    // use super::part1;

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }
}
