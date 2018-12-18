use std::env;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn draw(grid: &Vec<Vec<char>>) {
    println!("----------------");
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }
}

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut grids = vec![];
    grids.push(vec![]);
    grids.push(vec![]);
    for line in lines {
        let row : Vec<char> = line.chars().cloned().collect();
        let empty : Vec<char> = vec![];
        empty.resize(row.len(), '.');
        grids[0].push(row);
        grids[1].push(empty);
    }
    // An open acre will become filled with trees if three or more adjacent acres contained trees. Otherwise, nothing happens.
    // An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards. Otherwise, nothing happens.
    // An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other lumberyard and at least one acre containing trees. Otherwise, it becomes open.
    let mut curr = 0;
    let mut minutes = 0;
    loop {
        let g = &grids[curr];
        if curr == 0 {
            curr = 1;
        } else {
            curr = 0;
        }
        let mut n = &grids[curr];
        for (y, row) in grid.iter().enumerate() {
            if y < miny - 1 {
                continue;
            }
            for (y, row) in g.iter().enumerate() {
                for (x, col) in row.iter().enumerate() {
                    let mut trees = 0;
                    let mut lumberyards = 0;
                    for ny in ((y-1) as i64)..((y+2) as i64) {
                        if ny < 0 || ny > (g.len() as i64) {
                            continue;
                        }
                        for nx in ((x-1) as i64)..((x+2) as i64) {
                            if nx < 0 || nx > (row.len() as i64) {
                                continue;
                            }
                            match col {
                                '|' => trees += 1,
                                '#' => lumberyards += 1;
                            }
                        }
                    }
                    match col {
                        '.' => {
                            if trees >= 3 {
                                n[y][x] = '|';
                            } else {
                                n[y][x] = '.';
                            }
                        },
                        '|' => {
                            if lumberyards >= 3 {
                                n[y][x] = '#';
                            } else {
                                n[y][x] = '|';
                            }
                        },
                        '#' => {
                            if trees >= 1 && lumberyards >= 1 {
                                n[y][x] = '#';
                            } else {
                                n[y][x] = '.';
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
