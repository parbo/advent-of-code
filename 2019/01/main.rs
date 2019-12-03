use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn get_fuel(mass: i64) -> i64 {
    std::cmp::max((mass / 3) - 2, 0)
}

fn get_fuel_recursive(mass: i64) -> i64 {
    let fuel = (mass / 3) - 2;
    if fuel <= 0 {
        return 0
    }
    return fuel + get_fuel_recursive(fuel);
}

fn part1(masses: &Vec<i64>) -> i64 {
    masses.iter().map(|&m| get_fuel(m)).sum()
}

fn part2(masses: &Vec<i64>) -> i64 {
    masses.iter().map(|&m| get_fuel_recursive(m)).sum()
}

fn input(path: &Path) -> Vec<i64> {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    lines.iter().map(|v| v.parse::<i64>().unwrap()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();
    let filename = &args[2];

    let parsed = input(Path::new(&filename));

    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::{get_fuel, get_fuel_recursive};

    #[test]
    fn fuel() {
        assert_eq!(get_fuel(12), 2);
        assert_eq!(get_fuel(14), 2);
        assert_eq!(get_fuel(1969), 654);
        assert_eq!(get_fuel(100756), 33583);
        assert_eq!(get_fuel_recursive(12), 2);
        assert_eq!(get_fuel_recursive(14), 2);
        assert_eq!(get_fuel_recursive(1969), 966);
        assert_eq!(get_fuel_recursive(100756), 50346);
    }
}
