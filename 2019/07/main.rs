use std::iter::*;

type Parsed = Vec<i128>;

fn part1(numbers: &Parsed) -> i128 {
    let phases = vec![0, 1, 2, 3, 4];
    let mut max_power = 0;
    for permutation in permute::permutations_of(&phases) {
        let mut val = 0;
        for perm in permutation {
            let mut m = intcode::Machine::with_input(numbers, &[*perm, val]);
            val = m.run_to_next_output().unwrap();
        }
        max_power = std::cmp::max(max_power, val);
    }
    max_power
}

fn part2(numbers: &Parsed) -> i128 {
    let phases = vec![5, 6, 7, 8, 9];
    let mut max_power = 0;
    for permutation in permute::permutations_of(&phases) {
        let mut machines: Vec<intcode::Machine> = permutation
            .map(|x| intcode::Machine::with_input(numbers, &[*x]))
            .collect();
        machines[0].add_input(0);
        let mut last_output = None;
        let power = loop {
            let mut out = None;
            for i in 0..machines.len() {
                if let Some(v) = machines[i].run_to_next_output() {
                    machines[(i + 1) % 5].add_input(v);
                    out = Some(v)
                }
            }
            if let Some(v) = out {
                last_output = Some(v);
            } else {
                break last_output.unwrap();
            }
        };
        max_power = std::cmp::max(max_power, power);
    }
    max_power
}

fn main() {
    aoc::run_main(intcode::parse_intcode, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::part2;

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ]),
            139629729
        );
    }
}
