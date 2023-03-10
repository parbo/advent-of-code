type Parsed = Vec<i128>;

fn run_all(numbers: &Parsed) -> Option<(i128, i128)> {
    for ai in 0..=99 {
        for bi in 0..=99 {
            let mut m = intcode::Machine::with_input(numbers, &[0]);
            // Init
            *m.memory_mut().get_mut(1).unwrap() = ai;
            *m.memory_mut().get_mut(2).unwrap() = bi;
            m.run();
            if m.memory().first() == Some(&19690720) {
                return Some((ai, bi));
            }
        }
    }
    None
}

fn part1(numbers: &Parsed) -> i128 {
    let mut m = intcode::Machine::with_input(numbers, &[0]);
    // Init
    *m.memory_mut().get_mut(1).unwrap() = 12;
    *m.memory_mut().get_mut(2).unwrap() = 2;
    m.run();
    *m.memory().first().unwrap()
}

fn part2(numbers: &Parsed) -> i128 {
    let (noun, verb) = run_all(numbers).unwrap();
    100 * noun + verb
}

fn main() {
    aoc::run_main(intcode::parse_intcode, part1, part2);
}
