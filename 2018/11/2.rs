use std::cmp;
use std::env;

fn power_level(x: i64, y: i64, serial: i64) -> i64 {
    let rack = x + 10;
    let mut power = rack * y;
    power += serial;
    power = power * rack;
    let hundreds = (power - 1000 * (power / 1000)) / 100;
    return hundreds - 5;
}

fn solve(serial: i64) -> (i64, i64, i64) {
    let mut grid = vec![];
    grid.resize(300 * 300, 0);
    for y in 1..301 {
        for x in 1..301 {
            let offset = (y - 1) * 300 + (x - 1);
            grid[offset as usize] = power_level(x, y, serial);
        }
    }
    let mut max_power = 0;
    let mut max_coord = (0, 0, 0);
    for y in 1..301 {
//        println!("y: {}", y);
        for x in 1..301 {
            let max_s = cmp::min(301 - y, 301 - x);
            let mut p = 0;
            for s in 1..max_s {
                for i in 0..s {
                    let oa = (y + (s - 1) - 1) * 300 + (x + i - 1);
                    let ob = (y + i - 1) * 300 + (x + (s - 1) - 1);
                    p += grid[oa as usize];
                    if oa != ob {
                        p += grid[ob as usize];
                    }
                }
                if p > max_power {
                    max_power = p;
                    max_coord = (x, y, s);
                }
            }
        }
    }
    return max_coord;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let serial = args[1].parse::<i64>().unwrap();

    println!("{}", power_level(3, 5, 8));
    println!("{}", power_level(122, 79, 57));
    println!("{}", power_level(217, 196, 39));
    println!("{}", power_level(101, 153, 71));

    let result = solve(serial);
    println!("{:?}", result);
}
