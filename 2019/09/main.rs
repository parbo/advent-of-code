use aoc;
use intcode;

fn part1(numbers: &Vec<i128>) -> i128 {
    let mut m = intcode::Machine::with_input(&numbers, &[1]);
    m.run();
    *m.outputs().last().unwrap()
}

fn part2(numbers: &Vec<i128>) -> i128 {
    let mut m = intcode::Machine::with_input(&numbers, &[2]);
    m.run();
    *m.outputs().last().unwrap()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = aoc::parse_intcode(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    // use super::{part1, part2};

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&vec![0]), 0);
    // }
}
