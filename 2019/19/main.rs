use aoc;
use aoc::GridDrawer;
use std::iter::*;
use std::collections::HashMap;

fn part1(program: &Vec<i128>) -> i128 {
    let mut grid = HashMap::new();
    for y in 0..50 {
	for x in 0..50 {
	    let mut m = intcode::Machine::new(program);
	    m.add_input(x as i128);
	    m.add_input(y as i128);
	    if let Some(v) = m.run_to_next_output() {
		grid.insert((x, y), v);
	    }
	}
    }
    grid.iter().filter(|(_, v)| **v == 1).count() as i128
}

fn to_char(ch: i128) -> char {
    if ch == 1 {
	'#'
    } else {
	'.'
    }
}

fn get_width_at(program: &Vec<i128>, width: i128, beg: i128, end: i128) -> Option<(i128, i128, i128)> {
    let mut a = beg;
    let mut b = end;
    let res = loop {
	let m = (a + b) / 2;
	let mut s = 0;
	let mut e = 0;
	let mut x = 0;
	loop {
	    let mut mach = intcode::Machine::new(program);
	    mach.add_input(x as i128);
	    mach.add_input(m as i128);
	    if let Some(v) = mach.run_to_next_output() {
		if v == 1 && s == 0 {
		    s = x;
		}
		if v == 0 && e == 0 {
		    e = x;
		    break;
		}
	    }
	    x += 1;
	}
	if e - s < width {
	    a = m + 1;
	} else if e - s > width {
	    b = m - 1;
	} else if e - s == width {
	    break Some((s, e, m));
	}
	if a == b && e - s != width {
	    break None;
	}
    };
    res
}

fn get_start_at(program: &Vec<i128>, start: i128, beg: i128, end: i128) -> Option<(i128, i128, i128)> {
    let mut a = beg;
    let mut b = end;
    let res = loop {
	let m = (a + b) / 2;
	let mut s = 0;
	let mut x = 0;
	loop {
	    let mut mach = intcode::Machine::new(program);
	    mach.add_input(x as i128);
	    mach.add_input(m as i128);
	    if let Some(v) = mach.run_to_next_output() {
		if v == 1 && s == 0 {
		    s = x;
		    break;
		}
	    }
	    x += 1;
	}
	if s < start {
	    a = m + 1;
	} else if s > start {
	    b = m - 1;
	} else if s == start {
	    break Some((s, m));
	}
	if a == b && s != start {
	    break None;
	}
    };
}

fn part2(program: &Vec<i128>) -> i128 {
    let mut y = 1200;
    // let mut widths = HashMap::new();
    loop {
	if let Some((s, e, yy)) = get_width_at(program, 150, y, y + 500) {
	    if let Some((ss, yyy)) = get_start_at(program, e - 100, yy, yy + 500) {
		println!("Found at {}, {}; {} {} {}", ss, yyy, s, e, yy);
		break;
	    } else {
		println!("not found at {}, {}, {}", s, e, yy);
	    }
	}
	println!("not found at {}", y);
	y += 1;
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
