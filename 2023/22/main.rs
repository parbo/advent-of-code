use std::iter::*;

use aoc::Vec3;

type ParsedItem = (Vec3, Vec3);
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let mut data = data.to_vec();
    // sort on descending z
    data.sort_by(|a, b| b.0[2].cmp(&a.0[2]));
    dbg!(&data);
    let mut num = 1;
    for i in 1..data.len() {
        let d = data[i];
        let istart = [d.0[0], d.0[1]];
        let iend = [d.1[0], d.1[1]];
        let mut any_overlap = false;
        for j in (i + 1)..data.len() {
            let dj = data[j];
            let jstart = [dj.0[0], dj.0[1]];
            let jend = [dj.1[0], dj.1[1]];
            let min_xx = istart[0].max(jstart[0]);
            let min_yy = istart[1].max(jstart[1]);
            let max_xx = iend[0].min(jend[0]);
            let max_yy = iend[0].min(jend[1]);
            if min_xx != max_xx || min_yy != max_yy {
                println!("overlap {} {}, {:?} {:?}", i, j, d, dj);
                any_overlap = true;
                break;
            }
        }
        if !any_overlap {
            println!("{} does not overlap any other", i);
            num += 1;
        }
    }
    num
}

fn part2(_: &Parsed) -> i64 {
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let (a, b) = x.split_once('~').unwrap();
            let a = aoc::things::<i64>(a);
            let b = aoc::things::<i64>(b);
            (a[0..3].try_into().unwrap(), b[0..3].try_into().unwrap())
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn example() -> Vec<String> {
    //     let s = include_str!("example.txt");
    //     s.lines().map(|x| x.to_string()).collect()
    // }

    // fn example() -> Vec<String> {
    // 	   vec![
    //         "0".into()
    //     ]
    // }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&parse(&example())), 0);
    // }
}
