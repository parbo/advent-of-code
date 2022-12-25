use aoc::ParseError;
use regex::Regex;
use std::collections::HashMap;
use std::iter::*;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Rule {
    Char(char),
    Ref(Vec<usize>, Vec<usize>),
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<usize>() {
            Ok(Rule::Ref(vec![n], vec![]))
        } else if s.len() == 3 {
            s.chars()
                .nth(1)
                .map(|c| Rule::Char(c))
                .ok_or(aoc::ParseError::Generic)
        } else {
            let parts = aoc::split_ch(s, '|');
            let a = aoc::parse_to_vec_of(&aoc::split_ch(parts[0], ' '))?;
            let b = if parts.len() > 1 {
                aoc::parse_to_vec_of(&aoc::split_ch(parts[1], ' '))?
            } else {
                vec![]
            };
            Ok(Rule::Ref(a, b))
        }
    }
}

type Parsed = (HashMap<usize, Rule>, Vec<String>);
type Answer = usize;

fn expand_rule(rules: &HashMap<usize, Rule>, ix: usize) -> Vec<String> {
    match rules.get(&ix).unwrap() {
        Rule::Char(c) => vec![c.to_string()],
        Rule::Ref(a, b) => {
            let mut v = vec!["(".to_string()];
            for r in a {
                v.append(&mut expand_rule(rules, *r));
            }
            if !b.is_empty() {
                v.push("|".to_string());
                for r in b {
                    v.append(&mut expand_rule(rules, *r));
                }
            }
            v.push(")".to_string());
            v
        }
    }
}

fn part1(input: &Parsed) -> Answer {
    let (rules, strings) = input;
    // Expand the rules
    let expanded = expand_rule(rules, 0).join("");
    let re = Regex::new(&expanded).unwrap();
    let mut n = 0;
    for s in strings {
        if let Some(mat) = re.find(s) {
            if mat.start() == 0 && mat.end() == s.len() {
                n += 1;
            }
        }
    }
    n
}

fn find(re: &Regex, s: &str, mut ix: usize) -> (usize, usize) {
    let mut num = 0;
    while let Some(mat) = re.find_at(s, ix) {
        if mat.start() == ix {
            num += 1;
            ix = mat.end();
        } else {
            break;
        }
    }
    (num, ix)
}

fn part2(input: &Parsed) -> Answer {
    // 0: 8 11
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    // so (42 42 42) (42 31) or (42 42 42) (42 42 31 31)
    // Which means (42){2,}(31)+
    // But there must be more 42 than 31
    let (rules, strings) = input;
    let r42 = Regex::new(&expand_rule(rules, 42).join("")).unwrap();
    let r31 = Regex::new(&expand_rule(rules, 31).join("")).unwrap();
    let mut n = 0;
    for s in strings {
        let (num42, ix) = find(&r42, s, 0);
        let (num31, ix) = find(&r31, s, ix);
        if num42 > num31 && num31 > 0 && ix == s.len() {
            n += 1;
        }
    }
    n
}

fn parse(lines: &[String]) -> Parsed {
    let sections = aoc::split_by_empty_line(lines);
    let r: HashMap<_, _> = sections[0]
        .iter()
        .map(|line| {
            let parts = aoc::split_ch(line, ':');
            (
                parts[0].parse::<usize>().unwrap(),
                parts[1].parse::<Rule>().unwrap(),
            )
        })
        .collect();
    (r, sections[1].iter().map(|s| s.to_string()).collect())
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
    fn test_part2() {
        let example_str = include_str!("p2example.txt");
        let example: Vec<_> = example_str
            .split('\n')
            .map(|x| x.trim_end_matches('\n').trim_end_matches('\r').to_string())
            .collect();
        let parsed = parse(&example);
        assert_eq!(part2(&parsed), 12);
    }
}
