use aoc::{Mat4, Vec4};
use std::collections::{HashMap, HashSet};
use std::iter::*;
use std::time::Instant;

type ParsedItem = Vec<Vec4>;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn make_matrices() -> Vec<Mat4> {
    let mut matrices = vec![];
    for f in 0..4 {
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if x == y || x == z || y == z {
                        continue;
                    }
                    let rows = [
                        [if f == 1 { -1 } else { 1 }, 0, 0, 0],
                        [0, if f == 2 { -1 } else { 1 }, 0, 0],
                        [0, 0, if f == 3 { -1 } else { 1 }, 0],
                    ];
                    let m = [rows[x], rows[y], rows[z], [0, 0, 0, 1]];
                    matrices.push(m);
                }
            }
        }
    }
    // println!("num m: {}", matrices.len());
    // for m in &matrices {
    //     println!("{:?}", m);
    // }
    // assert_eq!(matrices.len(), 24);
    matrices
}

fn align(matrices: &[Mat4], s_i: &[Vec4], s_j: &[Vec4]) -> Option<(Mat4, HashSet<Vec4>)> {
    let seti = s_i.iter().copied().collect::<HashSet<_>>();
    for m in matrices {
        // Rotate beacons
        let rbj = s_j
            .iter()
            .map(|bj| aoc::mat_transform(*m, *bj))
            .collect::<Vec<_>>();
	println!("{:?}", s_i);
	println!("{:?}", s_j);
	println!("{:?}", rbj);
	println!();
        for b1 in s_i {
            for b2 in &rbj {
                let dist = aoc::vec4_sub(*b1, *b2);
                let tbj = rbj
                    .iter()
                    .map(|b| aoc::vec4_add(*b, dist))
                    .collect::<Vec<_>>();
                let setj = tbj.iter().copied().collect::<HashSet<_>>();
                let found = seti.intersection(&setj).copied().collect::<HashSet<_>>();
                if found.len() > 3 {
                    println!("{}, {:?}", found.len(), found);
                    // println!("{:?}", s_i);
                    // println!("{:?}", tbj);
                    println!();
                }
                if found.len() >= 12 {
                    return Some((*m, found));
                }
            }
        }
    }
    None
}

fn part1(sensors: &[ParsedItem]) -> Answer {
    let matrices = make_matrices();
    let mut rots = HashMap::new();
    for i in 0..sensors.len() {
        for j in 0..sensors.len() {
            if i == j {
                continue;
            }
            if let Some((mt, common)) = align(&matrices, &sensors[i], &sensors[j]) {
                println!(
                    "found alignment of {:?}, {:?} with {:?}, common: {:?}",
                    i, j, mt, common
                );
                rots.insert((i, j), mt);
            }
        }
    }
    // Transform back to 0's coordinate system
    // for b in sensors {}
    for mt in rots {
        println!("mt: {:?}", mt);
    }
    0
}

fn part2(_: &[ParsedItem]) -> Answer {
    0
}

fn parse(lines: &[String]) -> Parsed {
    let sections = aoc::split_by_empty_line(lines);
    sections
        .iter()
        .map(|x| {
            x[2..]
                .iter()
                .map(|x| {
                    let p = aoc::split_ch(*x, ',');
                    [
                        p[0].parse::<i64>().unwrap(),
                        p[1].parse::<i64>().unwrap(),
                        p[2].parse::<i64>().unwrap(),
                        1,
                    ]
                })
                .collect()
        })
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
        include_str!("sample.txt")
            .lines()
            .map(|x| x.into())
            .collect()
    }

    #[test]
    fn test_part1() {
        let parsed = parse(&example());
        assert_eq!(part1(&parsed[0..2]), 79);
    }
}
