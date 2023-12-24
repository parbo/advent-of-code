use std::iter::*;

use aoc::{point_dot, point_sub, vec_add, Itertools, PointBig, Vec3};

type ParsedItem = (Vec3, Vec3);
type Parsed = Vec<ParsedItem>;

fn intersect_lines(a1: PointBig, a2: PointBig, a3: PointBig, a4: PointBig) -> Option<PointBig> {
    let denom = (a1[0] - a2[0]) * (a3[1] - a4[1]) - (a1[1] - a2[1]) * (a3[0] - a4[0]);
    if denom == 0 {
        None
    } else {
        let px = ((a1[0] * a2[1] - a1[1] * a2[0]) * (a3[0] - a4[0])
            - (a1[0] - a2[0]) * (a3[0] * a4[1] - a3[1] * a4[0]))
            / denom;
        let py = ((a1[0] * a2[1] - a1[1] * a2[0]) * (a3[1] - a4[1])
            - (a1[1] - a2[1]) * (a3[0] * a4[1] - a3[1] * a4[0]))
            / denom;
        Some([px, py])
    }
}

fn solve(data: &Parsed, min: i128, max: i128) -> i64 {
    data.iter()
        .combinations(2)
        .filter_map(|x| {
            let p1 = x[0].0;
            let v1 = x[0].1;
            let a1 = [p1[0] as i128, p1[1] as i128];
            let pv1 = vec_add(p1, v1);
            let a2 = [pv1[0] as i128, pv1[1] as i128];
            let p2 = x[1].0;
            let v2 = x[1].1;
            let a3 = [p2[0] as i128, p2[1] as i128];
            let pv2 = vec_add(p2, v2);
            let a4 = [pv2[0] as i128, pv2[1] as i128];
            intersect_lines(a1, a2, a3, a4)
                .map(|x| (a1, point_sub(a2, a1), a3, point_sub(a4, a3), x))
        })
        .filter(|(a1, l1, a3, l2, p)| {
            let a = point_dot(point_sub(*p, *a1), *l1);
            let b = point_dot(point_sub(*p, *a3), *l2);
            // dbg!(a1, a3, p, a, b);
            a > 0 && b > 0
        })
        .filter(|(_, _, _, _, p)| p[0] >= min && p[0] <= max && p[1] >= min && p[1] <= max)
        .count() as i64
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 200000000000000, 400000000000000)
}

fn part2(_: &Parsed) -> i64 {
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let (p, v) = x.split_once('@').unwrap();
            let p = aoc::things(p);
            let v = aoc::things(v);
            ([p[0], p[1], p[2]], [v[0], v[1], v[2]])
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
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&parse(&example()), 7, 27), 2);
    }
}
