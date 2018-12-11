use std::env;

fn power_level(x: i64, y: i64, serial: i64) -> i64 {
    let rack = x + 10;
    let mut power = rack * y;
    power += serial;
    power = power * rack;
    let hundreds = (power - 1000 * (power / 1000)) / 100;
    return hundreds - 5;
}

fn solve(serial: i64) -> (i64, i64) {
    let mut grid = vec![];
    grid.resize(300 * 300, 0);
    for y in 1..301 {
        for x in 1..301 {
            let offset = (y - 1) * 300 + (x - 1);
            grid[offset as usize] = power_level(x, y, serial);
        }
    }
    let mut max_power = 0;
    let mut max_coord = (0, 0);
    for y in 1..299 {
        for x in 1..299 {
            let offset = ((y - 1) * 300 + (x - 1)) as usize;
            let p = grid[offset] + grid[offset+1] + grid[offset+2] +
                grid[300 + offset] + grid[300 + offset+1] + grid[300 + offset+2] +
                grid[600 + offset] + grid[600 + offset+1] + grid[600 + offset+2];
            if p > max_power {
                max_power = p;
                max_coord = (x, y);
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
