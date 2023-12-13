use rayon::prelude::*;
use std::collections::VecDeque;
use std::iter::*;

type ParsedItem = (Vec<char>, Vec<i64>);
type Parsed = Vec<ParsedItem>;

fn count_ways(springs: &[char], groups: &[i64]) -> i64 {
    // dbg!(springs);
    // dbg!(groups);
    let mut stack = vec![springs.to_vec()];
    stack.reserve(1000000);
    let mut num = 0;
    let mut grps = Vec::with_capacity(groups.len());
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
    (
        s.iter().take_while(|x| **x == c && **x != '?').count() as i64,
        s.iter().take_while(|x| **x == c || **x == '?').count() as i64,
    )
}

fn count_ways2(springs: &[char], groups: &[i64]) -> i64 {
    let rem: i64 = springs.len() as i64
        // - springs.iter().filter(|x| **x == '#').count() as i64
        - groups.iter().sum::<i64>();
    // let rem = rem - (groups.len() - 1) as i64;
    let mut frontier = vec![(0, 0, '.', rem /*, vec![], vec![]*/)];
    frontier.reserve(100000);
    let mut num = 0;
    let mut cnt_memo = aoc::FxHashMap::default();
    let mut gg_memo = aoc::FxHashMap::default();
    let mut end_memo = aoc::FxHashMap::default();
    while let Some((pos, spos, curr, rem /*, result, grps*/)) = frontier.pop() {
        let rem_g = (groups.len() - pos) as i64;
        let rem_g = rem_g - 1;
        // dbg!(rem_g);
        // dbg!(rem);
        if rem < rem_g {
            continue;
        }
        let (min, max) = if let Some(v) = cnt_memo.get(&(spos, curr)) {
            *v
        } else {
            let v = count(&springs[spos..], curr);
            cnt_memo.insert((spos, curr), v);
            v
        };
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
            let end = if let Some(v) = end_memo.get(&spos) {
                *v
            } else {
                let v = springs[spos..].iter().all(|x| *x == '.' || *x == '?');
                end_memo.insert(spos, v);
                v
            };
            if end {
                num += 1;
                // if num % 1000000 == 0 {
                //     println!("num: {}", num);
                // }
            }
            continue;
        }
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
                frontier.push(next);
            }
        } else if pos < groups.len() {
            let g = groups[pos];
            let gg = if let Some(v) = gg_memo.get(&spos) {
                *v
            } else {
                let v = springs[spos..]
                    .iter()
                    .filter(|x| **x == '#' || **x == '?')
                    .count() as i64;
                gg_memo.insert(spos, v);
                v
            };
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
                frontier.push(next);
            }
        }
    }
    num
}

fn count_ways3(springs: &[char], groups: &[i64]) -> i64 {
    // Divide and conquer
    let mut ranges = VecDeque::new();
    let mut start = None;
    for (i, c) in springs.iter().enumerate() {
        if start.is_none() {
            if *c == '#' || *c == '?' {
                start = Some(i);
            }
        } else if *c == '.' {
            ranges.push_back((start.unwrap(), i));
            start = None;
        }
    }
    if let Some(s) = start {
        ranges.push_back((s, springs.len()));
    }
    println!("ranges: {:?}", ranges);
    let mut todo = vec![(1, 0, 0)];
    todo.reserve(100000);
    let mut s = 0;
    let mut memo = aoc::FxHashMap::default(); //lru::LruCache::new(std::num::NonZeroUsize::new(100000).unwrap());
    let mut hits = 0;
    let mut misses = 0;
    while let Some((p, r, pp)) = todo.pop() {
        if let Some(&(start, end)) = ranges.get(r) {
            let len = (end - start) as i64;
            // let num_hash = (len + 1) / 2;
            // dbg!((len, num_hash, start, end, &ranges));
            for mut l in 0..=len {
                let mut pos = pp;
                while l > 0 && pos < groups.len() {
                    let g = groups[pos];
                    if g <= l {
                        pos += 1;
                        l -= g;
                    } else {
                        break;
                    }
                }
                if l == 0 {
                    // dbg!((start, end, pp, pos));
                    let v = if let Some(v) = memo.get(&(start, end, pp, pos)) {
                        hits += 1;
                        *v
                    } else {
                        let v = count_ways2(&springs[start..end], &groups[pp..pos]);
                        memo.insert((start, end, pp, pos), v);
                        misses += 1;
                        v
                    };
                    // dbg!(v);
                    let mut new_p = p;
                    new_p *= v;
                    // dbg!(new_p);
                    todo.push((new_p, r + 1, pos));
                }
            }
            // println!("1");
        } else if pp == groups.len() {
            // dbg!(&divs);
            s += p;
        }
        // println!("2");
    }
    dbg!(hits);
    dbg!(misses);
    s
}

