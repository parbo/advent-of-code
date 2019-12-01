use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn get_fuel(mass: i64) -> i64 {
    std::cmp::max((mass / 3) - 2, 0)
}

fn solve(path: &Path) -> i64 {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let result = lines.iter().map(|v| v.parse::<i64>().unwrap()).map(|m| get_fuel(m)).sum();
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::get_fuel;

    #[test]
    fn fuel() {
        assert_eq!(get_fuel(12), 2);
        assert_eq!(get_fuel(14), 2);
        assert_eq!(get_fuel(1969), 654);
        assert_eq!(get_fuel(100756), 33583);
    }
}
