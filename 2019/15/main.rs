use aoc;
use std::collections::HashMap;
use std::iter::*;

fn walk(
    m: &mut intcode::Machine,
    path: Vec<i128>,
    pos: (i128, i128),
    seen: &mut HashMap<(i128, i128), i128>,
) -> Option<Vec<i128>> {
    let mut paths = vec![];
    for d in 1..=4 {
        //  north (1), south (2), west (3), and east (4)
        let new_pos = match d {
            1 => (pos.0, pos.1 - 1),
            2 => (pos.0, pos.1 + 1),
            3 => (pos.0 - 1, pos.1),
            4 => (pos.0 + 1, pos.1),
            _ => panic!(),
        };
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

fn print_area(area: &HashMap<(i128, i128), i128>) {
    let min_x = area.iter().map(|p| (p.0).0).min().unwrap();
    let min_y = area.iter().map(|p| (p.0).1).min().unwrap();
    let max_x = area.iter().map(|p| (p.0).0).max().unwrap();
    let max_y = area.iter().map(|p| (p.0).1).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match area.get(&(x, y)) {
                Some(c) => match c {
                    0 => print!("#"),
                    1 => print!("."),
                    2 => print!("O"),
                    _ => panic!(),
                },
                None => print!(" "),
            };
        }
        println!();
    }
}

fn part2(program: &Vec<i128>) -> i128 {
    let mut m = intcode::Machine::new(program);
    let mut seen = HashMap::new();
    let _ = walk(&mut m, vec![], (0, 0), &mut seen);
    let mut minutes = 0;
    loop {
        minutes += 1;
        let mut expand = vec![];
        for (pos, v) in &seen {
            if *v == 2 {
                expand.push(*pos);
            }
        }
        for pos in &expand {
            for d in 1..=4 {
                let new_pos = match d {
                    1 => (pos.0, pos.1 - 1),
                    2 => (pos.0, pos.1 + 1),
                    3 => (pos.0 - 1, pos.1),
                    4 => (pos.0 + 1, pos.1),
                    _ => panic!(),
                };
                let p = seen.entry(new_pos).or_insert(0);
                if *p == 1 {
                    *p = 2;
                }
            }
        }
        let c = seen.iter().filter(|x| *x.1 == 1).count();
        if c == 0 {
            break;
        }
        print_area(&seen);
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
