use std::{collections::HashMap, iter::*};

type ParsedItem = (Vec<String>, Vec<String>);
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(patterns: &Parsed) -> Answer {
    patterns
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .filter(|p| [2, 3, 4, 7].contains(&p.len()))
                .count()
        })
        .sum::<usize>() as Answer
}

fn part2(patterns: &Parsed) -> Answer {
    let mut sum = 0;
    for (pattern, output) in patterns {
        let mut mappings = HashMap::new();
        let mut ptn: Vec<Vec<char>> = pattern.iter().map(|p| p.chars().collect()).collect();
        ptn.sort_by_key(|a| a.len());
        for i in 0..ptn.len() {
            let p = &ptn[i];
            for c in p {
                if mappings.contains_key(c) {
                    continue;
                }
                let cn = (0..ptn.len()).filter(|x| ptn[*x].contains(c)).count();
                match p.len() {
                    2 => {
                        if cn == 8 {
                            mappings.insert(*c, 'c');
                        } else if cn == 9 {
                            mappings.insert(*c, 'f');
                        } else {
                            panic!();
                        }
                    }
                    3 => {
                        if cn == 8 {
                            mappings.insert(*c, 'a');
                        } else {
                            panic!();
                        }
                    }
                    4 => {
                        if cn == 6 {
                            mappings.insert(*c, 'b');
                        } else if cn == 7 {
                            mappings.insert(*c, 'd');
                        } else {
                            panic!();
                        }
                    }
                    7 => {
                        if cn == 4 {
                            mappings.insert(*c, 'e');
                        } else if cn == 7 {
                            mappings.insert(*c, 'g');
                        } else {
                            panic!();
                        }
                    }
                    _ => (),
                }
            }
        }
        let mut val = 0;
        for o in output {
            val *= 10;
            let mut segments: Vec<char> = o.chars().map(|c| *mappings.get(&c).unwrap()).collect();
            segments.sort_unstable();
            let s: String = segments.into_iter().collect();
            match s.as_str() {
                "abcefg" => val += 0,
                "cf" => val += 1,
                "acdeg" => val += 2,
                "acdfg" => val += 3,
                "bcdf" => val += 4,
                "abdfg" => val += 5,
                "abdefg" => val += 6,
                "acf" => val += 7,
                "abcdefg" => val += 8,
                "abcdfg" => val += 9,
                _ => panic!(),
            }
        }
        sum += val;
    }
    sum
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| aoc::split_ch(x, '|'))
        .map(|x| {
            (
                aoc::split_w(x[0]).into_iter().map(String::from).collect(),
                aoc::split_w(x[1]).into_iter().map(String::from).collect(),
            )
        })
        .collect()
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
            part1(&parse(&vec![
		"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".into(),
		"edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".into(),
		"fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".into(),
		"fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".into(),
		"aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".into(),
		"fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".into(),
		"dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".into(),
		"bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".into(),
		"egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".into(),
		"gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".into(),
	    ])),
            26
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse(&vec![
		"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".into(),
		"edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".into(),
		"fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".into(),
		"fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".into(),
		"aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".into(),
		"fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".into(),
		"dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".into(),
		"bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".into(),
		"egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".into(),
		"gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".into(),
	    ])),
            61229
        );
    }
}
