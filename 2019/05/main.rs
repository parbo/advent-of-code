use aoc;

extern crate intcode;

fn part1(numbers: &Vec<i128>) -> i128 {
    let mut m = intcode::Machine::new(&numbers, &vec![1]);
    m.run().unwrap();
    *m.outputs().last().unwrap()
}

fn part2(numbers: &Vec<i128>) -> i128 {
    let mut m = intcode::Machine::new(&numbers, &vec![5]);
    m.run().unwrap();
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
