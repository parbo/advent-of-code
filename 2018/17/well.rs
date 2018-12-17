use std::env;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

static EMPTY : [char;2] = ['.', '|'];
static FIRM : [char;2] = ['#', '~'];

fn fill_left(grid: &mut Vec<Vec<char>>, sources: &mut HashSet<(usize, usize)>, sy: usize, sx: usize, minx: usize, maxx: usize, miny: usize, maxy: usize) -> bool {
    // if sources.contains(&(sy, sx)) {
    //     return false;
    // }
    // sources.insert((sy, sx));
    if sx < 1 {
        return false;
    }
//    println!("left: {}, {}", sx, sy);
    let wy = sy;
    let mut wx = sx;
    let mut any_change = false;
    // Move left
    while wx > 1 && EMPTY.contains(&grid[wy][wx-1]) && FIRM.contains(&grid[wy+1][wx-1]) {
        wx -= 1;
        if grid[wy][wx] != '|' {
            grid[wy][wx] = '|';
            any_change = true;
        }
    }
    if FIRM.contains(&grid[wy+1][wx-1]) {
        let x = wx;
        while wx + 1 < maxx && FIRM.contains(&grid[wy+1][wx]) && EMPTY.contains(&grid[wy][wx+1]) {
            wx += 1;
            if grid[wy][wx] != '|' {
                grid[wy][wx] = '|';
                any_change = true;
            }
        }
        if wx + 1 < maxx && FIRM.contains(&grid[wy][wx+1]) {
            for xx in x..(wx+1) {
                if grid[wy][xx] != '~' {
                    grid[wy][xx] = '~';
                    any_change = true;
                }
            }
        } else {
 //           println!("L Go down!");
            if fill_down(grid, sources, wy, wx, minx, maxx, miny, maxy) {
                any_change = true;
            }
        }
    } else {
//        println!("L Go down 2!");
        if fill_down(grid, sources, wy, wx-1, minx, maxx, miny, maxy) {
            any_change = true;
        }
    }
    return any_change;
}

fn fill_right(grid: &mut Vec<Vec<char>>, sources: &mut HashSet<(usize, usize)>, sy: usize, sx: usize, minx: usize, maxx: usize, miny: usize, maxy: usize) -> bool {
    // if sources.contains(&(sy, sx)) {
    //     return false;
    // }
    // sources.insert((sy, sx));
    if sx >= maxx {
        return false;
    }
//    println!("right: {}, {}", sx, sy);
    let wy = sy;
    let mut wx = sx;
    let mut any_change = false;
    // Move Right
    while wx + 1 < maxx && EMPTY.contains(&grid[wy][wx+1]) && FIRM.contains(&grid[wy+1][wx+1]) {
        wx += 1;
        if grid[wy][wx] != '|' {
            grid[wy][wx] = '|';
            any_change = true;
        }
    }
    if FIRM.contains(&grid[wy+1][wx+1]) {
        let x = wx;
        while wx > 1 && FIRM.contains(&grid[wy+1][wx]) && EMPTY.contains(&grid[wy][wx-1]) {
            wx -= 1;
            if grid[wy][wx] != '|' {
                grid[wy][wx] = '|';
                any_change = true;
            }
        }
        if wx > 1 && FIRM.contains(&grid[wy][wx-1]) {
            for xx in wx..(x+1) {
                if grid[wy][xx] != '~' {
                    grid[wy][xx] = '~';
                    any_change = true;
                }
            }
        } else {
  //          println!("R Go down!");
            if fill_down(grid, sources, wy, wx, minx, maxx, miny, maxy) {
                any_change = true;
            }
        }
    } else {
    //    println!("R Go down 2!");
        if fill_down(grid, sources, wy, wx+1, minx, maxx, miny, maxy) {
            any_change = true;
        }
    }
    return any_change;
}

