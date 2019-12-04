use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn check(number: i64) -> bool {
    let mut prev = None;
    let mut two_same = false;
    let mut num = number;
    while num != 0 {
        let digit = num % 10;
        if let Some(p) = prev {
            if digit > p {
                return false;
            }
            if digit == p {
                two_same = true;
            }
        }
        prev = Some(digit);
        num /= 10;
    }
    return two_same;
}

fn check2(number: i64) -> bool {
    let mut prev = None;
    let mut count_same = vec![1];
    let mut num = number;
    while num != 0 {
        let digit = num % 10;
        if let Some(p) = prev {
            if digit > p {
                return false;
            }
            if digit == p {
                *count_same.last_mut().unwrap() += 1;
            } else {
                count_same.push(1);
            }
        }
        prev = Some(digit);
        num /= 10;
    }
    count_same.iter().filter(|&&x| x == 2).count() > 0
}

fn part1(range: (i64, i64)) -> i64 {
    let (low, high) = range;
    let r = low..=high;
    r.filter(|&x| check(x)).count() as i64
}

fn part2(range: (i64, i64)) -> i64 {
    let (low, high) = range;
    let r = low..=high;
    r.filter(|&x| check2(x)).count() as i64
}

fn input(path: &Path) -> (i64, i64) {
    let mut input = File::open(path).unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let range : Vec<_> = buffer.split('-').map(|x| x.trim().parse::<i64>()).filter_map(Result::ok).collect();
    (range[0], range[1])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();
    let filename = &args[2];

    let parsed = input(Path::new(&filename));

    let result = if part == 1 {
        part1(parsed)
    } else {
        part2(parsed)
    };
        println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::{check, check2};

    #[test]
    fn test_part1() {
        assert_eq!(check(111111), true);
        assert_eq!(check(567789), true);
        assert_eq!(check(223450), false);
        assert_eq!(check(123789), false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(check2(112233), true);
        assert_eq!(check2(123444), false);
        assert_eq!(check2(111122), true);
        assert_eq!(check2(111223), true);
    }
}
