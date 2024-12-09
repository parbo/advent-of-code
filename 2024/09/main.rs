use std::iter::*;

type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let mut exp: Vec<_> = data
        .iter()
        .enumerate()
        .flat_map(|(ix, num)| {
            let v = if ix % 2 == 0 {
                Some((ix / 2) as i64)
            } else {
                None
            };
            repeat(v).take(*num as usize)
        })
        .collect();
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
    let mut exp: Vec<_> = data
        .iter()
        .enumerate()
        .map(|(ix, num)| {
            let v = if ix % 2 == 0 {
                Some((ix / 2) as i64)
            } else {
                None
            };
            (v, *num)
        })
        .collect();
    let mut i = exp.len() - 1;
    loop {
        let mut inserted = 0;
        if exp[i].0.is_some() {
            if let Some(ix) = exp
                .iter()
                .take(i)
                .position(|(v, num)| v.is_none() && *num >= exp[i].1)
            {
                let al = exp[i].1;
                let bl = exp[ix].1;
                assert!(bl >= al);
                exp.swap(i, ix);
                if al != bl {
                    exp[i].1 = al;
                    exp.insert(ix + 1, (None, bl - al));
                    inserted += 1;
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
