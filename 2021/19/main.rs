use aoc::Itertools;
use aoc::{Mat4, Vec4};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::iter::*;
use std::time::Instant;

type ParsedItem = Vec<Vec4>;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn make_matrices() -> Vec<Mat4> {
    let mut matrices = vec![];
    for f in [0, 1, 2, 3, 4, 5, 6, 7, 8] {
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    if x == y || x == z || y == z {
                        continue;
                    }
                    let rows = [
                        [
                            if f == 1 || f == 4 || f == 5 || f == 7 {
                                -1
                            } else {
                                1
                            },
                            0,
                            0,
                            0,
                        ],
                        [
                            0,
                            if f == 2 || f == 4 || f == 6 || f == 7 {
                                -1
                            } else {
                                1
                            },
                            0,
                            0,
                        ],
                        [
                            0,
                            0,
                            if f == 3 || f == 5 || f == 6 || f == 7 {
                                -1
                            } else {
                                1
                            },
                            0,
                        ],
                    ];
                    let m = [rows[x], rows[y], rows[z], [0, 0, 0, 1]];
                    matrices.push(m);
                }
            }
        }
    }
    matrices
}

fn align(matrices: &[Mat4], s_i: &[Vec4], s_j: &[Vec4]) -> Option<(Mat4, Vec4, BTreeSet<Vec4>)> {
    let seti = s_i.iter().copied().collect::<BTreeSet<_>>();
    for m in matrices {
        // Rotate beacons
        let rbj = s_j
            .iter()
            .map(|bj| aoc::mat_transform(*m, *bj))
            .collect::<Vec<_>>();
        for b1 in s_i {
            for b2 in &rbj {
                let dist = aoc::vec4_sub(*b1, *b2);
                let tbj = rbj
                    .iter()
                    .map(|b| aoc::vec4_add(*b, dist))
                    .collect::<Vec<_>>();
                let setj = tbj.iter().copied().collect::<BTreeSet<_>>();
                let found = seti.intersection(&setj).copied().collect::<BTreeSet<_>>();
                if found.len() >= 12 {
                    return Some((*m, dist, found));
                }
            }
        }
    }
    None
}

fn transform(
    t: &BTreeMap<(usize, usize), (Mat4, Vec4, BTreeSet<Vec4>)>,
    i: usize,
    c: &BTreeSet<Vec4>,
    v: &mut BTreeSet<usize>,
    d: usize,
) -> Option<BTreeSet<Vec4>> {
    println!("t: {}, {:?}", i, c);
    if v.contains(&i) {
        return None;
    }
    v.insert(i);
    if i == 0 {
        println!("i == 0");
        return Some(c.clone());
    }
    for ((k, l), (m, dist, _c)) in t {
        println!("k: {}, l: {}, d: {:?}", k, l, dist);
        if *k == i {
            println!("alt 1");
            let new_c = c
                .iter()
                .map(|x| aoc::mat_transform(*m, aoc::vec4_sub(*x, *dist)))
                .collect::<BTreeSet<_>>();
            if let Some(c) = transform(t, *l, &new_c, v, d + 1) {
                return Some(c);
            }
        } else if *l == i {
            println!("alt 2");
            let new_c = c
                .iter()
                .map(|x| aoc::vec4_add(aoc::mat_transform(*m, *x), *dist))
                .collect::<BTreeSet<_>>();
            if let Some(c) = transform(t, *k, &new_c, v, d + 1) {
                return Some(c);
            }
        }
    }
    println!("failed!");
    None
}

fn solve(sensors: &[ParsedItem]) -> BTreeSet<Vec4> {
    let matrices = make_matrices();
    let mut rots = BTreeMap::new();
    for i in 0..sensors.len() {
        for j in 0..sensors.len() {
            if i == j {
                continue;
            }
            if let Some((mt, dist, common)) = align(&matrices, &sensors[i], &sensors[j]) {
                println!(
                    "found alignment of {:?}, {:?} with {:?}, dist: {:?}, common: {:?}",
                    i, j, mt, dist, common
                );
                rots.insert((i, j), (mt, dist, common));
            }
        }
    }
    // Transform back to 0's coordinate system
    let mut combined = BTreeSet::<Vec4>::new();
    for (i, bi) in sensors.iter().enumerate() {
	let s = bi.iter().copied().collect::<BTreeSet<_>>();
        let mut v = BTreeSet::new();
        if let Some(c) = transform(&rots, i, &s, &mut v, 0) {
            println!("transformed: {}, {:?}", i, c);
            combined.extend(&c);
        }
    }
    println!("combined: {}, {:?}", combined.len(), combined.iter().sorted().collect::<Vec<_>>());
    combined
}
fn part1(sensors: &[ParsedItem]) -> Answer {
    let combined = solve(sensors);
    combined.len() as Answer
}

fn part2(_: &[ParsedItem]) -> Answer {
    0
}

fn parse(lines: &[String]) -> Parsed {
    let sections = aoc::split_by_empty_line(lines);
    sections
        .iter()
        .map(|x| {
            x[1..]
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

    fn example_expected() -> BTreeSet<Vec4> {
        include_str!("sample_expected.txt")
            .lines()
            .map(|x| 
                 aoc::split_ch(x, ','))
            .map(|x| {
                    [
                        x[0].parse::<i64>().unwrap(),
                        x[1].parse::<i64>().unwrap(),
                        x[2].parse::<i64>().unwrap(),
                        1,
                    ]
                })
            .collect()
    }

    #[test]
    fn test_part1() {
        let parsed = parse(&example());
	let combined = solve(&parsed);
	let expected = example_expected();
	let diff = expected.difference(&combined);
	for d in diff {
	    println!("diff: {:?}", d);
	}
	assert_eq!(combined, expected);
        assert_eq!(combined.len(), 79);
    }
}
