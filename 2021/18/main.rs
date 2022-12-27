use std::iter::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailNumber {
    Num(i64),
    Pair(Box<SnailNumber>, Box<SnailNumber>),
}

impl std::fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailNumber::Num(x) => {
                write!(f, "{}", x)
            }
            SnailNumber::Pair(a, b) => {
                write!(f, "[{},{}]", *a, *b)
            }
        }
    }
}

type Parsed = Vec<SnailNumber>;
type Answer = i64;

fn do_split(number: &SnailNumber, split: &mut bool) -> SnailNumber {
    // println!("split: {:?}", number);
    match number {
        SnailNumber::Num(x) => {
            if *x >= 10 {
                *split = true;
                SnailNumber::Pair(
                    Box::new(SnailNumber::Num(x / 2)),
                    Box::new(SnailNumber::Num((x + 1) / 2)),
                )
            } else {
                number.clone()
            }
        }
        SnailNumber::Pair(a, b) => {
            let new_a = if !*split {
                do_split(a, split)
            } else {
                (**a).clone()
            };
            let new_b = if !*split {
                do_split(b, split)
            } else {
                (**b).clone()
            };
            // println!("new: {:?}, {:?}", new_a, new_b);
            SnailNumber::Pair(Box::new(new_a), Box::new(new_b))
        }
    }
}

fn split(number: &SnailNumber) -> SnailNumber {
    let mut s = false;
    do_split(number, &mut s)
}

fn do_explode(
    number: &SnailNumber,
    depth: usize,
    exploded: &mut Option<(i64, i64)>,
) -> SnailNumber {
    if let SnailNumber::Pair(a, b) = number {
        if depth == 4 && !exploded.is_some() {
            // println!("explode pair: {}", number);
            // explode this
            let aval = if let SnailNumber::Num(aval) = **a {
                aval
            } else {
                panic!();
            };
            let bval = if let SnailNumber::Num(bval) = **b {
                bval
            } else {
                panic!();
            };
            *exploded = Some((aval, bval));
            return SnailNumber::Num(-1);
        }
        let ea = do_explode(&(**a).clone(), depth + 1, exploded);
        let eb = do_explode(&(**b).clone(), depth + 1, exploded);
        return SnailNumber::Pair(Box::new(ea), Box::new(eb));
    }
    number.clone()
}

fn explode(number: &SnailNumber) -> SnailNumber {
    let mut exploded = None;
    let r = do_explode(number, 0, &mut exploded);
    if let Some((a, b)) = exploded {
        let r_s = format!("{}", r);
        let parts = aoc::split_str(&r_s, "-1");
        let rev: String = parts[0].chars().rev().collect();
        let re = regex::Regex::new(r"(^[^\d]*)(\d+)(.*$)").unwrap();
        let new_left = re.replace(&rev, |c: &regex::Captures| {
            //	    println!("{} + {}, {}",  c[2].parse::<i64>().unwrap(), a, c[2].parse::<i64>().unwrap() + a);
            format!(
                "{}{}{}",
                &c[3].chars().rev().collect::<String>(),
                c[2].chars()
                    .rev()
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap()
                    + a,
                &c[1].chars().rev().collect::<String>()
            )
        });
        // println!("old: {}, new_left: {}", parts[0], new_left);
        let new_right = re.replace(parts[1], |c: &regex::Captures| {
            format!("{}{}{}", &c[1], c[2].parse::<i64>().unwrap() + b, &c[3])
        });
        let n_s = format!("{}0{}", new_left, new_right);
        parse_line(&n_s.chars().collect::<Vec<char>>()).0
    } else {
        r
    }
}

fn reduce(number: &SnailNumber) -> SnailNumber {
    let mut n = number.clone();
    loop {
        // println!("exploding {}", n);
        let new_n = explode(&n);
        if new_n != n {
            n = new_n;
            continue;
        }
        // println!("splitting {}", n);
        let new_n = split(&n);
        if new_n == n {
            break;
        }
        n = new_n;
    }
    n
}

fn magnitude(number: &SnailNumber) -> i64 {
    match number {
        SnailNumber::Pair(a, b) => {
            let left_mag = magnitude(a);
            let right_mag = magnitude(b);
            3 * left_mag + 2 * right_mag
        }
        SnailNumber::Num(x) => *x,
    }
}

fn sum(numbers: &[SnailNumber]) -> SnailNumber {
    let mut n = numbers[0].clone();
    for nn in &numbers[1..] {
        n = reduce(&SnailNumber::Pair(Box::new(n), Box::new(nn.clone())));
    }
    n
}

fn part1(numbers: &Parsed) -> Answer {
    let mut n = sum(numbers);
    n = reduce(&n);
    magnitude(&n)
}

fn part2(numbers: &Parsed) -> Answer {
    let mut mx = 0;
    for i in 0..numbers.len() {
        for j in 1..numbers.len() {
            let s = sum(&[numbers[i].clone(), numbers[j].clone()]);
            let m = magnitude(&s);
            mx = mx.max(m);
        }
    }
    mx
}

fn parse_num(chars: &[char]) -> (i64, usize) {
    let mut pos = 0;
    while chars[pos] != ',' && chars[pos] != ']' {
        pos += 1;
    }
    (
        chars[0..pos]
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap(),
        pos + 1,
    )
}

