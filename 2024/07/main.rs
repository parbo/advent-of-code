use std::iter::*;

type Parsed = Vec<(i64, Vec<i64>)>;

fn possible(operands: &[i64]) -> Vec<i64> {
    if operands.len() == 1 {
        vec![operands[0]]
    } else {
        let mut res = vec![];
        for i in 0..2 {
            let mut new_operands = vec![];
            if i == 0 {
                new_operands.push(operands[0] + operands[1]);
            } else {
                new_operands.push(operands[0] * operands[1]);
            }
            new_operands.extend(&operands[2..]);
            res.extend(possible(&new_operands));
        }
        res
    }
}

fn possible2(operands: &[i64]) -> Vec<i64> {
    if operands.len() == 1 {
        vec![operands[0]]
    } else {
        let mut res = vec![];
        for i in 0..3 {
            let mut new_operands = vec![];
            if i == 0 {
                new_operands.push(operands[0] + operands[1]);
            } else if i == 1 {
                new_operands.push(operands[0] * operands[1]);
            } else {
                new_operands.push(
                    (operands[0].to_string() + &operands[1].to_string())
                        .parse()
                        .unwrap(),
                );
            }
            new_operands.extend(&operands[2..]);
            res.extend(possible2(&new_operands));
        }
        res
    }
}

fn part1(data: &Parsed) -> i64 {
    let mut sum = 0;
    for (res, operands) in data {
        let p = possible(operands);
        if p.contains(res) {
            sum += res;
        }
    }
    sum
}

fn part2(data: &Parsed) -> i64 {
    let mut sum = 0;
    for (res, operands) in data {
        let p = possible2(operands);
        if p.contains(res) {
            sum += res;
        }
    }
    sum
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|line| {
            let parts = aoc::split_ch(line, ':');
            (parts[0].parse().unwrap(), aoc::things(parts[1]))
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "190: 10 19".into(),
            "3267: 81 40 27".into(),
            "83: 17 5".into(),
            "156: 15 6".into(),
            "7290: 6 8 6 15".into(),
            "161011: 16 10 13".into(),
            "192: 17 8 14".into(),
            "21037: 9 7 18 13".into(),
            "292: 11 6 16 20".into(),
        ]
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 11387);
    }
}
