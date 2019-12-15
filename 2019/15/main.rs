use aoc;
use pancurses::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::*;

fn get_new_pos(pos: (i128, i128), dir: i128) -> (i128, i128) {
    match dir {
        1 => (pos.0, pos.1 - 1),
        2 => (pos.0, pos.1 + 1),
        3 => (pos.0 - 1, pos.1),
        4 => (pos.0 + 1, pos.1),
        _ => panic!(),
    }
}

fn walk(
    m: &mut intcode::Machine,
    path: Vec<i128>,
    pos: (i128, i128),
    seen: &mut HashMap<(i128, i128), i128>,
) -> Option<Vec<i128>> {
    let mut paths = vec![];
    for d in 1..=4 {
        //  north (1), south (2), west (3), and east (4)
        let new_pos = get_new_pos(pos, d);
        if seen.contains_key(&new_pos) {
            continue;
        }
        let mut mc = m.clone();
        mc.add_input(d);
        let out = mc.run_to_next_output().unwrap();
        seen.insert(new_pos, out);
        let pp = match out {
            0 => {
                // Wall, do not expand in this direction
                None
            }
            1 => {
                // Move ok
                let mut p = path.clone();
                p.push(d);
                walk(&mut mc, p, new_pos, seen)
            }
            2 => {
                // Found goal
                let mut p = path.clone();
                p.push(d);
                Some(p)
            }
            _ => panic!(),
        };
        if let Some(p) = pp {
            paths.push(p);
        }
    }
    // Return shortest path, if any
    if paths.len() == 0 {
        None
    } else {
        paths.into_iter().min_by(|a, b| a.len().cmp(&b.len()))
    }
}

fn part1(program: &Vec<i128>) -> i128 {
    let mut m = intcode::Machine::new(program);
    let mut seen = HashMap::new();
    let p = walk(&mut m, vec![], (0, 0), &mut seen);
    p.unwrap().len() as i128
}

fn draw(window: &Window, area: &HashMap<(i128, i128), i128>, x_offs: i128, y_offs: i128) {
    window.clear();
    for ((x, y), col) in area {
        let ch = match col {
            0 => '#',
            1 => '.',
            2 => 'O',
            _ => panic!(),
        };
        window.mvaddch((*y - y_offs) as i32, (*x - x_offs) as i32, ch);
    }
    let _ = window.getch();
    window.refresh();
}

fn part2(program: &Vec<i128>) -> i128 {
    let window = initscr();
    nl();
    noecho();
    curs_set(0);
    window.keypad(true);
    window.scrollok(true);
    window.timeout(20);
    let mut m = intcode::Machine::new(program);
    let mut seen = HashMap::new();
    let _ = walk(&mut m, vec![], (0, 0), &mut seen);
    let min_x = seen.iter().map(|p| (p.0).0).min().unwrap();
    let min_y = seen.iter().map(|p| (p.0).1).min().unwrap();
    let mut minutes = 0;
    let mut expand: Vec<_> = seen.iter().filter(|x| *x.1 == 2).map(|x| *x.0).collect();
    loop {
        minutes += 1;
        let mut new_expand = HashSet::new();
        for pos in &expand {
            for d in 1..=4 {
                let new_pos = get_new_pos(*pos, d);
                let p = seen.entry(new_pos).or_insert(0);
                if *p == 1 {
                    *p = 2;
                    new_expand.insert(new_pos);
                }
            }
        }
        expand = new_expand.into_iter().collect();
        if expand.len() == 0 {
            break;
        }
        draw(&window, &seen, min_x, min_y);
    }
    minutes
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
