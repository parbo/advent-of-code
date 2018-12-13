use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn draw_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }
}

#[derive(Debug)]
struct Car {
    dir: char,
    pos: (usize, usize),
    next_turn: char
}

fn tick(cars: &mut Vec<Car>, grid: &Vec<Vec<char>>) {
    for c in &mut cars {
        // maybe turn
        if grid[c.pos.1][c.pos.0] == '+' {
            c.dir = c.next_dir;
            match c.next_dir {
                '>' => c.next_dir = c.next_dir,
                '<' => c.dirpos = (c.pos.0 - 1, c.pos.1),
                '^' => c.pos = (c.pos.0, c.pos.1 - 1),
                'v' => c.pos = (c.pos.0, c.pos.1 + 1),
            }
        }
        // move
        match c {
            '>' => c.pos = (c.pos.0 + 1, c.pos.1),
            '<' => c.pos = (c.pos.0 - 1, c.pos.1),
            '^' => c.pos = (c.pos.0, c.pos.1 - 1),
            'v' => c.pos = (c.pos.0, c.pos.1 + 1),
        }
    }
}

fn solve(path: &Path) -> (i64, i64) {
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
        let row_cars : Vec<Car>= line.chars().enumerate().filter(|(_, c)| ['<', '>', 'v', '^'].contains(&c)).map(|(col, c)| Car {dir: c, pos: (col, row), next_dir: '<' }).collect();
        cars.extend(row_cars);
        let v : Vec<char> = line.chars().map(|c| *m.get(&c).unwrap_or(&c)).collect();
        max_w = std::cmp::max(v.len(), max_w);
        grid.push(v);
    }
    for row in &mut grid {
        row.resize(max_w, ' ');
    }
    println!("{:?}", cars);
//    draw_grid(&grid);
    return (-1, -1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{:?}", result);
}
