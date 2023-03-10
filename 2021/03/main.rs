use std::iter::*;

type ParsedItem = Vec<char>;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(numbers: &Parsed) -> Answer {
    let bits = numbers[0].len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for bit in 0..bits {
        let count_1 = numbers.iter().filter(|x| x[bit] == '1').count();
        let count_0 = numbers.iter().filter(|x| x[bit] == '0').count();
        if count_1 > count_0 {
            gamma += 1 << (bits - bit - 1);
        } else {
            epsilon += 1 << (bits - bit - 1);
        }
    }
    epsilon * gamma
}

fn find(nums: &[ParsedItem], most: bool) -> Answer {
    let mut numbers = nums.to_owned();
    let bits = numbers[0].len();
    for bit in 0..bits {
        let count_1 = numbers.iter().filter(|x| x[bit] == '1').count();
        let count_0 = numbers.iter().filter(|x| x[bit] == '0').count();
        let cond = if count_0 > count_1 { most } else { !most };
        numbers.retain(|x| if cond { x[bit] == '0' } else { x[bit] == '1' });
        if numbers.len() == 1 {
            break;
        }
    }
    numbers[0]
        .iter()
        .enumerate()
        .map(|(bit, x)| if *x == '1' { 1 << (bits - bit - 1) } else { 0 })
        .sum()
}

fn part2(numbers: &Parsed) -> Answer {
    let o2r = find(numbers, true);
    let co2 = find(numbers, false);
    o2r * co2
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
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
