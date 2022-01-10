use std::{collections::HashMap, iter::*};

type ParsedItem = Vec<char>;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn score_line(line: &[char]) -> (bool, i64) {
    let valid = HashMap::from([
        (')', ('(', 3)),
        (']', ('[', 57)),
        ('}', ('{', 1197)),
        ('>', ('<', 25137)),
    ]);
    let mut stack = vec![];
    let mut err = None;
    for c in line {
        match c {
            '[' | '(' | '{' | '<' => stack.push(c),
            x => {
                let (v, s) = valid.get(x).unwrap();
                if v != stack.pop().unwrap() {
                    err = Some(s);
                    break;
                }
            }
        }
    }

    if let Some(v) = err {
        (false, *v)
    } else {
        let score = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
        let mut s = 0;
        for c in stack.iter().rev() {
            s = s * 5 + score.get(c).unwrap();
        }
        (true, s)
    }
}

fn part1(lines: &[ParsedItem]) -> Answer {
    lines
        .iter()
        .map(|line| score_line(line))
        .filter_map(|(valid, score)| if valid { None } else { Some(score) })
        .sum()
}

fn part2(lines: &[ParsedItem]) -> Answer {
    let mut scores: Vec<_> = lines
        .iter()
        .map(|line| score_line(line))
        .filter_map(|(valid, score)| if valid { Some(score) } else { None })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
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

    fn example() -> Vec<String> {
	vec![
        "[({(<(())[]>[[{[]{<()<>>".into(),
        "[(()[<>])]({[<{<<[]>>(".into(),
        "{([(<{}[<>[]}>{[]{[(<()>".into(),
        "(((({<>}<{<{<>}{[]{[]{}".into(),
        "[[<[([]))<([[{}[[()]]]".into(),
        "[{[{({}]{}}([{[{{{}}([]".into(),
        "{<[[]]>}<{[{[{[]{()[[[]".into(),
        "[<(<(<(<{}))><([]([]()".into(),
        "<{([([[(<>()){}]>(<<{{".into(),
        "<{([{{}}[<[[[<>{}]]]>[]]".into()
	]
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse(&example())),
            26397
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse(&example())),
            288957
        );
    }
}
