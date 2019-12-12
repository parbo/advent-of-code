use aoc;
// use intcode;
use num;
use std::collections::HashSet;
use std::iter::*;

fn energy(m: &Vec<Vec<i64>>, steps: usize) -> i64 {
    let mut moons = m.clone();
    let mut vel = vec![];
    vel.resize(moons.len(), vec![0, 0, 0]);
    // for a in 0..moons.len() {
    //     println!("{:?}, {:?}", moons[a], vel[a]);
    // }
    // println!();
    for s in 0..steps {
        for a in 0..moons.len() {
            for b in (a + 1)..moons.len() {
                // Apply gravity
                for i in 0..3 {
                    if moons[a][i] < moons[b][i] {
                        vel[a][i] += 1;
                        vel[b][i] -= 1;
                    } else if moons[a][i] > moons[b][i] {
                        vel[a][i] -= 1;
                        vel[b][i] += 1;
                    }
                }
            }
        }
        for a in 0..moons.len() {
            // Apply velocity
            for i in 0..3 {
                moons[a][i] += vel[a][i];
            }
        }
        // println!("After {} steps", s + 1);
        // for a in 0..moons.len() {
        //     println!("{:?}, {:?}", moons[a], vel[a]);
        // }
        // for a in 0..moons.len() {
        //     let e1 : i64 = moons[a].iter().map(|x| x.abs()).sum();
        //     let e2 : i64 = vel[a].iter().map(|x| x.abs()).sum();
        //     println!("energy: {} * {} = {}", e1, e2, e1 * e2);
        // }
        // println!();
    }
    let mut e: i64 = 0;
    for a in 0..moons.len() {
        let e1: i64 = moons[a].iter().map(|x| x.abs()).sum();
        let e2: i64 = vel[a].iter().map(|x| x.abs()).sum();
        e += e1 * e2;
    }
    e
}

fn part1(m: &Vec<Vec<i64>>) -> i64 {
    energy(m, 1000)
}

fn part2(m: &Vec<Vec<i64>>) -> i64 {
    let mut moons = m.clone();
    let mut vel = vec![];
    vel.resize(moons.len(), vec![0, 0, 0]);
    let mut seen_x = HashSet::new();
    let mut seen_y = HashSet::new();
    let mut seen_z = HashSet::new();
    seen_x.insert((
        (moons[0][0], vel[0][0]),
        (moons[1][0], vel[1][0]),
        (moons[2][0], vel[2][0]),
    ));
    seen_y.insert((
        (moons[0][1], vel[0][1]),
        (moons[1][1], vel[1][1]),
        (moons[2][1], vel[2][1]),
    ));
    seen_z.insert((
        (moons[0][2], vel[0][2]),
        (moons[1][2], vel[1][2]),
        (moons[2][2], vel[2][2]),
    ));
    let mut f_x = 0;
    let mut f_y = 0;
    let mut f_z = 0;
    let mut s = 0;
    loop {
        for a in 0..moons.len() {
            for b in (a + 1)..moons.len() {
                // Apply gravity
                for i in 0..3 {
                    if moons[a][i] < moons[b][i] {
                        vel[a][i] += 1;
                        vel[b][i] -= 1;
                    } else if moons[a][i] > moons[b][i] {
                        vel[a][i] -= 1;
                        vel[b][i] += 1;
                    }
                }
            }
        }
        for a in 0..moons.len() {
            // Apply velocity
            for i in 0..3 {
                moons[a][i] += vel[a][i];
            }
        }
        // println!("After {} steps", s + 1);
        // for a in 0..moons.len() {
        //     println!("{:?}, {:?}", moons[a], vel[a]);
        // }
        s += 1;
        if f_x == 0
            && !seen_x.insert((
                (moons[0][0], vel[0][0]),
                (moons[1][0], vel[1][0]),
                (moons[2][0], vel[2][0]),
            ))
        {
            f_x = s;
        }
        if f_y == 0
            && !seen_y.insert((
                (moons[0][1], vel[0][1]),
                (moons[1][1], vel[1][1]),
                (moons[2][1], vel[2][1]),
            ))
        {
            f_y = s;
        }
        if f_z == 0
            && !seen_z.insert((
                (moons[0][2], vel[0][2]),
                (moons[1][2], vel[1][2]),
                (moons[2][2], vel[2][2]),
            ))
        {
            f_z = s;
        }
        if f_x != 0 && f_y != 0 && f_z != 0 {
            break;
        }
    }

    let mut un = HashSet::new();
    un.insert(num::integer::gcd(f_x, f_y));
    un.insert(num::integer::gcd(f_x, f_z));
    un.insert(num::integer::gcd(f_y, f_z));

    let mut v = f_x * f_y * f_z;

    for u in un {
        v /= u;
    }
    v
}

fn parse(lines: &Vec<String>) -> Vec<Vec<i64>> {
    let mut res = vec![];
    for line in lines {
        let mut v = vec![];
        let pos: Vec<_> = line[1..(line.len() - 1)].split(",").collect();
        for p in pos {
            let c: Vec<&str> = p.split("=").skip(1).take(1).map(|s| s.trim()).collect();
            let cv = c[0].parse::<i64>().unwrap();
            v.push(cv);
        }
        res.push(v);
    }
    res
}

fn main() {
    let (part, lines) = aoc::read_lines();
    //let parsed = aoc::parse_intcode(&lines);
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
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
