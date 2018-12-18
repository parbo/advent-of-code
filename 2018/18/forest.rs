use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
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
        let row : Vec<char> = line.chars().collect();
        let mut empty : Vec<char> = vec![];
        empty.resize(row.len(), '.');
        grids[0].push(row);
        grids[1].push(empty);
    }
    // An open acre will become filled with trees if three or more adjacent acres contained trees. Otherwise, nothing happens.
    // An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards. Otherwise, nothing happens.
    // An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other lumberyard and at least one acre containing trees. Otherwise, it becomes open.
    let mut curr = 0;
    let mut next = 1;
    let mut minutes = 0;
    let w = grids[0][0].len();
    let h = grids[0].len();
    let mut seen : HashMap<Vec<Vec<char>>, (usize, usize)> = HashMap::new();
    loop {
        let mut total_trees = 0;
        let mut total_lumberyards = 0;
        for y in 0..h {
            for x in 0..w {
                let mut trees = 0;
                let mut lumberyards = 0;
                for ny in (y as i64 - 1)..(y as i64 + 2) {
                    if ny < 0 || ny >= (h as i64) {
                        continue;
                    }
                    for nx in (x as i64 - 1)..(x as i64 + 2) {
                        if nx < 0 || nx >= (w as i64) {
                            continue;
                        }
                        if nx as usize == x && ny as usize == y {
                            continue;
                        }
                        match grids[curr][ny as usize][nx as usize] {
                            '|' => trees += 1,
                            '#' => lumberyards += 1,
                            _ => {}
                        }
                    }
                }
                let col = grids[curr][y][x];
                match col {
                    '.' => {
                        if trees >= 3 {
                            grids[next][y][x] = '|';
                            total_trees += 1;
                        } else {
                            grids[next][y][x] = '.';
                        }
                    },
                    '|' => {
                        if lumberyards >= 3 {
                            grids[next][y][x] = '#';
                            total_lumberyards += 1;
                        } else {
                            grids[next][y][x] = '|';
                            total_trees += 1;
                        }
                    },
                    '#' => {
                        if trees >= 1 && lumberyards >= 1 {
                            grids[next][y][x] = '#';
                            total_lumberyards += 1;
                        } else {
                            grids[next][y][x] = '.';
                        }
                    },
                    _ => panic!()
                }
            }
        }
        minutes += 1;
        if let Some(v) = seen.get(&grids[next]) {
            println!("loop found: {:?} -> {}", v, minutes);
            let loop_len = minutes - v.0;
            println!("loop length: {}", loop_len);
            let rem = (1000000000 - minutes) % loop_len;
            for (_, m) in &seen {
//                println!("m: {:?}, v: {:?}, rem: {}", m, v, rem);
                if v.0 + rem == m.0 {
                    println!("minutes: {}", m.1)
                }
            }
            break;
        }
        seen.insert(grids[next].clone(), (minutes, total_lumberyards * total_trees));
        std::mem::swap(&mut curr, &mut next);
//        draw(&grids[next]);
        // if minutes == 10 {
        //     println!("{} {} {}", total_lumberyards, total_trees, total_lumberyards * total_trees);
        //     break;
        // }
        if minutes % 1000 == 0 {
            println!("minutes: {}", minutes);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
