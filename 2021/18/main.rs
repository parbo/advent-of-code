use std::iter::*;
use std::time::Instant;

// #[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
// #[display("{thing}: {al}-{ah} or {bl}-{bh}")]
// struct Rule {
//     thing: String,
//     al: i64,
//     ah: i64,
//     bl: i64,
//     bh: i64,
// }

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

type ParsedItem = SnailNumber;
type Parsed = Vec<SnailNumber>;
type Answer = i64;

fn add_first(number: &SnailNumber, val: &mut i64) -> SnailNumber {
    if *val != 0 {
	println!("add first: {}, {}", number, val);
    }
    match number {
        SnailNumber::Num(x) => {
            let v = *val;
            *val = 0;
	    if v != 0 {
		println!("adding {}, {}", x, v);
	    }
            SnailNumber::Num(x + v)
        }
        SnailNumber::Pair(a, b) => SnailNumber::Pair(Box::new(add_first(a, val)), b.clone()),
    }
}

fn add_last(number: &SnailNumber, val: &mut i64) -> SnailNumber {
    if *val != 0 {
	println!("add last: {}, {}", number, val);
    }
    match number {
        SnailNumber::Num(x) => {
            let v = *val;
            *val = 0;
	    if v != 0 {
		println!("adding {}, {}", x, v);
	    }
            SnailNumber::Num(x + v)
        }
        SnailNumber::Pair(a, b) => SnailNumber::Pair(a.clone(), Box::new(add_last(b, val))),
    }
}

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
    exploded: &mut bool,
) -> (Option<SnailNumber>, Option<(Option<i64>, Option<i64>)>, bool) {
    // println!(
    //     "depth: {}, exploded: {}, exploding: {:?}",
    //     depth, exploded, number
    // );
    if let SnailNumber::Pair(a, b) = number {
        if depth == 4 && !*exploded {
	    println!("explode pair: {}", number);
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
            *exploded = true;
            return (None, Some((Some(aval), Some(bval))), false);
        }
        let explode_a = do_explode(a, depth + 1, exploded);
        let explode_b = do_explode(b, depth + 1, exploded);
        println!("a> {:?}", explode_a);
        println!("b> {:?}", explode_b);
        if let (n, Some((x, y)), inserted_a) = explode_a {
            let mut new_x = x;
            let mut new_y = y;
	    let mut inserted = false;
            let new_left = if let Some(nn) = n {
		if !inserted_a {
                    let mut v = new_x.unwrap_or(0);
                    let r = add_last(&a, &mut v);
                    if v == 0 {
			new_x = None;
                    }
		    r
		} else {
		    nn
		}
            } else {
		inserted = true;
                // println!("1 replacing {:?} with 0", a);
                SnailNumber::Num(0)
            };
            let old_right = if let Some(old_b) = explode_b.0 {
                old_b
            } else {
                (**b).clone()
            };
            let new_right = if let SnailNumber::Num(val) = old_right {
                if let Some(yy) = y {
                    new_y = None;
                    // println!("2 adding {}, {}", val, yy);
                    SnailNumber::Num(val + yy)
                } else {
                    SnailNumber::Num(val)
                }
            } else {
                let mut v = new_y.unwrap_or(0);
                let r = add_first(&old_right, &mut v);
		println!("r: {}", r);
                if v == 0 {
                    new_y = None;
                }
                r
            };
	    let mut num = SnailNumber::Pair(Box::new(new_left), Box::new(new_right));
            let ret = (
                Some(num),
                if new_x.is_some() || new_y.is_some() {
                    Some((new_x, new_y))
                } else {
                    None
                },
		inserted
            );
            // println!("a returning {:?}", ret);
            return ret;
        } else if let (n, Some((x, y)), _inserted_b) = explode_b {
            let mut new_x = x;
	    let mut inserted = false;
            let old_left = if let Some(old_a) = explode_a.0 {
                old_a
            } else {
                (**a).clone()
            };
            let new_left = if let SnailNumber::Num(val) = old_left {
                if let Some(xx) = x {
                    new_x = None;
                    // println!("3 adding {}, {}", val, xx);
                    SnailNumber::Num(val + xx)
                } else {
                    SnailNumber::Num(val)
                }
            } else {
                old_left
            };
            let new_right = if let Some(nn) = n {
                nn
            } else {
                // println!("4 replacing {:?} with 0", b);
		inserted = true;
                SnailNumber::Num(0)
            };
            let ret = (
                Some(SnailNumber::Pair(Box::new(new_left), Box::new(new_right))),
                if new_x.is_some() || y.is_some() {
                    Some((new_x, y))
                } else {
                    None
                },
		inserted
            );
            // println!("b returning {:?}", ret);
            return ret;
        } else {
            let ret = (
                Some(SnailNumber::Pair(
                    Box::new(explode_a.0.unwrap()),
                    Box::new(explode_b.0.unwrap()),
                )),
                None,
		false
            );
            // println!("c returning {:?}", ret);
            return ret;
        }
    }
    (Some(number.clone()), None, false)
}

