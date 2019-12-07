use aoc;
use intcode;
use std::iter::*;

fn part1(numbers: &Vec<i64>) -> i64 {
    let phases = vec![0, 1, 2, 3, 4];
    let mut max_power = 0;
    for permutation in permute::lexicographically(&phases) {
        let mut m1 = intcode::Machine::new(&numbers, &vec![*permutation[0], 0]);
        let out1 = loop {
            m1.step();
            let outputs = m1.outputs();
            if let Some(v) = outputs.last() {
                break *v;
            }
        };
        let mut m2 = intcode::Machine::new(&numbers, &vec![*permutation[1], out1]);
        let out2 = loop {
            m2.step();
            let outputs = m2.outputs();
            if let Some(v) = outputs.last() {
                break *v;
            }
        };
        let mut m3 = intcode::Machine::new(&numbers, &vec![*permutation[2], out2]);
        let out3 = loop {
            m3.step();
            let outputs = m3.outputs();
            if let Some(v) = outputs.last() {
                break *v;
            }
        };
        let mut m4 = intcode::Machine::new(&numbers, &vec![*permutation[3], out3]);
        let out4 = loop {
            m4.step();
            let outputs = m4.outputs();
            if let Some(v) = outputs.last() {
                break *v;
            }
        };
        let mut m5 = intcode::Machine::new(&numbers, &vec![*permutation[4], out4]);
        let out5 = loop {
            m5.step();
            let outputs = m5.outputs();
            if let Some(v) = outputs.last() {
                break *v;
            }
        };
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
            let out1 = loop {
                let cont = m1.step();
                let outputs = m1.outputs();
                if let Some(v) = outputs.last() {
                    break (cont, Some(*v));
                }
                if !cont {
                    break (false, None);
                }
            };
            if let Some(v) = out1.1 {
                m2.add_inputs(&vec![v]);
            }
            let out2 = loop {
                let cont = m2.step();
                let outputs = m2.outputs();
                if let Some(v) = outputs.last() {
                    break (cont, Some(*v));
                }
                if !cont {
                    break (false, None);
                }
            };
            if let Some(v) = out2.1 {
                m3.add_inputs(&vec![v]);
            }
            let out3 = loop {
                let cont = m3.step();
                let outputs = m3.outputs();
                if let Some(v) = outputs.last() {
                    break (cont, Some(*v));
                }
                if !cont {
                    break (false, None);
                }
            };
            if let Some(v) = out3.1 {
                m4.add_inputs(&vec![v]);
            }
            let out4 = loop {
                let cont = m4.step();
                let outputs = m4.outputs();
                if let Some(v) = outputs.last() {
                    break (cont, Some(*v));
                }
                if !cont {
                    break (false, None);
                }
            };
            if let Some(v) = out4.1 {
                m5.add_inputs(&vec![v]);
            }
            let out5 = loop {
                let cont = m5.step();
                let outputs = m5.outputs();
                if let Some(v) = outputs.last() {
                    break (cont, Some(*v));
                }
                if !cont {
                    break (false, None);
                }
            };
            if let Some(v) = out5.1 {
                m1.add_inputs(&vec![v]);
                last_output = Some(v);
            }
            if !out5.0 {
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
