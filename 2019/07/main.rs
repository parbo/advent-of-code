use aoc;
use intcode;
use std::iter::*;

fn part1(numbers: &Vec<i64>) -> i64 {
    let phases = vec![0, 1, 2, 3, 4];
    let mut max_power = 0;
    for permutation in permute::lexicographically(&phases) {
        let mut m1 = intcode::Machine::new(&numbers, &vec![*permutation[0], 0]);
        let out1 = m1.run_to_next_output().unwrap();
        let mut m2 = intcode::Machine::new(&numbers, &vec![*permutation[1], out1]);
        let out2 = m2.run_to_next_output().unwrap();
        let mut m3 = intcode::Machine::new(&numbers, &vec![*permutation[2], out2]);
        let out3 = m3.run_to_next_output().unwrap();
        let mut m4 = intcode::Machine::new(&numbers, &vec![*permutation[3], out3]);
        let out4 = m4.run_to_next_output().unwrap();
        let mut m5 = intcode::Machine::new(&numbers, &vec![*permutation[4], out4]);
        let out5 = m5.run_to_next_output().unwrap();
        println!("phases {:?} produces {}", permutation, out5);
        max_power = std::cmp::max(max_power, out5);
    }
    max_power
}

fn part2(numbers: &Vec<i64>) -> i64 {
    let phases = vec![5, 6, 7, 8, 9];
    let mut max_power = 0;
    let mut max_phase = None;
    for permutation in permute::lexicographically(&phases) {
        let mut m1 = intcode::Machine::new(&numbers, &vec![*permutation[0], 0]);
        let mut m2 = intcode::Machine::new(&numbers, &vec![*permutation[1]]);
        let mut m3 = intcode::Machine::new(&numbers, &vec![*permutation[2]]);
        let mut m4 = intcode::Machine::new(&numbers, &vec![*permutation[3]]);
        let mut m5 = intcode::Machine::new(&numbers, &vec![*permutation[4]]);
        let mut last_output = None;
        let power = loop {
            if let Some(v) = m1.run_to_next_output() {
                m2.add_inputs(&vec![v]);
            }
            if let Some(v) = m2.run_to_next_output() {
                m3.add_inputs(&vec![v]);
            }
            if let Some(v) = m3.run_to_next_output() {
                m4.add_inputs(&vec![v]);
            }
            if let Some(v) = m4.run_to_next_output() {
                m5.add_inputs(&vec![v]);
            }
            if let Some(v) = m5.run_to_next_output() {
                m1.add_inputs(&vec![v]);
                last_output = Some(v);
            } else {
                break last_output.unwrap();
            }
        };
        if power > max_power {
            max_power = power;
            max_phase = Some(permutation.clone());
        }
    }
    println!("max power {} produced by {:?}", max_power, max_phase);
    max_power
}

fn parse(lines: &Vec<String>) -> Vec<i64> {
    let result: Vec<i64> = lines[0]
        .split(|c| c == ',')
        .map(|s| s.trim())
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    result
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
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
