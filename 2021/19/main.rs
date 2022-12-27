use aoc::FxHashSet;
use aoc::Itertools;
use aoc::{Mat4, Vec4};
use std::iter::*;

type ParsedItem = Vec<Vec4>;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn make_matrices() -> Vec<Mat4> {
    let mut matrices = vec![];
    for f in [0, 1, 2, 3, 4, 5, 6, 7] {
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

fn align(
    matrices: &[Mat4],
    seti: &FxHashSet<Vec4>,
    rbjs: &[FxHashSet<Vec4>],
) -> Option<(Mat4, Vec4)> {
    for ix in 0..matrices.len() {
        let m = matrices[ix];
        let rbj = &rbjs[ix];
        for b1 in seti {
            let tot = rbj.len();
            for (i, b2) in rbj.iter().enumerate() {
                let dist = aoc::vec4_sub(*b1, *b2);
                let mut found = 0;
                for b in rbj {
                    let bj = aoc::vec4_add(*b, dist);
                    if seti.contains(&bj) {
                        found += 1;
                    }
                    if found >= 12 {
                        let mut mt = m;
                        mt[0][3] = dist[0];
                        mt[1][3] = dist[1];
                        mt[2][3] = dist[2];
                        return Some((mt, dist));
                    }
                    if tot - i + found < 12 {
                        break;
                    }
                }
            }
        }
    }
    None
}

fn solve(sensors: &[ParsedItem]) -> (FxHashSet<Vec4>, Vec<Vec4>) {
    let matrices = make_matrices();
    let rbjs = sensors
        .iter()
        .map(|x| {
            matrices
                .iter()
                .map(|m| {
                    x.iter()
                        .map(|bj| aoc::mat_transform(*m, *bj))
                        .collect::<FxHashSet<Vec4>>()
                })
                .collect::<Vec<FxHashSet<Vec4>>>()
        })
        .collect::<Vec<Vec<FxHashSet<_>>>>();
    let mut translated = sensors[0].iter().copied().collect::<FxHashSet<_>>();
    translated.reserve(sensors[0].len() * (sensors.len() + 1));
    let mut dists = vec![];
    dists.reserve(sensors.len());
    let mut ti = (1..sensors.len()).collect::<FxHashSet<_>>();
    while !ti.is_empty() {
        let mut remove = vec![];
        for j in &ti {
            if let Some((mt, dist)) = align(&matrices, &translated, &rbjs[*j]) {
                println!("found alignment of {:?} with {:?}, dist: {:?}", j, mt, dist);
                for bj in &sensors[*j] {
                    let tbj = aoc::mat_transform(mt, *bj);
                    translated.insert(tbj);
                }
                dists.push(dist);
                remove.push(*j);
            }
        }
        for i in remove {
            ti.remove(&i);
        }
    }
    println!("{}, {:?}", translated.len(), translated);
    (translated, dists)
}

fn part1(sensors: &Parsed) -> Answer {
    let (combined, _) = solve(sensors);
    combined.len() as Answer
}

fn part2(sensors: &Parsed) -> Answer {
    let (_combined, dists) = solve(sensors);
    dists
        .iter()
        .permutations(2)
        .map(|a| aoc::manhattan_vec4(*a[0], *a[1]))
        .max()
        .unwrap()
}

fn parse(lines: &[String]) -> Parsed {
    let sections = aoc::split_by_empty_line(lines);
    sections
        .iter()
        .map(|x| {
            x[1..]
                .iter()
                .map(|x| {
                    let p = aoc::split_ch(x, ',');
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
    aoc::run_main(parse, part1, part2);
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

    fn example_expected() -> FxHashSet<Vec4> {
        include_str!("sample_expected.txt")
            .lines()
            .map(|x| aoc::split_ch(x, ','))
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
        let (combined, _) = solve(&parsed);
        let expected = example_expected();
        let diff = expected.difference(&combined);
        for d in diff {
            println!("diff: {:?}", d);
        }
        assert_eq!(combined, expected);
        assert_eq!(combined.len(), 79);
    }

    #[test]
    fn test_part2() {
        let parsed = parse(&example());
        assert_eq!(part2(&parsed), 3621);
    }
}
