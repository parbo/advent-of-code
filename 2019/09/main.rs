type Parsed = Vec<i128>;

fn part1(numbers: &Parsed) -> i128 {
    let mut m = intcode::Machine::with_input(numbers, &[1]);
    m.run();
    *m.outputs().last().unwrap()
}

fn part2(numbers: &Parsed) -> i128 {
    let mut m = intcode::Machine::with_input(numbers, &[2]);
    m.run();
    *m.outputs().last().unwrap()
}

fn main() {
    aoc::run_main(intcode::parse_intcode, part1, part2);
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
