use std::iter::*;

use aoc::Grid;

use bacon_sci::interp::lagrange;

type Parsed = Vec<Vec<char>>;

fn solve(data: &Parsed, steps: i64) -> i64 {
    let p = data
        .points()
        .find(|p| data.get_value(*p) == Some('S'))
        .unwrap();
    let ([min_x, min_y], [max_x, max_y]) = data.extents();
    let h = max_y - min_y + 1;
    let w = max_x - min_x + 1;
    // dbg!(w, h);
    let mut reachable = vec![p];
    let mut y = vec![];
    let x = [w / 2, w / 2 + w, w / 2 + 2 * w];
    for s in 0..=x[2] {
        if s == steps {
            return reachable.len() as i64;
        }
        if x.contains(&s) {
            y.push(reachable.len() as i64);
        }
        let mut seen = aoc::FxHashSet::default();
        let mut new_reachable = vec![];
        for p in &reachable {
            for pp in aoc::neighbors(*p) {
                let ppp = [pp[0].rem_euclid(w), pp[1].rem_euclid(h)];
                let c = data.get_value(ppp);
                if (c == Some('.') || c == Some('S')) && seen.insert(pp) {
                    new_reachable.push(pp);
                }
            }
        }
        reachable = new_reachable;
    }
    let poly = lagrange(
        &[x[0] as f64, x[1] as f64, x[2] as f64],
        &[y[0] as f64, y[1] as f64, y[2] as f64],
        1e-8,
    )
    .unwrap();
    poly.evaluate(steps as f64) as i64
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 64)
}

fn part2(data: &Parsed) -> i64 {
    solve(data, 26501365)
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid(lines)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
