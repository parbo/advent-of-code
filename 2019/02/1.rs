use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;



fn solve(path: &Path) -> Vec<usize> {
    let mut input = File::open(path).unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let result : Vec<usize> = buffer.split(|c| c == ',').map(|s| s.trim()).map(|v| v.parse::<usize>().unwrap()).collect();
    result
}

fn run(result: &mut Vec<usize>) -> usize {
    let mut pos : usize = 0;
    while result[pos] != 99 {
        let op = result[pos];
//        println!("{}, {}, {:?}", pos, result[pos], result);
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
            _ => panic!("OH NOES")
        }
        pos += 4;
    }
//    println!("{}, {}, {:?}", pos, result[pos], result);
    result[0]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut numbers = solve(Path::new(&filename));
    // Init
    numbers[1] = 12;
    numbers[2] = 2;

    let result = run(&mut numbers);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test() {
        let mut input = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        assert_eq!(run(&mut input), 3500);
    }
}
