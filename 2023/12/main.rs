use rayon::prelude::*;
use std::iter::*;

type ParsedItem = (Vec<char>, Vec<i64>);
type Parsed = Vec<ParsedItem>;

fn do_count_ways(
    s: &[char],
    sa: usize,
    curr: char,
    g: &[i64],
    ga: usize,
    was_hash: bool,
    acc: Vec<char>,
    cache: &mut aoc::FxHashMap<(usize, usize, char, bool), i64>,
) -> i64 {
    let k = (sa, ga, curr, was_hash);
    dbg!(k, &acc);
    if let Some(v) = cache.get(&k) {
        println!("v: {}", *v);
        return *v;
    } else if g.is_empty() {
        let v = if (curr == '.' || curr == '?')
            && s[(sa + 1)..].iter().all(|x| *x == '.' || *x == '?')
        {
            println!("ok");
            1
        } else {
            0
        };
        cache.insert(k, v);
        return v;
    } else {
        let mut num_g = 0;
        for i in sa..s.len() {
            let c = if i == sa { curr } else { s[i] };
            dbg!(c, i);
            if c == '?' {
                let mut num = 0;
                for cc in ['#', '.'] {
                    if cc == '#' && was_hash && i == sa {
                        continue;
                    }
                    dbg!(cc);
                    let mut acc = acc.clone();
                    acc[i] = cc;
                    let wh = if i == sa { was_hash } else { s[i - 1] == '#' };
                    num += do_count_ways(s, sa + i, cc, g, ga, wh, acc, cache);
                }
                dbg!(num);
                cache.insert(k, num);
                return num;
            } else if c == '#' {
                assert!(!was_hash || i > sa);
                num_g += 1;
            } else if let Some(&n) = g.first() {
                if num_g == n {
                    let mut acc = acc.clone();
                    return do_count_ways(s, sa + i, s[sa + i], g, ga + 1, true, acc, cache);
                } else if num_g > 0 {
                    cache.insert(k, 0);
                    println!("fail");
                    return 0;
                }
            } else {
                let mut acc = acc.clone();
                return do_count_ways(s, sa + i, s[sa + i], g, ga, false, acc, cache);
            }
        }
        if g.len() - ga == 1 && num_g == g[ga] {
            cache.insert(k, 1);
            println!("ok 2");
            return 1;
        }
    }
    println!("fail 2");
    cache.insert(k, 0);
    0
}

fn count_ways(springs: &[char], groups: &[i64]) -> i64 {
    let mut cache = aoc::FxHashMap::default();
    do_count_ways(
        springs,
        0,
        springs[0],
        groups,
        0,
        false,
        springs.to_vec(),
        &mut cache,
    )
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
    data.iter()
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

    // #[test]
    // fn test_count_ways_1() {
    //     let (s, g) = &parse(&["???.### 1,1,3".to_string()])[0];
    //     assert_eq!(count_ways(s, g), 1);
    // }
    // #[test]
    // fn test_count_ways_2() {
    //     let (s, g) = &parse(&[".??..??...?##. 1,1,3".to_string()])[0];
    //     assert_eq!(count_ways(s, g), 4);
    // }
    // #[test]
    // fn test_count_ways_3() {
    //     let (s, g) = &parse(&["?#?#?#?#?#?#?#? 1,3,1,6".to_string()])[0];
    //     assert_eq!(count_ways(s, g), 1);
    // }
    // #[test]
    // fn test_count_ways_4() {
    //     let (s, g) = &parse(&["????.#...#... 4,1,1".to_string()])[0];
    //     assert_eq!(count_ways(s, g), 1);
    // }
    // #[test]
    // fn test_count_ways_5() {
    //     let (s, g) = &parse(&["????.######..#####. 1,6,5".to_string()])[0];
    //     assert_eq!(count_ways(s, g), 4);
    // }
    // #[test]
    // fn test_count_ways_6() {
    //     let (s, g) = &parse(&["?###???????? 3,2,1".to_string()])[0];
    //     assert_eq!(count_ways(s, g), 10);
    // }

    // #[test]
    // fn test_count_ways_a() {
    //     let (s, g) = &parse(&["?.?????#???#? 1, 1, 2, 2".to_string()])[0];
    //     assert_eq!(count_ways(s, g), 22);
    // }

    // #[test]
    // fn test_count_ways_unfolded_1() {
    //     let (s, g) = &parse(&["???.### 1,1,3".to_string()])[0];
    //     let (s, g) = unfold(s, g);
    //     assert_eq!(count_ways(&s, &g), 1);
    // }

    // #[test]
    // fn test_count_ways_unfolded_2() {
    //     let (s, g) = &parse(&[".??..??...?##. 1,1,3".to_string()])[0];
    //     let (s, g) = unfold(s, g);
    //     assert_eq!(count_ways(&s, &g), 16384);
    // }

    // #[test]
    // fn test_count_ways_unfolded_3() {
    //     let (s, g) = &parse(&["?#?#?#?#?#?#?#? 1,3,1,6".to_string()])[0];
    //     let (s, g) = unfold(s, g);
    //     assert_eq!(count_ways(&s, &g), 1);
    // }

    // #[test]
    // fn test_count_ways_unfolded_4() {
    //     let (s, g) = &parse(&["????.#...#... 4,1,1".to_string()])[0];
    //     let (s, g) = unfold(s, g);
    //     assert_eq!(count_ways(&s, &g), 16);
    // }

    // #[test]
    // fn test_count_ways_unfolded_5() {
    //     let (s, g) = &parse(&["????.######..#####. 1,6,5".to_string()])[0];
    //     let (s, g) = unfold(s, g);
    //     assert_eq!(count_ways(&s, &g), 2500);
    // }

    // #[test]
    // fn test_count_ways_unfolded_6() {
    //     let (s, g) = &parse(&["?###???????? 3,2,1".to_string()])[0];
    //     let (s, g) = unfold(s, g);
    //     assert_eq!(count_ways(&s, &g), 506250);
    // }

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
