use std::iter::*;

// #[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
// #[display("{thing}: {al}-{ah} or {bl}-{bh}")]
// struct Rule {
//     thing: String,
//     al: i64,
//     ah: i64,
//     bl: i64,
//     bh: i64,
// }

type Parsed = Vec<Vec<char>>;
type Answer = i64;

fn part1(numbers: &Parsed) -> Answer {
    let bits = numbers[0].len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for bit in 0..bits {
        let mut count_1 = 0;
        let mut count_0 = 0;
        for number in numbers {
            let val = number[bit];
            match val {
                '0' => count_0 += 1,
                '1' => count_1 += 1,
                _ => panic!(),
            }
        }
        if count_1 > count_0 {
            gamma = gamma + (1 << (bits - bit - 1));
        } else {
            epsilon = epsilon + (1 << (bits - bit - 1));
        }
    }
    epsilon * gamma
}

fn find(nums: &Parsed, most: bool) -> Answer {
    let mut numbers = nums.clone();
    let bits = numbers[0].len();
    for bit in 0..bits {
        let mut count_1 = 0;
        let mut count_0 = 0;
        for number in &numbers {
            let val = number[bit];
            match val {
                '0' => count_0 += 1,
                '1' => count_1 += 1,
                _ => panic!(),
            }
        }
        if count_1 > count_0 {
	    if most {
		numbers = numbers.into_iter().filter(|x| x[bit] == '1').collect();
	    } else {
		numbers = numbers.into_iter().filter(|x| x[bit] == '0').collect();
	    }
        } else if count_0 > count_1 {
	    if most {
		numbers = numbers.into_iter().filter(|x| x[bit] == '0').collect();
	    } else {
		numbers = numbers.into_iter().filter(|x| x[bit] == '1').collect();
	    }
        } else {
            numbers = numbers
                .into_iter()
                .filter(|x| if most { x[bit] == '1' } else { x[bit] == '0' })
                .collect();
        }
        if numbers.len() == 1 {
            break;
        }
    }
    let mut ans = 0;
    for bit in 0..bits {
        let val = numbers[0][bit];
        if val == '1' {
            ans = ans + (1 << (bits - bit - 1));
        }
    }
    ans
}

fn part2(numbers: &Parsed) -> Answer {
    let o2r = find(&numbers, true);
    let co2 = find(&numbers, false);
    o2r * co2
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.chars().collect()).collect()
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
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&vec![
                "00100".chars().collect(),
                "11110".chars().collect(),
                "10110".chars().collect(),
                "10111".chars().collect(),
                "10101".chars().collect(),
                "01111".chars().collect(),
                "00111".chars().collect(),
                "11100".chars().collect(),
                "10000".chars().collect(),
                "11001".chars().collect(),
                "00010".chars().collect(),
                "01010".chars().collect(),
            ]),
            198
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&vec![
                "00100".chars().collect(),
                "11110".chars().collect(),
                "10110".chars().collect(),
                "10111".chars().collect(),
                "10101".chars().collect(),
                "01111".chars().collect(),
                "00111".chars().collect(),
                "11100".chars().collect(),
                "10000".chars().collect(),
                "11001".chars().collect(),
                "00010".chars().collect(),
                "01010".chars().collect(),
            ]),
            230
        );
    }
}
