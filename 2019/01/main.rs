use std::iter::*;

type Parsed = Vec<i64>;

fn get_fuel(mass: i64) -> i64 {
    std::cmp::max((mass / 3) - 2, 0)
}

fn get_fuel_recursive(mass: i64) -> i64 {
    let fuel = (mass / 3) - 2;
    if fuel <= 0 {
        return 0;
    }
    fuel + get_fuel_recursive(fuel)
}

fn part1(masses: &Parsed) -> i64 {
    masses.iter().map(|&m| get_fuel(m)).sum()
}

fn part2(masses: &Parsed) -> i64 {
    masses.iter().map(|&m| get_fuel_recursive(m)).sum()
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|v| v.parse::<i64>().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
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