fn fill_down(grid: &mut Vec<Vec<char>>, sources: &mut HashSet<(usize, usize)>, sy: usize, sx: usize, minx: usize, maxx: usize, miny: usize, maxy: usize) -> bool {
    if sources.contains(&(sy, sx)) {
        return false;
    }
    sources.insert((sy, sx));
    if sy > maxy {
        return false;
    }
//    println!("down: {}, {}, {}, {}", sx, maxx, sy, maxy);
    let mut wy = sy;
    let wx = sx;
    let mut any_change = false;
    // Fill down
    if wy < maxy && EMPTY.contains(&grid[wy+1][wx]) {
        // Move down
        while wy < maxy && EMPTY.contains(&grid[wy+1][wx]) {
            if grid[wy][wx] != '|' {
                grid[wy][wx] = '|';
                any_change = true;
            }
            wy += 1;
        }
    }
    if wy >= maxy {
        if grid[wy][wx] != '|' {
            grid[wy][wx] = '|';
            any_change = true;
        }
        return any_change;
    }
    if wx > 0 && EMPTY.contains(&grid[wy][wx-1]) && EMPTY.contains(&grid[wy][wx]) {
        // if grid[wy][wx] != '|' {
        //     grid[wy][wx] = '|';
        //     any_change = true;
        // }
        if fill_left(grid, sources, wy, wx, minx, maxx, miny, maxy) {
            any_change = true;
        }
    }
    if wx < maxx && EMPTY.contains(&grid[wy][wx+1]) && EMPTY.contains(&grid[wy][wx]) {
        // if grid[wy][wx] != '|' {
        //     grid[wy][wx] = '|';
        //     any_change = true;
        // }
        if fill_right(grid, sources, wy, wx, minx, maxx, miny, maxy) {
            any_change = true;
        }
    }
    return any_change;
}

fn fill(grid: &mut Vec<Vec<char>>, sy: usize, sx: usize, minx: usize, maxx: usize, miny: usize, maxy: usize) {
    loop {
        let mut sources : HashSet<(usize, usize)> = HashSet::new();
        if !fill_down(grid, &mut sources, sy, sx, minx, maxx, miny, maxy) {
            break;
        }
        if grid.len() < 20 {
            println!("----------------");
            for (y, row) in grid.iter().enumerate() {
                if y < miny {
                    continue;
                }
                for (x, col) in row.iter().enumerate() {
                    if x < minx {
                        continue;
                    }
                    print!("{}", col);
                }
                println!("");
            }
        }
    }
}

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut miny = std::usize::MAX;
    let mut maxy = 0;
    let mut minx = std::usize::MAX;
    let mut maxx = 0;
    let mut rects = vec![];
    for line in lines {
        let comma = line[2..].find(',').unwrap() + 2;
        let a = line[2..comma].parse::<usize>().unwrap();
        let dotdot = line[comma..].find("..").unwrap() + comma;
        let b = line[(comma+4)..dotdot].parse::<usize>().unwrap();
        let c = line[(dotdot+2)..].parse::<usize>().unwrap();
        if line[0..1] == *"x" {
            rects.push(((b, c), (a, a)));
            minx = std::cmp::min(minx, a);
            maxx = std::cmp::max(maxx, a);
            miny = std::cmp::min(miny, b);
            maxy = std::cmp::max(maxy, b);
            miny = std::cmp::min(miny, c);
            maxy = std::cmp::max(maxy, c);
        } else if line[0..1] == *"y" {
            rects.push(((a, a), (b, c)));
            miny = std::cmp::min(miny, a);
            maxy = std::cmp::max(maxy, a);
            minx = std::cmp::min(minx, b);
            maxx = std::cmp::max(maxx, b);
            minx = std::cmp::min(minx, c);
            maxx = std::cmp::max(maxx, c);
        }
    }
    println!("{}, {}, {}, {}", miny, maxy, minx, maxx);
    // Make grid
    let mut grid : Vec<Vec<char>> = vec![];
    for _ in 0..(maxy+1) {
        let mut r : Vec<char> = vec![];
        r.resize(maxx+1, '.');
        grid.push(r);
    }
    println!("y: {}, x: {}", grid.len(), grid[0].len());
    // Fill grid
    for r in rects {
        let ((y1, y2), (x1, x2)) = r;
        println!("{:?}", r);
        for y in y1..(y2+1) {
            for x in x1..(x2+1) {
                grid[y][x] = '#';
            }
        }
    }
    if grid.len() < 20 {
        for (y, row) in grid.iter().enumerate() {
            if y < miny {
                continue;
            }
            for (x, col) in row.iter().enumerate() {
                if x < minx {
                    continue;
                }
                print!("{}", col);
            }
            println!("");
        }
    }
    // fill it up
    fill(&mut grid, 0, 500, minx, maxx, miny, maxy);
    // calc the sum
    let mut sum = 0;
    for (y, row) in grid.iter().enumerate() {
        if y < miny || y > maxy {
            continue;
        }
        for (x, _) in row.iter().enumerate() {
            let c = grid[y][x];
            if c == '|' || c == '~' {
                sum += 1;
            }
        }
    }
    println!("water: {}", sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