fn do_count_ways4(
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
                    num += do_count_ways4(&ss, &g, was_hash, cache);
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
                        return do_count_ways4(&s[i..], &g[1..], true, cache);
                    } else if num_g > 0 {
                        // dbg!(s, g);
                        // dbg!(num_g, i, c);
                        cache.insert(k, 0);
                        // println!("invalid 2");
                        return 0;
                    }
                } else {
                    return do_count_ways4(&s[i..], &g, false, cache);
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

fn count_ways4(springs: &[char], groups: &[i64]) -> i64 {
    let mut cache = aoc::FxHashMap::default();
    do_count_ways4(springs, groups, false, &mut cache)
}

fn part1(data: &Parsed) -> i64 {
    for x in data {
        let a = count_ways(&x.0, &x.1);
        let b = count_ways2(&x.0, &x.1);
        let c = count_ways3(&x.0, &x.1);
        let d = count_ways4(&x.0, &x.1);
        println!("{:?}, {}, {}, {}, {}", x, a, b, c, d);
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(a, d);
    }
    data.par_iter().map(|x| count_ways2(&x.0, &x.1)).sum()
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
        .map(move |(ix, x)| {
            println!("{}/{}", ix, data.len());
            let (s, g) = unfold(&x.0, &x.1);
            println!("{:?} {:?}", s, g);
            let x = count_ways4(&s, &g);
            println!("{}", x);
            x
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
    // fn test_count() {
    //     assert_eq!(count(&['#', '#', '#'], '.'), (0, 0));
    //     assert_eq!(count(&['.', '#', '#'], '.'), (1, 1));
    //     assert_eq!(count(&['.', '.', '#'], '.'), (2, 2));
    //     assert_eq!(count(&['.', '?', '#'], '.'), (1, 2));
    //     assert_eq!(count(&['?', '?', '#'], '.'), (0, 2));
    //     assert_eq!(count(&['?', '.', '#'], '.'), (0, 2));
    // }

    // #[test]
    // fn test_count_ways_1() {
    //     let (s, g) = &parse(&["???.### 1,1,3".to_string()])[0];
    //     assert_eq!(count_ways2(s, g), 1);
    // }
    // #[test]
    // fn test_count_ways_2() {
    //     let (s, g) = &parse(&[".??..??...?##. 1,1,3".to_string()])[0];
    //     assert_eq!(count_ways2(s, g), 4);
    // }
    // #[test]
    // fn test_count_ways_3() {
    //     let (s, g) = &parse(&["?#?#?#?#?#?#?#? 1,3,1,6".to_string()])[0];
    //     assert_eq!(count_ways2(s, g), 1);
    // }
    // #[test]
    // fn test_count_ways_4() {
    //     let (s, g) = &parse(&["????.#...#... 4,1,1".to_string()])[0];
    //     assert_eq!(count_ways2(s, g), 1);
    // }
    // #[test]
    // fn test_count_ways_5() {
    //     let (s, g) = &parse(&["????.######..#####. 1,6,5".to_string()])[0];
    //     assert_eq!(count_ways2(s, g), 4);
    // }
    // #[test]
    // fn test_count_ways_6() {
    //     let (s, g) = &parse(&["?###???????? 3,2,1".to_string()])[0];
    //     assert_eq!(count_ways2(s, g), 10);
    // }

    // #[test]
    // fn test_count_ways_a() {
    //     let (s, g) = &parse(&["?.?????#???#? 1, 1, 2, 2".to_string()])[0];
    //     assert_eq!(count_ways2(s, g), 22);
    // }

    // #[test]
    // fn test_count_ways_unfolded_1() {
    //     let (s, g) = &parse(&["???.### 1,1,3".to_string()])[0];
    //     let (s, g) = unfold(s, g);
    //     assert_eq!(count_ways2(&s, &g), 1);
    // }

    // #[test]
    // fn test_count_ways_unfolded_2() {
    //     let (s, g) = &parse(&[".??..??...?##. 1,1,3".to_string()])[0];
    //     let (s, g) = unfold(s, g);
    //     assert_eq!(count_ways2(&s, &g), 16384);
    // }

    // #[test]
    // fn test_count_ways_unfolded_3() {
    //     let (s, g) = &parse(&["?#?#?#?#?#?#?#? 1,3,1,6".to_string()])[0];
    //     let (s, g) = unfold(s, g);
    //     assert_eq!(count_ways2(&s, &g), 1);
    // }

    // #[test]
    // fn test_count_ways_unfolded_4() {
    //     let (s, g) = &parse(&["????.#...#... 4,1,1".to_string()])[0];
    //     let (s, g) = unfold(s, g);
    //     assert_eq!(count_ways2(&s, &g), 16);
    // }

    // #[test]
    // fn test_count_ways_unfolded_5() {
    //     let (s, g) = &parse(&["????.######..#####. 1,6,5".to_string()])[0];
    //     let (s, g) = unfold(s, g);
    //     assert_eq!(count_ways2(&s, &g), 2500);
    // }

    // #[test]
    // fn test_count_ways_unfolded_6() {
    //     let (s, g) = &parse(&["?###???????? 3,2,1".to_string()])[0];
    //     let (s, g) = unfold(s, g);
    //     assert_eq!(count_ways2(&s, &g), 506250);
    // }

    // #[test]
    // fn test_count_ways_unfolded_7() {
    //     let (s, g) = &parse(&["???..??????? 1,1,1,1".to_string()])[0];
    //     let (s, g) = unfold(s, g);
    //     assert_eq!(count_ways2(&s, &g), 506250);
    // }

    //    #[test]
    //     fn test_count_ways_unfolded_7_2() {
    //         let (s, g) = &parse(&["???..??????? 1,1,1,1".to_string()])[0];
    //         let (s, g) = unfold(s, g);
    //         assert_eq!(count_ways3(&s, &g), 405968);
    //     }

    #[test]
    fn test_count_ways_dp() {
        let (s, g) = &parse(&[". 1".to_string()])[0];
        assert_eq!(count_ways4(s, g), 0);
        let (s, g) = &parse(&["# 1".to_string()])[0];
        assert_eq!(count_ways4(s, g), 1);
        let (s, g) = &parse(&["? 1".to_string()])[0];
        assert_eq!(count_ways4(s, g), 1);
        let (s, g) = &parse(&["?? 1".to_string()])[0];
        assert_eq!(count_ways4(s, g), 2);
        let (s, g) = &parse(&["??? 1,1".to_string()])[0];
        assert_eq!(count_ways4(s, g), 1);
        let (s, g) = &parse(&["??? 1,1".to_string()])[0];
        assert_eq!(count_ways4(s, g), 1);
        let (s, g) = &parse(&["?.? 1,1".to_string()])[0];
        assert_eq!(count_ways4(s, g), 1);
        let (s, g) = &parse(&["?.# 1,1".to_string()])[0];
        assert_eq!(count_ways4(s, g), 1);
        let (s, g) = &parse(&["?.# 1,1".to_string()])[0];
        assert_eq!(count_ways4(s, g), 1);
        let (s, g) = &parse(&["??.## 2".to_string()])[0];
        assert_eq!(count_ways4(s, g), 1);
        let (s, g) = &parse(&["??.?? 2".to_string()])[0];
        assert_eq!(count_ways4(s, g), 2);
    }
}