fn parse_line(line: &[char]) -> (SnailNumber, usize) {
    if line[0] == '[' {
        let (left, left_len) = parse_line(&line[1..]);
        let (right, right_len) = parse_line(&line[(1 + left_len)..]);
        (
            SnailNumber::Pair(Box::new(left), Box::new(right)),
            1 + left_len + right_len + 1,
        )
    } else if line[0] == ']' || line[0] == ',' {
        panic!();
    } else {
        // Read the number
        let (num, len) = parse_num(line);
        (SnailNumber::Num(num), len)
    }
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| parse_line(&x.chars().collect::<Vec<char>>()).0)
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explode_1() {
        let num = parse_line(&"[[[[[9,8],1],2],3],4]".chars().collect::<Vec<char>>());
        let expected = parse_line(&"[[[[0,9],2],3],4]".chars().collect::<Vec<char>>());
        let res = explode(&num.0);
        assert_eq!(res, expected.0, "{} != {}", res, expected.0);
    }

    #[test]
    fn test_explode_2() {
        let num = parse_line(&"[7,[6,[5,[4,[3,2]]]]]".chars().collect::<Vec<char>>());
        let expected = parse_line(&"[7,[6,[5,[7,0]]]]".chars().collect::<Vec<char>>());
        let res = explode(&num.0);
        assert_eq!(res, expected.0, "{} != {}", res, expected.0);
    }

    #[test]
    fn test_explode_3() {
        let num = parse_line(&"[[6,[5,[4,[3,2]]]],1]".chars().collect::<Vec<char>>());
        let expected = parse_line(&"[[6,[5,[7,0]]],3]".chars().collect::<Vec<char>>());
        let res = explode(&num.0);
        assert_eq!(res, expected.0, "{} != {}", res, expected.0);
    }

    #[test]
    fn test_explode_4() {
        let num = parse_line(
            &"[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let expected = parse_line(
            &"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let res = explode(&num.0);
        assert_eq!(res, expected.0, "{} != {}", res, expected.0);
    }

    #[test]
    fn test_explode_5() {
        let num = parse_line(
            &"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let expected = parse_line(
            &"[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let res = explode(&num.0);
        assert_eq!(res, expected.0, "{} != {}", res, expected.0);
    }

    #[test]
    fn test_split_1() {
        let num = parse_line(
            &"[[[[0,7],4],[15,[0,13]]],[1,1]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let expected = parse_line(
            &"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let res = split(&num.0);
        assert_eq!(res, expected.0, "{} != {}", res, expected.0);
    }

    #[test]
    fn test_split_2() {
        let num = parse_line(
            &"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let expected = parse_line(
            &"[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let res = split(&num.0);
        assert_eq!(res, expected.0, "{} != {}", res, expected.0);
    }

    #[test]
    fn test_reduce_1() {
        let num = parse_line(
            &"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let expected = parse_line(
            &"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let res = reduce(&num.0);
        assert_eq!(res, expected.0, "{} != {}", res, expected.0);
    }

    #[test]
    fn test_reduce_2() {
        let num = parse_line(
            &"[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let expected = parse_line(
            &"[[[[3,0],[5,3]],[4,4]],[5,5]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let res = reduce(&num.0);
        assert_eq!(res, expected.0, "{} != {}", res, expected.0);
    }

    #[test]
    fn test_reduce_3() {
        let num = parse_line(
            &"[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let expected = parse_line(
            &"[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        let res = reduce(&num.0);
        assert_eq!(res, expected.0, "{} != {}", res, expected.0);
    }

    #[test]
    fn test_sum_1() {
        let lines = vec![
            "[1,1]".into(),
            "[2,2]".into(),
            "[3,3]".into(),
            "[4,4]".into(),
        ];
        let parsed = parse(&lines);
        let res = sum(&parsed);
        let expected = parse_line(
            &"[[[[1,1],[2,2]],[3,3]],[4,4]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        assert_eq!(res, expected.0);
    }

    #[test]
    fn test_sum_2() {
        let lines = vec![
            "[1,1]".into(),
            "[2,2]".into(),
            "[3,3]".into(),
            "[4,4]".into(),
            "[5,5]".into(),
        ];
        let parsed = parse(&lines);
        let res = sum(&parsed);
        let expected = parse_line(
            &"[[[[3,0],[5,3]],[4,4]],[5,5]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        assert_eq!(res, expected.0);
    }

    #[test]
    fn test_sum_3() {
        let lines = vec![
            "[1,1]".into(),
            "[2,2]".into(),
            "[3,3]".into(),
            "[4,4]".into(),
            "[5,5]".into(),
            "[6,6]".into(),
        ];
        let parsed = parse(&lines);
        let res = sum(&parsed);
        let expected = parse_line(
            &"[[[[5,0],[7,4]],[5,5]],[6,6]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        assert_eq!(res, expected.0);
    }

    #[test]
    fn test_magnitude_1() {
        let num = parse_line(&"[[1,2],[[3,4],5]]".chars().collect::<Vec<char>>());
        assert_eq!(magnitude(&num.0), 143);
    }

    #[test]
    fn test_magnitude_2() {
        let num = parse_line(
            &"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        assert_eq!(magnitude(&num.0), 1384);
    }

    #[test]
    fn test_magnitude_3() {
        let num = parse_line(
            &"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .chars()
                .collect::<Vec<char>>(),
        );
        assert_eq!(magnitude(&num.0), 3488);
    }
}
