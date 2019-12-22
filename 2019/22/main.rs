use aoc;
use std::iter::*;

enum Shuffle {
    DealIntoNewStack,
    Cut(i64),
    DealWithIncrement(usize),
}

fn pos_mod(x: i128, y: i128) -> i128 {
    x.rem_euclid(y)
}

fn mod_inverse(a: i128, m: i128) -> i128 {
    println!("a: {}, m: {}", a, m);
    aoc::modinverse(a, m).unwrap()
}

fn shuffle_idx(how: &Vec<Shuffle>, len: i128, idx: i128) -> i128 {
    let mut new_idx = idx;
    // println!("====== {}", idx);
    for s in how {
        match s {
            Shuffle::DealIntoNewStack => {
                new_idx = len - new_idx - 1;
            }
            Shuffle::Cut(x) => {
                new_idx = new_idx + len - *x as i128;
            }
            Shuffle::DealWithIncrement(x) => {
                new_idx = new_idx * *x as i128;
            }
        }
        // println!("{}, {}", new_idx, pos_mod(new_idx, len));
        new_idx = pos_mod(new_idx, len);
    }
    new_idx
}

fn reverse_shuffle_idx(how: &Vec<Shuffle>, len: i128, idx: i128) -> i128 {
    let mut new_idx = idx;
    for s in how.iter().rev() {
        match s {
            Shuffle::DealIntoNewStack => {
                new_idx = len - new_idx - 1;
            }
            Shuffle::Cut(x) => {
                new_idx = new_idx.clone() + *x as i128;
            }
            Shuffle::DealWithIncrement(x) => {
		let xx = *x as i128;
		let f = mod_inverse(xx, len);
                new_idx = f * new_idx;
            }
        }
        // println!("{}, {}", new_idx, pos_mod(new_idx, len));
    }
    new_idx
}

fn shuffle(how: &Vec<Shuffle>, len: i128) -> Vec<i128> {
    let deck: Vec<i128> = (0..len).into_iter().collect();
    let mut new_deck = vec![0; len as usize];
    for i in 0..len {
        let x = shuffle_idx(how, len, i);
        println!("x: {}, i: {}", x, i);
        new_deck[x as usize] = deck[i as usize];
    }
    new_deck
}

fn part1(input: &Vec<Shuffle>) -> i128 {
    let len = 10007i128;
    let shuffled = shuffle(input, len);
    shuffled
        .iter()
        .enumerate()
        .find(|(_, x)| **x == 2019)
        .unwrap()
        .0 as i128
}

fn part2(input: &Vec<Shuffle>) -> i128 {
    let len = 119315717514047i128;
    let times = 101741582076661i128;
    println!(
        "{}, {}, {}, {}",
        len / times,
        len % times,
        aoc::gcd(len, times),
        aoc::lcm(len, times)
    );
    let mut i = 0;
    let mut ix = 2020;
    let mod_ix = mod_inverse(2020, times);
    // println!("mod_ix: {}", mod_ix);
    let new_ix = reverse_shuffle_idx(input, len, 2020);
    let ans = mod_ix * new_ix;
    println!("{}, {}", ans, pos_mod(ans, len));
    new_ix
}

fn parse(lines: &Vec<String>) -> Vec<Shuffle> {
    let mut res = vec![];
    for line in lines {
        if line == "deal into new stack" {
            res.push(Shuffle::DealIntoNewStack);
        } else if let Ok(x) = aoc::scan!("cut {}" <- line) {
            res.push(Shuffle::Cut(x));
        } else if let Ok(x) = aoc::scan!("deal with increment {}" <- line) {
            res.push(Shuffle::DealWithIncrement(x));
        } else {
            panic!();
        }
    }
    res
}

fn main() {
    let (part, lines) = aoc::read_lines();
    //let parsed = aoc::parse_intcode(&lines);
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
    use super::{parse, reverse_shuffle_idx, shuffle, pos_mod};

    #[test]
    fn test_shuffle_rev() {
        let input = vec!["deal into new stack".to_string()];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_shuffle_rev2() {
        let input = vec![
            "deal into new stack".to_string(),
            "deal into new stack".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_shuffle_incr() {
        let input = vec!["deal with increment 1".to_string()];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_shuffle_inc3() {
        let input = vec!["deal with increment 3".to_string()];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
        assert_eq!(pos_mod(reverse_shuffle_idx(&how, 10, 0), 10), 0);
        assert_eq!(pos_mod(reverse_shuffle_idx(&how, 10, 1), 10), 7);
        assert_eq!(pos_mod(reverse_shuffle_idx(&how, 10, 2), 10), 4);
        assert_eq!(pos_mod(reverse_shuffle_idx(&how, 10, 3), 10), 1);
        assert_eq!(pos_mod(reverse_shuffle_idx(&how, 10, 4), 10), 8);
    }

    #[test]
    fn test_shuffle_cut3() {
        let input = vec!["cut 3".to_string()];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_shuffle_cutminus4() {
        let input = vec!["cut -4".to_string()];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_shuffle_inc_rev() {
        let input = vec![
            "deal with increment 3".to_string(),
            "deal into new stack".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![3, 6, 9, 2, 5, 8, 1, 4, 7, 0]);
    }

    #[test]
    fn test_shuffle_1() {
        let input = vec![
            "deal with increment 7".to_string(),
            "deal into new stack".to_string(),
            "deal into new stack".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
        assert_eq!(pos_mod(reverse_shuffle_idx(&how, 10, 0), 10), 0);
        assert_eq!(pos_mod(reverse_shuffle_idx(&how, 10, 1), 10), 3);
        assert_eq!(pos_mod(reverse_shuffle_idx(&how, 10, 2), 10), 6);
    }

    #[test]
    fn test_shuffle_2() {
        let input = vec![
            "cut 6".to_string(),
            "deal with increment 7".to_string(),
            "deal into new stack".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_shuffle_3() {
        let input = vec![
            "deal with increment 7".to_string(),
            "deal with increment 9".to_string(),
            "cut -2".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_shuffle_4() {
        let input = vec![
            "deal into new stack".to_string(),
            "cut -2".to_string(),
            "deal with increment 7".to_string(),
            "cut 8".to_string(),
            "cut -4".to_string(),
            "deal with increment 7".to_string(),
            "cut 3".to_string(),
            "deal with increment 9".to_string(),
            "deal with increment 3".to_string(),
            "cut -1".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
