use aoc::GridDrawer;
use std::collections::HashMap;
use std::iter::*;

type Parsed = Vec<i128>;

fn to_char(col: i128) -> char {
    match col {
        0 => '#',
        1 => '.',
        2 => 'O',
        _ => panic!(),
    }
}

fn get_new_pos(pos: aoc::Point, dir: i128) -> aoc::Point {
    match dir {
        1 => [pos[0], pos[1] - 1],
        2 => [pos[0], pos[1] + 1],
        3 => [pos[0] - 1, pos[1]],
        4 => [pos[0] + 1, pos[1]],
        _ => panic!(),
    }
}

fn walk(
    m: &mut intcode::Machine,
    path: Vec<i128>,
    pos: aoc::Point,
    seen: &mut HashMap<aoc::Point, i128>,
    drawer: &mut dyn aoc::GridDrawer<HashMap<aoc::Point, i128>, i128>,
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
        drawer.draw(seen);
        let pp = match out {
            0 => {
                // Wall, do not expand in this direction
                None
            }
            1 => {
                // Move ok
                let mut p = path.clone();
                p.push(d);
                walk(&mut mc, p, new_pos, seen, drawer)
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
    if paths.is_empty() {
        None
    } else {
        paths.into_iter().min_by(|a, b| a.len().cmp(&b.len()))
    }
}

fn part1(program: &Parsed) -> i128 {
    let mut m = intcode::Machine::new(program);
    let mut seen = HashMap::new();
    let p = walk(
        &mut m,
        vec![],
        [0, 0],
        &mut seen,
        &mut aoc::NopGridDrawer {},
    );
    p.unwrap().len() as i128
}

fn part2(program: &Parsed) -> i128 {
    let mut d = aoc::CursesGridDrawer::new(to_char);
    let mut m = intcode::Machine::new(program);
    let mut seen = HashMap::new();
    let _ = walk(&mut m, vec![], [0, 0], &mut seen, &mut d);
    let mut minutes = 0;
    let mut expand: Vec<_> = seen.iter().filter(|x| *x.1 == 2).map(|x| *x.0).collect();
    loop {
        minutes += 1;
        let mut new_expand = vec![];
        for pos in &expand {
            for d in 1..=4 {
                let new_pos = get_new_pos(*pos, d);
                let p = seen.entry(new_pos).or_insert(0);
                if *p == 1 {
                    *p = 2;
                    new_expand.push(new_pos);
                }
            }
        }
        expand = new_expand;
        if expand.is_empty() {
            break;
        }
        d.draw(&seen);
    }
    minutes
}

fn main() {
    aoc::run_main(intcode::parse_intcode, part1, part2);
}
