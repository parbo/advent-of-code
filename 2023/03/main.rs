use aoc::Grid;
use std::{collections::HashMap, iter::*};

type ParsedItem = Vec<char>;
type Parsed = Vec<ParsedItem>;

fn vec_to_num(num: &[u32]) -> u32 {
    let mut tens = 1;
    let mut n = 0;
    for nn in num.iter().rev() {
        n += nn * tens;
        tens *= 10;
    }
    n
}

fn makenum(num: &mut Vec<u32>, adjacent: &mut bool) -> u32 {
    let n = vec_to_num(num);
    num.clear();
    if *adjacent {
        *adjacent = false;
        n
    } else {
        0
    }
}

fn part1(data: &Parsed) -> i64 {
    let mut num = vec![];
    let mut adjacent = false;
    let mut ss = 0;
    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if let Some(n) = c.to_digit(10) {
                num.push(n);
                if !adjacent {
                    for nb in aoc::neighbors_incl_diagonals([x as i64, y as i64]) {
                        if let Some(v) = data.get_value(nb) {
                            if v.is_ascii_digit() || v == '.' {
                                // nop
                            } else {
                                adjacent = true;
                            }
                        }
                    }
                }
            } else {
                ss += makenum(&mut num, &mut adjacent);
            }
        }
        ss += makenum(&mut num, &mut adjacent);
    }
    ss += makenum(&mut num, &mut adjacent);
    ss as i64
}

fn makegear(
    num: &mut Vec<u32>,
    adjacent: &mut Vec<aoc::Point>,
    gears: &mut HashMap<aoc::Point, Vec<u32>>,
) {
    let n = vec_to_num(num);
    num.clear();
    if !adjacent.is_empty() {
        for nb in adjacent.iter() {
            gears.entry(*nb).or_default().push(n);
        }
        adjacent.clear()
    }
}

fn part2(data: &Parsed) -> i64 {
    let mut num = vec![];
    let mut adjacent = vec![];
    let mut gears = HashMap::new();
    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if let Some(n) = c.to_digit(10) {
                num.push(n);
                for nb in aoc::neighbors_incl_diagonals([x as i64, y as i64]) {
                    if let Some(v) = data.get_value(nb) {
                        if v == '*' && !adjacent.contains(&nb) {
                            adjacent.push(nb);
                        }
                    }
                }
            } else {
                makegear(&mut num, &mut adjacent, &mut gears);
            }
        }
        makegear(&mut num, &mut adjacent, &mut gears);
    }
    makegear(&mut num, &mut adjacent, &mut gears);
    gears
        .iter()
        .filter(|(_k, v)| v.len() == 2)
        .map(|(_k, v)| v[0] * v[1])
        .sum::<u32>() as i64
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "467..114..".into(),
            "...*......".into(),
            "..35..633.".into(),
            "......#...".into(),
            "617*......".into(),
            ".....+.58.".into(),
            "..592.....".into(),
            "......755.".into(),
            "...$.*....".into(),
            ".664.598..".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 467835);
    }

    #[test]
    fn test_part2_multi_gear() {
        let v = vec![".123.".into(), "*...*".into(), ".345.".into()];
        assert_eq!(part2(&parse(&v)), 84870);
    }
}
