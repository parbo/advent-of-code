use std::iter::*;

type Parsed = Vec<Shuffle>;

enum Shuffle {
    DealIntoNewStack,
    Cut(i64),
    DealWithIncrement(usize),
}

fn pos_mod(x: i128, y: i128) -> i128 {
    x.rem_euclid(y)
}

fn mod_inverse(a: i128, m: i128) -> i128 {
    modinverse::modinverse(a, m).unwrap()
}

fn shuffle_idx(how: &Vec<Shuffle>, len: i128, idx: i128) -> i128 {
    let mut new_idx = idx;
    for s in how {
        match s {
            Shuffle::DealIntoNewStack => {
                new_idx = len - new_idx - 1;
            }
            Shuffle::Cut(x) => {
                new_idx = new_idx + len - *x as i128;
            }
            Shuffle::DealWithIncrement(x) => {
                new_idx *= *x as i128;
            }
        }
        new_idx = pos_mod(new_idx, len);
    }
    new_idx
}

fn reverse_shuffle_idx(how: &[Shuffle], len: i128, idx: i128) -> i128 {
    let mut new_idx = idx;
    for s in how.iter().rev() {
        match s {
            Shuffle::DealIntoNewStack => {
                new_idx = len - new_idx - 1;
            }
            Shuffle::Cut(x) => {
                new_idx += *x as i128;
            }
            Shuffle::DealWithIncrement(x) => {
                let xx = *x as i128;
                let f = mod_inverse(xx, len);
                new_idx *= f;
            }
        }
        new_idx = pos_mod(new_idx, len);
    }
    new_idx
}

fn shuffle(how: &Vec<Shuffle>, len: i128) -> Vec<i128> {
    let deck: Vec<i128> = (0..len).collect();
    let mut new_deck = vec![0; len as usize];
    for i in 0..len {
        let x = shuffle_idx(how, len, i);
        new_deck[x as usize] = deck[i as usize];
    }
    new_deck
}

fn part1(input: &Parsed) -> i128 {
    let len = 10007i128;
    let shuffled = shuffle(input, len);
    shuffled
        .iter()
        .enumerate()
        .find(|(_, x)| **x == 2019)
        .unwrap()
        .0 as i128
}

fn part2(input: &Parsed) -> i128 {
    let len = 119315717514047i128;
    let times = 101741582076661i128;
    // a*(a*(a*x + b) + b) + b = z
    let x = 2020;
    let y = reverse_shuffle_idx(input, len, x);
    let z = reverse_shuffle_idx(input, len, y);
    println!("x: {}, y: {}, z: {}", x, y, z);
    let a = pos_mod((y - z) * mod_inverse(x - y, len), len);
    let b = pos_mod(y - a * x, len);
    println!("a: {}, b: {}", a, b);
    let xx = pos_mod(b * mod_inverse(1 - a, len), len);
    let yy = xx - xx * aoc::mod_exp(a, times, len);
    let ans = aoc::mod_exp(a, times, len) * x + yy;
    println!("ans: {}, ans % len: {}", ans, pos_mod(ans, len));
    pos_mod(ans, len)
}

fn parse(lines: &[String]) -> Parsed {
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
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::{parse, pos_mod, reverse_shuffle_idx, shuffle, Shuffle};

    fn validate(how: &[Shuffle], idx: &[i128], len: i128) {
        for (i, id) in idx.iter().enumerate() {
            assert_eq!(pos_mod(reverse_shuffle_idx(how, len, i as i128), len), *id);
        }
    }

    #[test]
    fn test_shuffle_rev() {
        let input = vec!["deal into new stack".to_string()];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
        validate(&how, &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0], 10);
    }

    #[test]
    fn test_shuffle_rev2() {
        let input = vec![
            "deal into new stack".to_string(),
            "deal into new stack".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        validate(&how, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 10);
    }

    #[test]
    fn test_shuffle_incr() {
        let input = vec!["deal with increment 1".to_string()];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        validate(&how, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 10);
    }

    #[test]
    fn test_shuffle_inc3() {
        let input = vec!["deal with increment 3".to_string()];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
        validate(&how, &[0, 7, 4, 1, 8, 5, 2, 9, 6, 3], 10);
    }

    #[test]
    fn test_shuffle_cut3() {
        let input = vec!["cut 3".to_string()];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
        validate(&how, &[3, 4, 5, 6, 7, 8, 9, 0, 1, 2], 10);
    }

    #[test]
    fn test_shuffle_cutminus4() {
        let input = vec!["cut -4".to_string()];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
        validate(&how, &[6, 7, 8, 9, 0, 1, 2, 3, 4, 5], 10);
    }

    #[test]
    fn test_shuffle_inc_rev() {
        let input = vec![
            "deal with increment 3".to_string(),
            "deal into new stack".to_string(),
        ];
        let how = parse(&input);
        assert_eq!(shuffle(&how, 10), vec![3, 6, 9, 2, 5, 8, 1, 4, 7, 0]);
        validate(&how, &[3, 6, 9, 2, 5, 8, 1, 4, 7, 0], 10);
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
        validate(&how, &[0, 3, 6, 9, 2, 5, 8, 1, 4, 7], 10);
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
        validate(&how, &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6], 10);
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
        validate(&how, &[6, 3, 0, 7, 4, 1, 8, 5, 2, 9], 10);
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
        validate(&how, &[9, 2, 5, 8, 1, 4, 7, 0, 3, 6], 10);
    }
}
