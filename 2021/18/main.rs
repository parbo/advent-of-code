use std::iter::*;
use std::time::Instant;

// #[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
// #[display("{thing}: {al}-{ah} or {bl}-{bh}")]
// struct Rule {
//     thing: String,
//     al: i64,
//     ah: i64,
//     bl: i64,
//     bh: i64,
// }

#[derive(Debug)]
enum SnailNumber {
    Num(i64),
    RecPair(Box<SnailNumber>, Box<SnailNumber>),
}

type ParsedItem = SnailNumber;
type Parsed = Vec<SnailNumber>;
type Answer = i64;

fn reduce(number: &SnailNumber) -> SnailNumber {
    SnailNumber::Num(0)
}

fn magnitude(number: &SnailNumber) -> i64 {
    0
}

fn part1(numbers: &[ParsedItem]) -> Answer {
    for n in numbers {
        println!("{:?}", n);
    }
    numbers
        .iter()
        .map(|x| reduce(x))
        .map(|x| magnitude(&x))
        .sum()
}

fn part2(_: &[ParsedItem]) -> Answer {
    0
}

fn parse_num(chars: &[char]) -> (i64, usize) {
    let mut pos = 0;
    while chars[pos] != ',' && chars[pos] != ']' {
        pos += 1;
    }
    (
        chars[0..pos]
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap(),
        pos + 1,
    )
}

fn parse_line(line: &[char]) -> (SnailNumber, usize) {
    if line[0] == '[' {
        let (left, left_len) = parse_line(&line[1..]);
        let (right, right_len) = parse_line(&line[(1 + left_len)..]);
        (
            SnailNumber::RecPair(Box::new(left), Box::new(right)),
            1 + left_len + right_len + 1,
        )
    } else if line[0] == ']' {
        panic!();
    } else {
        // Read the number
        let (num, len) = parse_num(line);
        (SnailNumber::Num(num), len)
    }
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| parse_line(&x.chars().collect::<Vec<char>>()).0)
        .collect()
}

fn main() {
    let start_time = Instant::now();
    let (part, lines) = aoc::read_lines();
    let io_time = Instant::now();
    let parsed = parse(&lines);
    let parse_time = Instant::now();
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    let done_time = Instant::now();
    println!(
        "read: {:?}, parse: {:?}, solve: {:?}\n",
        io_time.duration_since(start_time),
        parse_time.duration_since(io_time),
        done_time.duration_since(parse_time)
    );
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "[1,2]".into(),
            "[[1,2],3]".into(),
            "[9,[8,7]]".into(),
            "[[1,9],[8,5]]".into(),
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]".into(),
            "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]".into(),
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]".into(),
        ]
    }

    #[test]
    fn test_part1() {
	for line in example() {
	    let num = parse_line(&line.chars().collect::<Vec<char>>());
	    println!("{:?}", num);
	}
	let parsed = parse(&example());
        assert_eq!(part1(&parsed), 7);
    }
}