fn explode(number: &SnailNumber) -> SnailNumber {
    let mut exploded = false;
    let r = do_explode(number, 0, &mut exploded);
    let ret = r.0.unwrap();
    println!("explode result: {}, {:?}", ret, r.1);
    ret
}

fn reduce(number: &SnailNumber) -> SnailNumber {
    let mut n = number.clone();
    loop {
        let old_n = n.clone();
        loop {
            println!("exploding {}", n);
            let new_n = explode(&n);
            if new_n == n {
                break;
            }
            n = new_n;
        }
        loop {
            println!("splitting {}", n);
            let new_n = split(&n);
            if new_n == n {
                break;
            }
            n = new_n;
        }
        if old_n == n {
            break;
        }
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
        print!("sum: {} + {} = ", n, nn);
        n = reduce(&SnailNumber::Pair(Box::new(n), Box::new(nn.clone())));
        println!("{}", n);
    }
    n
}

fn part1(numbers: &[ParsedItem]) -> Answer {
    let mut n = sum(numbers);
    println!("added: {}", n);
    n = reduce(&n);
    magnitude(&n)
}

fn part2(_: &[ParsedItem]) -> Answer {
    0
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
    let start_time = Instant::now();
    let (part, lines) = aoc::read_lines();
    let io_time = Instant::now();
    let parsed = parse(&lines);
    let parse_time = Instant::now();
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    let done_time = Instant::now();
    println!(
        "read: {:?}, parse: {:?}, solve: {:?}\n",
        io_time.duration_since(start_time),
        parse_time.duration_since(io_time),
        done_time.duration_since(parse_time)
    );
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]".into(),
            "[[[5,[2,8]],4],[5,[[9,9],0]]]".into(),
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]".into(),
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]".into(),
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]".into(),
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]".into(),
            "[[[[5,4],[7,7]],8],[[8,3],8]]".into(),
            "[[9,3],[[9,9],[6,[4,9]]]]".into(),
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]".into(),
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]".into(),
        ]
    }

    // #[test]
    // fn test_part1() {
    // 	let parsed = parse(&example());
    //     assert_eq!(part1(&parsed), 4140);
    // }

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

    // #[test]
    // fn test_sum_4() {
    //     let lines = vec![
    //         "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".into(),
    //         "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".into(),
    //         "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]".into(),
    //         "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]".into(),
    //         "[7,[5,[[3,8],[1,4]]]]".into(),
    //         "[[2,[2,2]],[8,[8,1]]]".into(),
    //         "[2,9]".into(),
    //         "[1,[[[9,3],9],[[9,0],[0,7]]]]".into(),
    //         "[[[5,[7,4]],7],1]".into(),
    //         "[[[[4,2],2],6],[8,7]]".into(),
    //     ];
    //     let parsed = parse(&lines);
    //     let num = sum(&parsed);
    //     let expected = parse_line(
    //         &"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
    //             .chars()
    //             .collect::<Vec<char>>(),
    //     );
    //     assert_eq!(num, expected.0);
    // }

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
