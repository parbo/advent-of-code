use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn run(result: &mut Vec<usize>) -> Option<usize> {
    let mut pos : usize = 0;
    while result[pos] != 99 {
        let op = result[pos];
        match op {
            1 => {
                let res = result[result[pos + 1]] + result[result[pos + 2]];
                let p = result[pos + 3];
                result[p] = res;
            },
            2 => {
                let res = result[result[pos + 1]] * result[result[pos + 2]];
                let p = result[pos + 3];
                result[p] = res;
            }
            _ => {
                return None
            }
        }
        pos += 4;
    }
    Some(result[0])
}

fn run_all(numbers: &Vec<usize>) -> Option<(usize, usize)> {
    for ai in 0..=99 {
        for bi in 0..=99 {
            let mut num = numbers.clone();
            // Init
            num[1] = ai;
            num[2] = bi;
            let res = run(&mut num);
            if res == Some(19690720) {
                return Some((ai, bi));
            }
        }
    }
    None
}

fn part1(numbers: &Vec<usize>) -> usize {
    let mut num = numbers.clone();
    // Init
    num[1] = 12;
    num[2] = 02;
    run(&mut num).unwrap()
}

fn part2(numbers: &Vec<usize>) -> usize {
    let (noun, verb) = run_all(numbers).unwrap();
    100 * noun + verb
}

fn input(path: &Path) -> Vec<usize> {
    let mut inp = File::open(path).unwrap();
    let mut buffer = String::new();
    inp.read_to_string(&mut buffer).unwrap();

    let result : Vec<usize> = buffer.split(|c| c == ',').map(|s| s.trim()).map(|v| v.parse::<usize>().unwrap()).collect();
    result
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
    use super::run;

    #[test]
    fn test() {
        let mut input = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        assert_eq!(run(&mut input), Some(3500));
    }
}
