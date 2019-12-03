use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn part1(things: &Vec<i64>) -> i64 {
    things[0]
}

fn part2(things: &Vec<i64>) -> i64 {
    things[0]
}

fn input(path: &Path) -> Vec<i64> {
    // let input = File::open(path).unwrap();
    // let buffered = BufReader::new(input);
    // let mut buffer = String::new();
    // input.read_to_string(&mut buffer).unwrap();
    // let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    // lines.iter().map(|line| segments(&line)).collect()
    vec![0]
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
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(&vec![0]), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![0]), 0);
    }
}
