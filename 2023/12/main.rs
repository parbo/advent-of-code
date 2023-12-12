use rayon::prelude::*;
use std::collections::HashSet;
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

fn count(s: &[char], c: char) -> (i64, i64) {
    let mut min = 0;
    let mut max = 0;
    let mut inc = 1;
    for x in s {
        if *x == c {
            min += inc;
            max += 1;
        } else if *x == '?' {
            max += 1;
            inc = 0;
        } else {
            break;
        }
    }
    (min, max)
}

fn count_ways2(springs: &[char], groups: &[i64]) -> i64 {
    println!("{:?} {:?}", springs, groups);
    let rem: i64 = springs.len() as i64
        // - springs.iter().filter(|x| **x == '#').count() as i64
        - groups.iter().sum::<i64>();
    // let rem = rem - (groups.len() - 1) as i64;
    let mut frontier = vec![(0, 0, '.', rem /*, vec![], vec![]*/)];
    frontier.reserve(springs.len());
    let mut num = 0;
    // let mut failed = HashSet::new();
    while let Some((pos, spos, curr, rem /*, result, grps*/)) = frontier.pop() {
        let rem_g = (groups.len() - pos) as i64;
        let rem_g = rem_g - 1;
        // dbg!(rem_g);
        // dbg!(rem);
        if rem < rem_g {
            continue;
        }
        let (min, max) = count(&springs[spos..], curr);
        let mut min = min;
        if pos > 0 && min == 0 {
            min = 1;
        }
        // println!(
        //     "{}, {}, {}, {:?}, {:?}, {}, {}, {:?}, {:?}",
        //     rem,
        //     pos,
        //     spos,
        //     result,
        //     &springs[spos..],
        //     curr,
        //     max,
        //     grps,
        //     groups,
        // );
        if pos == groups.len() {
            if springs[spos..].iter().all(|x| *x == '.' || *x == '?') {
                num += 1;
                if num % 1000000 == 0 {
                    println!("num: {}", num);
                }
            }
            continue;
        }
        let bef = frontier.len();
        if curr == '.' {
            let max = max.min(rem);
            for i in (min..=max).rev() {
                // println!("try {} .", i);
                // let mut r = result.clone();
                // assert!(r.is_empty() || *r.last().unwrap() == '#');
                // r.extend(std::iter::repeat('.').take(i as usize));
                let next = (
                    pos,
                    spos + i as usize,
                    '#',
                    rem - i, /*r, grps.clone()*/
                );
                // if !failed.contains(&next) {
                frontier.push(next);
                // }
            }
        } else if pos < groups.len() {
            let g = groups[pos];
            let gg = springs[spos..]
                .iter()
                .filter(|x| **x == '#' || **x == '?')
                .count() as i64;
            let ggg = groups[(pos + 1)..].iter().sum::<i64>();
            if g >= min && g <= max && ggg <= gg {
                // println!("pick {} #", g);
                // let mut r: Vec<char> = result.clone();
                // assert!(r.is_empty() || *r.last().unwrap() == '.');
                // r.extend(std::iter::repeat('#').take(g as usize));
                // let mut gg = grps.clone();
                // gg.push(g);
                // assert!(groups.starts_with(&gg));
                let next = (pos + 1, spos + g as usize, '.', rem /*, r, gg*/);
                // if !failed.contains(&next) {
                frontier.push(next);
                // }
            }
        }
        // if frontier.len() == bef {
        //     failed.insert((pos, spos, curr, rem));
        // }
    }
    num
}

fn part1(data: &Parsed) -> i64 {
    for x in data {
        let a = count_ways(&x.0, &x.1);
        let b = count_ways2(&x.0, &x.1);
        println!("{:?}, {}, {}", x, a, b);
        assert_eq!(a, b);
    }
    data.iter().map(|x| count_ways2(&x.0, &x.1)).sum()
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
        .enumerate()
        .map(|(ix, x)| {
            println!("{}/{}", ix, data.len());
            let (s, g) = unfold(&x.0, &x.1);
            count_ways2(&s, &g)
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
    fn test_count() {
        assert_eq!(count(&['#', '#', '#'], '.'), (0, 0));
        assert_eq!(count(&['.', '#', '#'], '.'), (1, 1));
        assert_eq!(count(&['.', '.', '#'], '.'), (2, 2));
        assert_eq!(count(&['.', '?', '#'], '.'), (1, 2));
        assert_eq!(count(&['?', '?', '#'], '.'), (0, 2));
        assert_eq!(count(&['?', '.', '#'], '.'), (0, 2));
    }

    #[test]
    fn test_count_ways_1() {
        let (s, g) = &parse(&["???.### 1,1,3".to_string()])[0];
        assert_eq!(count_ways2(s, g), 1);
    }
    #[test]
    fn test_count_ways_2() {
        let (s, g) = &parse(&[".??..??...?##. 1,1,3".to_string()])[0];
        assert_eq!(count_ways2(s, g), 4);
    }
    #[test]
    fn test_count_ways_3() {
        let (s, g) = &parse(&["?#?#?#?#?#?#?#? 1,3,1,6".to_string()])[0];
        assert_eq!(count_ways2(s, g), 1);
    }
    #[test]
    fn test_count_ways_4() {
        let (s, g) = &parse(&["????.#...#... 4,1,1".to_string()])[0];
        assert_eq!(count_ways2(s, g), 1);
    }
    #[test]
    fn test_count_ways_5() {
        let (s, g) = &parse(&["????.######..#####. 1,6,5".to_string()])[0];
        assert_eq!(count_ways2(s, g), 4);
    }
    #[test]
    fn test_count_ways_6() {
        let (s, g) = &parse(&["?###???????? 3,2,1".to_string()])[0];
        assert_eq!(count_ways2(s, g), 10);
    }

    #[test]
    fn test_count_ways_a() {
        let (s, g) = &parse(&["?.?????#???#? 1, 1, 2, 2".to_string()])[0];
        assert_eq!(count_ways2(s, g), 22);
    }

    #[test]
    fn test_count_ways_unfolded_1() {
        let (s, g) = &parse(&["???.### 1,1,3".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways2(&s, &g), 1);
    }

    #[test]
    fn test_count_ways_unfolded_2() {
        let (s, g) = &parse(&[".??..??...?##. 1,1,3".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways2(&s, &g), 16384);
    }

    #[test]
    fn test_count_ways_unfolded_3() {
        let (s, g) = &parse(&["?#?#?#?#?#?#?#? 1,3,1,6".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways2(&s, &g), 1);
    }

    #[test]
    fn test_count_ways_unfolded_4() {
        let (s, g) = &parse(&["????.#...#... 4,1,1".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways2(&s, &g), 16);
    }

    #[test]
    fn test_count_ways_unfolded_5() {
        let (s, g) = &parse(&["????.######..#####. 1,6,5".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways2(&s, &g), 2500);
    }

    #[test]
    fn test_count_ways_unfolded_6() {
        let (s, g) = &parse(&["?###???????? 3,2,1".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways2(&s, &g), 506250);
    }

    #[test]
    fn test_count_ways_unfolded_7() {
        let (s, g) = &parse(&["???..??????? 1,1,1,1".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways2(&s, &g), 506250);
    }
}
