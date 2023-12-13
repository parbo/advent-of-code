use aoc::Itertools;
use rayon::prelude::*;
use std::iter::*;

type ParsedItem = (Vec<char>, Vec<i64>);
type Parsed = Vec<ParsedItem>;

fn do_count_ways(
    s: &[char],
    sa: usize,
    curr: Option<char>,
    g: &[i64],
    ga: usize,
    num_g: i64,
    cache: &mut aoc::FxHashMap<(usize, usize, Option<char>, i64), i64>,
) -> i64 {
    let k = (sa, ga, curr, num_g);
    if let Some(v) = cache.get(&k) {
        *v
    } else if sa == s.len() {
        if (ga == g.len() && num_g == 0) || (g.len() - ga == 1 && num_g == g[ga]) {
            cache.insert(k, 1);
            1
        } else {
            cache.insert(k, 0);
            0
        }
    } else {
        let c = curr.unwrap_or(s[sa]);
        match c {
            '?' => {
                let mut num = 0;
                for cc in ['#', '.'] {
                    num += do_count_ways(s, sa, Some(cc), g, ga, num_g, cache);
                }
                cache.insert(k, num);
                num
            }
            '#' => do_count_ways(s, sa + 1, None, g, ga, num_g + 1, cache),
            '.' => {
                let n = g.get(ga);
                if Some(&num_g) == n {
                    do_count_ways(s, sa + 1, None, g, ga + 1, 0, cache)
                } else if num_g == 0 {
                    do_count_ways(s, sa + 1, None, g, ga, 0, cache)
                } else {
                    cache.insert(k, 0);
                    0
                }
            }
            _ => panic!(),
        }
    }
}

fn count_ways(springs: &[char], groups: &[i64]) -> i64 {
    let mut cache = aoc::FxHashMap::default();
    do_count_ways(springs, 0, None, groups, 0, 0, &mut cache)
}

fn part1(data: &Parsed) -> i64 {
    data.par_iter().map(|x| count_ways(&x.0, &x.1)).sum()
}

fn unfold(springs: &[char], groups: &[i64]) -> (Vec<char>, Vec<i64>) {
    let s = (0..5)
        .map(|_| springs.iter().collect::<String>())
        .join("?")
        .chars()
        .collect();
    let g = std::iter::repeat(groups.to_vec())
        .take(5)
        .flatten()
        .collect::<Vec<_>>();
    (s, g)
}

fn part2(data: &Parsed) -> i64 {
    data.par_iter()
        .map(move |(s, g)| {
            let (s, g) = unfold(s, g);
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
    fn test_count_ways_dp1() {
        let (s, g) = &parse(&[". 1".to_string()])[0];
        assert_eq!(count_ways(s, g), 0);
    }
    #[test]
    fn test_count_ways_dp2() {
        let (s, g) = &parse(&["# 1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
    }
    #[test]
    fn test_count_ways_dp3() {
        let (s, g) = &parse(&["? 1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
    }
    #[test]
    fn test_count_ways_dp4() {
        let (s, g) = &parse(&["?? 1".to_string()])[0];
        assert_eq!(count_ways(s, g), 2);
    }
    #[test]
    fn test_count_ways_dp5() {
        let (s, g) = &parse(&["??? 1,1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
    }
    #[test]
    fn test_count_ways_dp6() {
        let (s, g) = &parse(&["?.? 1,1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
    }
    #[test]
    fn test_count_ways_dp7() {
        let (s, g) = &parse(&["?.# 1,1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
    }
    #[test]
    fn test_count_ways_dp8() {
        let (s, g) = &parse(&["?.# 1,1".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
    }
    #[test]
    fn test_count_ways_dp9() {
        let (s, g) = &parse(&["??.## 2".to_string()])[0];
        assert_eq!(count_ways(s, g), 1);
    }
    #[test]
    fn test_count_ways_dp10() {
        let (s, g) = &parse(&["??.?? 2".to_string()])[0];
        assert_eq!(count_ways(s, g), 2);
    }
}
