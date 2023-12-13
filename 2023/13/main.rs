use aoc::Grid;
use std::fmt::Debug;
use std::iter::*;

type Parsed = Vec<Vec<Vec<char>>>;

fn find_reflection<G, T>(d: &G, skip: (Option<i64>, Option<i64>)) -> (Option<i64>, Option<i64>)
where
    G: Grid<T>,
    T: PartialEq + Copy + Debug,
{
    let ([min_x, min_y], [max_x, max_y]) = d.extents();
    let h = max_y - min_y + 1;
    let w = max_x - min_x + 1;
    let mut yr = None;
    let mut xr = None;
    'outer: for y in min_y..=max_y {
        for delta in 0..h {
            for x in min_x..=max_x {
                let a = d.get_value([x, y - delta]);
                let b = d.get_value([x, y + delta + 1]);
                if a.is_none() || b.is_none() {
                    break;
                }
                if a != b {
                    continue 'outer;
                }
            }
        }
        // If we got here, there was a match
        if y != max_y {
            let yrr = Some(y + 1);
            if yrr != skip.1 {
                yr = yrr;
                break;
            }
        }
    }
    'outer2: for x in min_x..=max_x {
        for delta in 0..w {
            for y in min_y..=max_y {
                let a = d.get_value([x - delta, y]);
                let b = d.get_value([x + delta + 1, y]);
                if a.is_none() || b.is_none() {
                    break;
                }
                if a != b {
                    continue 'outer2;
                }
            }
        }
        // If we got here, there was a match
        if x != max_x {
            let xrr = Some(x + 1);
            if xrr != skip.0 {
                xr = xrr;
                break;
            }
        }
    }
    (xr, yr)
}

fn part1(data: &Parsed) -> i64 {
    let mut s = 0;
    for d in data {
        let (x, y) = find_reflection(d, (None, None));
        if let Some(x) = x {
            s += x;
        } else if let Some(y) = y {
            s += 100 * y;
        }
    }
    s
}

fn part2(data: &Parsed) -> i64 {
    let mut s = 0;
    'outer: for d in data {
        let ([min_x, min_y], [max_x, max_y]) = d.extents();
        let (ox, oy) = find_reflection(d, (None, None));
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let mut smudged = d.clone();
                if let Some(v) = smudged.get_value([x, y]) {
                    match v {
                        '#' => smudged.set_value([x, y], '.'),
                        '.' => smudged.set_value([x, y], '#'),
                        _ => panic!(),
                    }
                }
                let (xx, yy) = find_reflection(&smudged, (ox, oy));
                if xx.is_some() || yy.is_some() {
                    if let Some(x) = xx {
                        s += x;
                    }
                    if let Some(y) = yy {
                        s += 100 * y;
                    }
                    continue 'outer;
                }
            }
        }
        unreachable!();
    }
    s
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_by_empty_line(lines)
        .iter()
        .map(|x| x.iter().map(|x| x.chars().collect()).collect())
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "#.##..##.".into(),
            "..#.##.#.".into(),
            "##......#".into(),
            "##......#".into(),
            "..#.##.#.".into(),
            "..##..##.".into(),
            "#.#.##.#.".into(),
            "".into(),
            "#...##..#".into(),
            "#....#..#".into(),
            "..##..###".into(),
            "#####.##.".into(),
            "#####.##.".into(),
            "..##..###".into(),
            "#....#..#".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 400);
    }
}
