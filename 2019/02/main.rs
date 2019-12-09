use aoc;
use intcode;

fn run_all(numbers: &Vec<i128>) -> Option<(i128, i128)> {
    for ai in 0..=99 {
        for bi in 0..=99 {
            let mut m = intcode::Machine::new(&numbers, &vec![0]);
            // Init
            *m.memory_mut().get_mut(1).unwrap() = ai;
            *m.memory_mut().get_mut(2).unwrap() = bi;
            let res = m.run();
            if res == Some(19690720) {
                return Some((ai, bi));
            }
        }
    }
    None
}

fn part1(numbers: &Vec<i128>) -> i128 {
    let mut m = intcode::Machine::new(&numbers, &vec![0]);
    // Init
    *m.memory_mut().get_mut(1).unwrap() = 12;
    *m.memory_mut().get_mut(2).unwrap() = 02;
    m.run().unwrap()
}

fn part2(numbers: &Vec<i128>) -> i128 {
    let (noun, verb) = run_all(numbers).unwrap();
    100 * noun + verb
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
