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
    aoc::run_main(intcode::parse_intcode, part1, part2);
}
