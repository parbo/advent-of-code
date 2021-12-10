use std::{collections::HashMap, iter::*};

type ParsedItem = Vec<char>;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(lines: &[ParsedItem]) -> Answer {
    let mut score = HashMap::new();
    score.insert(')', 3);
    score.insert(']', 57);
    score.insert('}', 1197);
    score.insert('>', 25137);
    let mut valid = HashMap::new();
    valid.insert(')', '(');
    valid.insert(']', '[');
    valid.insert('}', '{');
    valid.insert('>', '<');
    let mut sum = 0;
    for line in lines {
        let mut stack = vec![];
        let mut err = None;
        for c in line {
            match c {
                '[' | '(' | '{' | '<' => stack.push(c),
                x => {
                    let p = stack.pop().unwrap();
                    if valid.get(x).unwrap() != p {
                        err = score.get(x);
                        break;
                    }
                }
            }
        }

        if let Some(v) = err {
            sum += v;
        }
    }
    sum
}

fn part2(lines: &[ParsedItem]) -> Answer {
    let mut score = HashMap::new();
    score.insert(')', 3);
    score.insert(']', 57);
    score.insert('}', 1197);
    score.insert('>', 25137);
    let mut cscore = HashMap::new();
    cscore.insert('(', 1);
    cscore.insert('[', 2);
    cscore.insert('{', 3);
    cscore.insert('<', 4);
    let mut valid = HashMap::new();
    valid.insert(')', '(');
    valid.insert(']', '[');
    valid.insert('}', '{');
    valid.insert('>', '<');
    let mut scores = vec![];
    for line in lines {
        let mut stack = vec![];
        let mut err = None;
        for c in line {
            match c {
                '[' | '(' | '{' | '<' => stack.push(c),
                x => {
                    let p = stack.pop().unwrap();
                    if valid.get(x).unwrap() != p {
                        err = score.get(x);
                        break;
                    }
                }
            }
        }

        if let Some(_v) = err {
            continue;
        }

        let mut s = 0;
        for c in stack.iter().rev() {
            s = s * 5;
            s += cscore.get(c).unwrap();
        }
        scores.push(s);
    }
    scores.sort();
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

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse(&vec![
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
            ])),
            26397
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse(&vec![
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
            ])),
            288957
        );
    }
}
