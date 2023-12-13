use rayon::prelude::*;
use std::iter::*;

type ParsedItem = (Vec<char>, Vec<i64>);
type Parsed = Vec<ParsedItem>;

fn do_count_ways(
    s: &[char],
    g: &[i64],
    was_hash: bool,
    cache: &mut aoc::FxHashMap<(Vec<char>, Vec<i64>, bool), i64>,
) -> i64 {
    let k = (s.to_vec(), g.to_vec(), was_hash);
    if let Some(v) = cache.get(&k) {
        return *v;
    } else if g.is_empty() {
        let v = if s.iter().all(|x| *x == '.' || *x == '?') {
            1
        } else {
            0
        };
        // dbg!(v);
        cache.insert(k, v);
        return v;
    } else {
        // dbg!(s, g, was_hash);
        let mut num_g = 0;
        // dbg!(s, g);
        for (i, c) in s.iter().enumerate() {
            // dbg!(i, c);
            if *c == '?' {
                let mut num = 0;
                for cc in ['#', '.'] {
                    if cc == '#' && was_hash && i == 0 {
                        continue;
                    }
                    let mut ss = s.to_vec();
                    ss[i] = cc;
                    // dbg!(cc, &ss);
                    num += do_count_ways(&ss, &g, was_hash, cache);
                }
                cache.insert(k, num);
                return num;
            } else if *c == '#' {
                assert!(!was_hash || i > 0);
                num_g += 1;
            } else {
                if let Some(&n) = g.first() {
                    if num_g == n {
                        // dbg!(num_g, &s[i..], &g[1..]);
                        return do_count_ways(&s[i..], &g[1..], true, cache);
                    } else if num_g > 0 {
                        // dbg!(s, g);
                        // dbg!(num_g, i, c);
                        cache.insert(k, 0);
                        // println!("invalid 2");
                        return 0;
                    }
                } else {
                    return do_count_ways(&s[i..], &g, false, cache);
                }
            }
        }
        if g.len() == 1 && num_g == *g.last().unwrap() {
            // println!("Ok");
            cache.insert(k, 1);
            return 1;
        }
    }
    cache.insert(k, 0);
    return 0;
}

fn count_ways(springs: &[char], groups: &[i64]) -> i64 {
    let mut cache = aoc::FxHashMap::default();
    do_count_ways(springs, groups, false, &mut cache)
}

fn part1(data: &Parsed) -> i64 {
    data.par_iter().map(|x| count_ways(&x.0, &x.1)).sum()
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
        .map(move |(s, g)| {
            let (s, g) = unfold(&s, &g);
            count_ways(&s, &g)
        })
        .sum()
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
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
    fn test_count_ways_4() {
        let (s, g) = &parse(&["????.#...#... 4,1,1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
    }
    #[test]
    fn test_count_ways_5() {
        let (s, g) = &parse(&["????.######..#####. 1,6,5".to_string()])[0];
        assert_eq!(count_ways(s, g), 4);
    }
    #[test]
    fn test_count_ways_6() {
        let (s, g) = &parse(&["?###???????? 3,2,1".to_string()])[0];
        assert_eq!(count_ways(s, g), 10);
    }

    #[test]
    fn test_count_ways_a() {
        let (s, g) = &parse(&["?.?????#???#? 1, 1, 2, 2".to_string()])[0];
        assert_eq!(count_ways(s, g), 22);
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
    fn test_count_ways_unfolded_4() {
        let (s, g) = &parse(&["????.#...#... 4,1,1".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways(&s, &g), 16);
    }

    #[test]
    fn test_count_ways_unfolded_5() {
        let (s, g) = &parse(&["????.######..#####. 1,6,5".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways(&s, &g), 2500);
    }

    #[test]
    fn test_count_ways_unfolded_6() {
        let (s, g) = &parse(&["?###???????? 3,2,1".to_string()])[0];
        let (s, g) = unfold(s, g);
        assert_eq!(count_ways(&s, &g), 506250);
    }

    #[test]
    fn test_count_ways_dp() {
        let (s, g) = &parse(&[". 1".to_string()])[0];
        assert_eq!(count_ways(s, g), 0);
        let (s, g) = &parse(&["# 1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
        let (s, g) = &parse(&["? 1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
        let (s, g) = &parse(&["?? 1".to_string()])[0];
        assert_eq!(count_ways(s, g), 2);
        let (s, g) = &parse(&["??? 1,1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
        let (s, g) = &parse(&["??? 1,1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
        let (s, g) = &parse(&["?.? 1,1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
        let (s, g) = &parse(&["?.# 1,1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
        let (s, g) = &parse(&["?.# 1,1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
        let (s, g) = &parse(&["??.## 2".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
        let (s, g) = &parse(&["??.?? 2".to_string()])[0];
        assert_eq!(count_ways(s, g), 2);
    }
}
