use aoc;

extern crate intcode;

type Parsed = Vec<i128>;

fn part1(numbers: &Parsed) -> i128 {
    let mut m = intcode::Machine::with_input(numbers, &[1]);
    m.run();
    *m.outputs().last().unwrap()
}

fn part2(numbers: &Parsed) -> i128 {
    let mut m = intcode::Machine::with_input(numbers, &[5]);
    m.run();
    *m.outputs().last().unwrap()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = intcode::parse_intcode(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}
