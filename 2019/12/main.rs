use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::*;

type Parsed = Vec<Vec<i64>>;

fn energy(m: &Parsed, steps: usize) -> i64 {
    let mut moons = m.clone();
    let mut vel = vec![];
    vel.resize(moons.len(), vec![0, 0, 0]);
    for _s in 0..steps {
        for i in 0..3 {
            for a in 0..moons.len() {
                for b in (a + 1)..moons.len() {
                    // Apply gravity
                    match moons[a][i].cmp(&moons[b][i]) {
                        Ordering::Less => {
                            vel[a][i] += 1;
                            vel[b][i] -= 1;
                        }
                        Ordering::Greater => {
                            vel[a][i] -= 1;
                            vel[b][i] += 1;
                        }
                        _ => (),
                    }
                }
                // Apply velocity
                moons[a][i] += vel[a][i];
            }
        }
    }
    let mut e: i64 = 0;
    for a in 0..moons.len() {
        let e1: i64 = moons[a].iter().map(|x| x.abs()).sum();
        let e2: i64 = vel[a].iter().map(|x| x.abs()).sum();
        e += e1 * e2;
    }
    e
}

fn part1(m: &Parsed) -> i64 {
    energy(m, 1000)
}

fn part2(m: &Parsed) -> i64 {
    let mut moons = m.clone();
    let mut vel = vec![];
    vel.resize(moons.len(), vec![0, 0, 0]);
    let mut cycles = vec![];
    for i in 0..3 {
        let mut seen = HashSet::new();
        let mut s = 1;
        let mut state = vec![];
        for a in 0..moons.len() {
            state.push((moons[a][i], vel[a][i]));
        }
        seen.insert(state.clone());
        let c = loop {
            state.clear();
            for a in 0..moons.len() {
                for b in (a + 1)..moons.len() {
                    // Apply gravity
                    match moons[a][i].cmp(&moons[b][i]) {
                        Ordering::Less => {
                            vel[a][i] += 1;
                            vel[b][i] -= 1;
                        }
                        Ordering::Greater => {
                            vel[a][i] -= 1;
                            vel[b][i] += 1;
                        }
                        _ => (),
                    }
                }
                // Apply velocity
                moons[a][i] += vel[a][i];
                state.push((moons[a][i], vel[a][i]));
            }
            if !seen.insert(state.clone()) {
                break s;
            }
            s += 1;
        };
        cycles.push(c);
    }
    aoc::lcm(cycles[0], aoc::lcm(cycles[1], cycles[2]))
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|line| {
            let v: Vec<i64> = aoc::scan!("<x={}, y={}, x={}>" <- line).unwrap();
            v
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::{energy, part2};

    #[test]
    fn test_part1() {
        let m = vec![
            vec![-1, 0, 2],
            vec![2, -10, -7],
            vec![4, -8, 8],
            vec![3, 5, -1],
        ];
        let r = energy(&m, 10);
        assert_eq!(r, 179);
    }

    #[test]
    fn test_part2() {
        let m = vec![
            vec![-8, -10, 0],
            vec![5, 5, 10],
            vec![2, -7, 3],
            vec![9, -8, -3],
        ];
        let r = part2(&m);
        assert_eq!(r, 4686774924);
    }
}
