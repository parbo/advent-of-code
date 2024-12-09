use std::iter::*;

// #[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
// #[display("{thing}: {al}-{ah} or {bl}-{bh}")]
// struct Rule {
//     thing: String,
//     al: i64,
//     ah: i64,
//     bl: i64,
//     bh: i64,
// }

type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let mut exp = Vec::with_capacity(data.len() * 9);
    // fill up
    for (ix, num) in data.iter().enumerate() {
        let v = if ix % 2 == 0 {
            Some((ix / 2) as i64)
        } else {
            None
        };
        exp.extend(repeat(v).take(*num as usize));
    }
    for i in (0..exp.len()).rev() {
        let ix = exp.iter().position(|x| x.is_none()).unwrap();
        exp.swap(i, ix);
    }
    exp.iter()
        .filter_map(|x| *x)
        .enumerate()
        .map(|(ix, x)| ix as i64 * x)
        .sum()
}

fn part2(data: &Parsed) -> i64 {
    let mut exp = vec![];
    // fill up
    for (ix, num) in data.iter().enumerate() {
        let v = if ix % 2 == 0 {
            Some((ix / 2) as i64)
        } else {
            None
        };
        exp.push((v, *num));
    }
    let mut i = exp.len() - 1;
    loop {
        let mut inserted = 0;
        if exp[i].0.is_some() {
            // dbg!(exp[i], &exp);
            if let Some(ix) = exp
                .iter()
                .position(|(v, num)| v.is_none() && *num >= exp[i].1)
            {
                if ix < i {
                    let al = exp[i].1;
                    let bl = exp[ix].1;
                    assert!(bl >= al);
                    exp.swap(i, ix);
                    if al != bl {
                        exp[i].1 = al;
                        exp.insert(ix + 1, (None, bl - al));
                        // Compact the swapped nones
                        let mut ix = ix + 1;
                        while ix + 1 < exp.len() {
                            if exp[ix].0.is_none() && exp[ix + 1].0.is_none() {
                                exp[ix].1 += exp[ix + 1].1;
                                exp.remove(ix + 1);
                            } else {
                                ix += 1;
                            }
                        }
                        inserted += 1;
                    }
                }
            }
        }
        if i == 0 {
            break;
        }
        if inserted == 0 {
            i -= 1;
        }
    }
    exp.iter()
        .flat_map(|(v, num)| repeat(v.unwrap_or(0)).take(*num as usize))
        .enumerate()
        .map(|(ix, x)| ix as i64 * x)
        .sum()
}

fn parse(lines: &[String]) -> Parsed {
    lines[0]
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec!["2333133121414131402".into()]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 2858);
    }
}
