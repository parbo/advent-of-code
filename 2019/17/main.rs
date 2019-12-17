use aoc;
use aoc::GridDrawer;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::*;

fn to_char(ch: i128) -> char {
    std::char::from_u32(ch as u32).unwrap()
}

fn is_scaffold(grid: &HashMap<(i128, i128), i128>, pos: (i128, i128)) -> bool {
    if let Some(x) = grid.get(&pos) {
        let ch = to_char(*x);
        let c = match ch {
            '#' => true,
            '<' => true,
            '>' => true,
            '^' => true,
            'v' => true,
            _ => false,
        };
        c
    } else {
        false
    }
}

fn is_crossing(grid: &HashMap<(i128, i128), i128>, pos: (i128, i128)) -> bool {
    let (x, y) = pos;
    // find the +
    is_scaffold(&grid, (x + 1, y))
        && is_scaffold(&grid, (x - 1, y))
        && is_scaffold(&grid, (x, y + 1))
        && is_scaffold(&grid, (x, y - 1))
}

fn find_align(grid: &HashMap<(i128, i128), i128>) -> Vec<(i128, i128)> {
    let scaffold: Vec<_> = grid.iter().filter(|x| *x.1 == 35).map(|x| *x.0).collect();
    let mut align = vec![];
    for p in scaffold {
        if is_crossing(&grid, p) {
            align.push(p);
        }
    }
    align
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Dir {
    Up,
    Left,
    Right,
    Down,
}

fn to_dir(d: char) -> Dir {
    match d {
        '<' => Dir::Left,
        '>' => Dir::Right,
        '^' => Dir::Up,
        'v' => Dir::Down,
        _ => panic!(),
    }
}

fn get_new_pos(pos: (i128, i128), dir: Dir, ch: char) -> ((i128, i128), Dir) {
    match dir {
        Dir::Up => match ch {
            'R' => ((pos.0 + 1, pos.1), Dir::Right),
            'L' => ((pos.0 - 1, pos.1), Dir::Left),
            _ => ((pos.0, pos.1 - 1), Dir::Up),
        },
        Dir::Left => match ch {
            'R' => ((pos.0, pos.1 - 1), Dir::Up),
            'L' => ((pos.0, pos.1 + 1), Dir::Down),
            _ => ((pos.0 - 1, pos.1), Dir::Left),
        },
        Dir::Right => match ch {
            'R' => ((pos.0, pos.1 + 1), Dir::Down),
            'L' => ((pos.0, pos.1 - 1), Dir::Up),
            _ => ((pos.0 + 1, pos.1), Dir::Right),
        },
        Dir::Down => match ch {
            'R' => ((pos.0 - 1, pos.1), Dir::Left),
            'L' => ((pos.0 + 1, pos.1), Dir::Right),
            _ => ((pos.0, pos.1 + 1), Dir::Down),
        },
    }
}

fn visited(start: ((i128, i128), Dir), path: &[i128]) -> HashSet<(i128, i128)> {
    let mut vis = HashSet::new();
    let mut pos = start.0;
    let mut dir = start.1;
    for p in path {
        let np = get_new_pos(pos, dir, to_char(*p));
        vis.insert(np.0);
        pos = np.0;
        dir = np.1
    }
    vis
}

fn print_stuff(grid: &HashMap<(i128, i128), i128>, start: ((i128, i128), Dir), p: &[i128]) {
    let vis = visited(start, &p);
    let mut g = grid.clone();
    for v in &vis {
        g.insert(*v, 42);
    }
    println!("______________________________");
    let mut d = aoc::PrintGridDrawer::new(to_char);
    d.draw(&g);
    println!();
}

fn walk(
    path: Vec<i128>,
    pos: (i128, i128),
    direction: Dir,
    start: ((i128, i128), Dir),
    grid: &HashMap<(i128, i128), i128>,
    seen: &mut HashSet<(i128, i128)>,
    goal: &HashSet<(i128, i128)>,
) -> Vec<Vec<i128>> {
    let mut paths = vec![];
    for d in &[70, 82, 76] {
        let new_pos = get_new_pos(pos, direction, to_char(*d));
        if !seen.insert(new_pos.0) && !is_crossing(grid, new_pos.0) {
            continue;
        }
        let pp = if is_scaffold(grid, new_pos.0) {
            // Move ok
            let mut p = path.clone();
            p.push(*d);
            walk(p, new_pos.0, new_pos.1, start, grid, seen, goal)
        } else {
            let vis = visited(start, &path);
            let diff: HashSet<(i128, i128)> = goal
                .symmetric_difference(&vis)
                .map(|x| x.to_owned())
                .collect();
            if diff.len() == 0 {
                println!("p: {:?}", path);
                print_stuff(grid, start, &path);
                vec![path.clone()]
            } else {
                vec![]
            }
        };
        for p in pp {
            if p.len() > 0 {
                paths.push(p);
            }
        }
    }
    paths
}

fn build_grid(program: &Vec<i128>) -> HashMap<(i128, i128), i128> {
    let mut m = intcode::Machine::new(program);
    let mut grid = HashMap::new();
    let mut y = 0;
    let mut x = 0;
    loop {
        if let Some(ch) = m.run_to_next_output() {
            match ch {
                10 => {
                    y += 1;
                    x = 0;
                }
                c => {
                    grid.insert((x, y), c);
                    x += 1;
                }
            }
        } else {
            break;
        }
    }
    grid
}

fn part1(program: &Vec<i128>) -> i128 {
    let mut d = aoc::PrintGridDrawer::new(to_char);
    let grid = build_grid(program);
    d.draw(&grid);
    let align = find_align(&grid);
    let mut g = grid.clone();
    for p in &align {
        g.insert(*p, 'O' as i128);
    }
    d.draw(&g);
    align.iter().map(|(x, y)| x * y).sum()
}

fn compact(path: &[i128]) -> Vec<i128> {
    let mut v = vec![];
    let mut last = None;
    let mut count = 0;
    for p in path {
        match last {
            None => {
                last = Some(*p);
                count += 1;
            }
            Some(x) => {
                if *p == 70 {
                    count += 1;
                } else {
                    v.push(x);
                    v.push(count);
                    last = Some(*p);
                    count = 1;
                }
            }
        }
    }
    if count > 0 {
        v.push(last.unwrap());
        v.push(count);
    }
    v
}

fn prog_to_str(prog: &[i128]) -> Vec<char> {
    let mut inp = vec![];
    for d in prog {
        match d {
            82 => inp.push('R'),
            76 => inp.push('L'),
            c => {
                for ch in c.to_string().chars() {
                    inp.push(ch);
                }
            }
        }
        inp.push(',');
    }
    if let Some(x) = inp.last_mut() {
        *x = '\n';
    }
    inp
}

fn assemble_seq(
    start: usize,
    end: usize,
    depth: usize,
    rev_index: &HashMap<usize, Vec<&[i128]>>,
    sofar_vec: Vec<Vec<i128>>,
    sofar: HashSet<Vec<i128>>,
) -> Vec<Vec<Vec<i128>>> {
    // println!("{:indent$}assemble: {}, {}, {}, {:?}", "", start, end, sofar.len(), sofar_vec, indent=depth);
    let mut results = vec![];
    if let Some(segments) = rev_index.get(&start) {
        for s in segments {
            if start + s.len() == end {
                let mut sf = sofar_vec.clone();
                sf.push(s.to_vec());
                println!("{:indent$} done: {:?}", "", s, indent = depth);
                results.push(sf);
            } else if start + s.len() < end {
                // Look up the rest
                let sv = s.to_vec();
                let mut new_sofar = sofar.clone();
                new_sofar.insert(sv.clone());
                let mut new_sofar_vec = sofar_vec.clone();
                new_sofar_vec.push(sv.clone());
                if new_sofar.len() <= 3 {
                    let res = assemble_seq(
                        start + s.len(),
                        end,
                        depth + 4,
                        rev_index,
                        new_sofar_vec,
                        new_sofar,
                    );
                    results.extend(res);
                } else {
                    // println!("{:indent$} too many parts!: {:?}", "", s, indent=depth);
                }
            } else {
                println!("{:indent$} too long!: {:?}", "", s, indent = depth);
            }
        }
    } else {
        // println!("{:indent$} done", "", indent=depth);
        results.push(sofar_vec.clone());
    }
    results
}

fn sub_seq(c: &[i128]) -> Vec<(Vec<i128>, char)> {
    let mut rev_index: HashMap<usize, Vec<&[i128]>> = HashMap::new();
    for i in 0..c.len() {
        for j in 1..c.len() {
            let e = i + j;
            if e > c.len() {
                continue;
            }
            let sg = &c[i..e];
            if sg.len() == 0 {
                continue;
            }
            let s = prog_to_str(sg);
            if s.len() > 21 {
                continue;
            }
            rev_index.entry(i).or_insert(Vec::new()).push(sg);
        }
    }
    println!("----------------");
    for (k, v) in &rev_index {
        println!("  {} -> {:?}", k, v);
    }
    println!("===============================================");
    let sofar = HashSet::new();
    let sofar_vec = Vec::new();
    let results = assemble_seq(0, c.len(), 0, &rev_index, sofar_vec, sofar);
    println!("===============================================");
    let ids = ['A', 'B', 'C', 'D', 'E', 'F', 'G'];
    let mut res = vec![];
    for r in &results {
        let mut ress = vec![];
        let mut id = 0;
        let mut char_ids = HashMap::new();
        for seg in r {
            let old = char_ids.contains_key(seg);
            if !old {
                char_ids.entry(seg.clone()).or_insert(ids[id]);
                println!("{} -> {:?}", ids[id], seg);
                id += 1;
            }
        }

        for seg in r {
            print!("{:?}, ", char_ids.get(seg).unwrap());
            ress.push((seg.clone(), *char_ids.get(seg).unwrap()));
        }
        println!();
        res.push(ress);
    }

    println!("===============================================");
    res[0].clone()
}

fn part2(program: &Vec<i128>) -> i128 {
    let grid = build_grid(program);
    let robot: Vec<_> = grid
        .iter()
        .filter(|x| {
            let ch = to_char(*x.1);
            ch == '<' || ch == '>' || ch == '^' || ch == 'v'
        })
        .map(|x| (*x.0, to_dir(to_char(*x.1))))
        .collect();
    let mut seen: HashSet<(i128, i128)> = HashSet::new();
    let goal: HashSet<(i128, i128)> = grid.iter().filter(|x| *x.1 == 35).map(|x| *x.0).collect();
    let paths = walk(
        vec![],
        robot[0].0,
        robot[0].1,
        robot[0],
        &grid,
        &mut seen,
        &goal,
    );
    let mut commands = vec![];
    for path in paths {
        commands.push(compact(&path));
    }
    let mut results = vec![];
    for c in commands {
        println!("==============");
        println!("{:?}", c);
        let res = sub_seq(&c);
        println!("==============");
        for r in &res {
            println!("{:?}", r);
        }
        results.push(res);
    }
    results.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("found: {}", results.len());

    let res = &results[0];

    let mut m = intcode::Machine::new(program);
    *m.memory_mut().get_mut(0).unwrap() = 2;
    let mut inp: Vec<char> = vec![];
    let mut progs: HashMap<char, Vec<i128>> = HashMap::new();
    for i in 0..res.len() {
        progs.insert(res[i].1, res[i].0.clone());
        inp.push(res[i].1);
        inp.push(',');
    }
    *inp.last_mut().unwrap() = '\n';
    for i in &['A', 'B', 'C', 'D', 'E', 'F'] {
        if let Some(p) = progs.get(i) {
            let s = prog_to_str(p);
            for c in s {
                inp.push(c);
            }
        }
    }
    let live = false;
    if live {
        inp.push('y');
    } else {
        inp.push('n');
    }
    inp.push('\n');
    for i in &inp {
        print!("{}", i);
    }
    for c in inp {
        m.add_input(c as i128);
    }
    let mut res = 0;
    loop {
        let state = m.run_to_next_io();
        if state != intcode::State::Output {
            break;
        }
        let o = m.outputs();
        for c in o {
            if c > 255 {
                res = c;
                println!("{}", c);
            } else {
                print!("{}", to_char(c));
            }
        }
    }
    res
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
