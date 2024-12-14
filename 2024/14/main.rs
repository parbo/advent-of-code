use aoc::GridDrawer;
use std::iter::*;

type ParsedItem = (aoc::Point, aoc::Point);
type Parsed = Vec<ParsedItem>;

#[allow(clippy::comparison_chain)]
fn solve(data: &Parsed, w: i64, h: i64, n: i64) -> i64 {
    let mut robots = data.clone();
    for (p, v) in &mut robots {
        p[0] = (p[0] + n * v[0]).rem_euclid(w);
        p[1] = (p[1] + n * v[1]).rem_euclid(h);
    }
    let mut q = [0, 0, 0, 0];
    for (p, _v) in &robots {
        if p[0] < w / 2 {
            if p[1] < h / 2 {
                q[0] += 1;
            } else if p[1] > h / 2 {
                q[1] += 1;
            }
        } else if p[0] > w / 2 {
            if p[1] < h / 2 {
                q[2] += 1;
            } else if p[1] > h / 2 {
                q[3] += 1;
            }
        }
    }
    q[0] * q[1] * q[2] * q[3]
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 101, 103, 100)
}

fn part2(data: &Parsed) -> i64 {
    let w = 101;
    let h = 103;
    let n = w * h;
    let mut robots = data.clone();
    let mut gd = aoc::BitmapGridDrawer::new(
        |x: i64| {
            if x == 0 {
                [0, 0, 0]
            } else {
                [0, 0xff, 0]
            }
        },
        "vis/14/day14",
    );
    gd.set_bg([0, 0, 0]);
    let mut gdc = aoc::PrintGridDrawer::new(|x: i64| if x == 0 { ' ' } else { '*' });
    for i in 0..n {
        for (p, v) in &mut robots {
            p[0] = (p[0] + v[0]).rem_euclid(w);
            p[1] = (p[1] + v[1]).rem_euclid(h);
        }
        let mut g = vec![vec![0; w as usize]; h as usize];
        for (p, _v) in &robots {
            g[p[1] as usize][p[0] as usize] = 1;
        }
        let mut numc = 0;
        for r in &g {
            let mut n = 0;
            let mut maxn = 0;
            let mut last = 0;
            for c in r {
                if *c > 0 && *c == last {
                    n += 1;
                    maxn = maxn.max(n)
                } else {
                    n = 0;
                }
                last = *c;
            }
            if maxn > 7 {
                numc += 1;
            }
        }
        if numc > 7 {
            gd.draw(&g);
            gdc.draw(&g);
            return i + 1;
        }
    }
    -1
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let parts = aoc::split_w(x);
            let p = aoc::split_ch(&parts[0][2..], ',');
            let v = aoc::split_ch(&parts[1][2..], ',');
            (
                [p[0].parse().unwrap(), p[1].parse().unwrap()],
                [v[0].parse().unwrap(), v[1].parse().unwrap()],
            )
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

    fn example2() -> Vec<String> {
        vec!["p=2,4 v=2,-3".into()]
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&parse(&example()), 11, 7, 100), 12);
    }
}
