use rayon::prelude::*;
use std::iter::*;

type ParsedItem = (Vec<char>, Vec<i64>);
type Parsed = Vec<ParsedItem>;

fn count_ways(springs: &[char], groups: &[i64]) -> i64 {
    // dbg!(springs);
    // dbg!(groups);
    let mut stack = vec![springs.to_vec()];
    stack.reserve(1000000);
    let mut num = 0;
    let mut grps = vec![];
    grps.reserve(groups.len());
    'outer: while let Some(s) = stack.pop() {
        grps.clear();
        // dbg!(&s);
        let mut last = None;
        for (i, c) in s.iter().enumerate() {
            if *c == '?' {
                for cc in ['#', '.'] {
                    let mut ss = s.clone();
                    ss[i] = cc;
                    // dbg!(&ss);
                    stack.push(ss);
                }
                // println!("break");
                continue 'outer;
            } else if *c == '#' {
                if last.unwrap_or('#') == '#' {
                    if grps.is_empty() {
                        grps.push(0);
                    }
                    *grps.last_mut().unwrap() += 1;
                } else {
                    grps.push(1);
                }
            } else if !groups.starts_with(&grps) {
                continue 'outer;
            }
            // dbg!(c);
            // dbg!(&grps);
            last = Some(*c);
        }
        if groups == grps {
            // println!("{:?} is valid", s);
            num += 1;
        }
    }
    num
}

fn part1(data: &Parsed) -> i64 {
    data.iter().map(|x| count_ways(&x.0, &x.1)).sum()
}

fn unfold(springs: &[char], groups: &[i64]) -> (Vec<char>, Vec<i64>) {
    let mut s = vec![];
    for i in 0..5 {
        s.push(springs.to_vec());
        if i != 4 {
            s.push(vec!['?']);
        }
    }
    let s = s.into_iter().flatten().collect::<Vec<_>>();
    let g = std::iter::repeat(groups.to_vec())
        .take(5)
        .flatten()
        .collect::<Vec<_>>();
    (s, g)
}

fn part2(data: &Parsed) -> i64 {
    data.par_iter()
        .map(|x| {
            let (s, g) = unfold(&x.0, &x.1);
            count_ways(&s, &g)
        })
        .sum()
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .par_iter()
        .map(|x| {
            let (springs, groups) = x.split_once(' ').unwrap();
            (springs.chars().collect(), aoc::things(groups))
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
    fn test_count_ways_1() {
        let (s, g) = &parse(&["???.### 1,1,3".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
    }
    #[test]
    fn test_count_ways_2() {
        let (s, g) = &parse(&[".??..??...?##. 1,1,3".to_string()])[0];
        assert_eq!(count_ways(s, g), 4);
    }
    #[test]
    fn test_count_ways_3() {
        let (s, g) = &parse(&["?#?#?#?#?#?#?#? 1,3,1,6".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
    }

    #[test]
    fn test_count_ways_unfolded_1() {
        let (s, g) = &parse(&["???.### 1,1,3".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways(&s, &g), 1);
    }

    #[test]
    fn test_count_ways_unfolded_2() {
        let (s, g) = &parse(&[".??..??...?##. 1,1,3".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways(&s, &g), 16384);
    }

    #[test]
    fn test_count_ways_unfolded_3() {
        let (s, g) = &parse(&["?#?#?#?#?#?#?#? 1,3,1,6".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways(&s, &g), 1);
    }

    #[test]
    fn test_count_ways_unfolded_6() {
        let (s, g) = &parse(&["?###???????? 3,2,1".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways(&s, &g), 506250);
    }

    //  - 1 arrangement
    // ????.#...#... 4,1,1 - 1 arrangement
    // ????.######..#####. 1,6,5 - 4 arrangements
    // ?###???????? 3,2,1 - 10 arrangements
}
