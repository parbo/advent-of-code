use std::{collections::VecDeque, iter::*};

type Parsed = (i64, i64, i64, Vec<i64>);

fn run(data: &Parsed, alt_a: Option<i64>, n: Option<usize>) -> Result<Vec<i64>, ()> {
    let mut a = alt_a.unwrap_or(data.0);
    let mut b = data.1;
    let mut c = data.2;
    let prog = data.3.clone();
    let mut ip = 0usize;
    let mut out = vec![];
    let mut j = 0;
    while ip + 1 < prog.len() && j <= n.unwrap_or(0) {
        // if alt_a.is_some() {
        //     println!("{}: {} {} {}", ip, a, b, c);
        // }
        let op = prog[ip];
        let operand = prog[ip + 1];
        let co = match operand {
            0..=3 => Some(operand),
            4 => Some(a),
            5 => Some(b),
            6 => Some(c),
            _ => None,
        };
        match op {
            0 => {
                a /= 2i64.pow(co.ok_or(())? as u32);
                ip += 2
            }
            1 => {
                b ^= operand;
                ip += 2
            }
            2 => {
                b = co.ok_or(())? & 0x7;
                ip += 2
            }
            3 => {
                if a != 0 {
                    if n.is_some() {
                        j += 1;
                    }
                    ip = operand as usize
                } else {
                    ip += 2
                }
            }
            4 => {
                b ^= c;
                ip += 2;
            }
            5 => {
                out.push(co.ok_or(())? & 0x7);
                ip += 2;
            }
            6 => {
                b = a / (2i64.pow(co.ok_or(())? as u32));
                ip += 2
            }
            7 => {
                c = a / (2i64.pow(co.ok_or(())? as u32));
                ip += 2
            }
            _ => return Err(()),
        }
    }
    Ok(out)
}

fn part1(data: &Parsed) -> i64 {
    println!("{:?}", run(data, None, None));
    0
}

fn part2(data: &Parsed) -> i64 {
    let mut todo: VecDeque<(i64, usize)> = (0..=255).map(|x| (x, 0)).collect();
    let mut seen = aoc::FxHashSet::default();
    let mut res = vec![];
    while let Some((a, ix)) = todo.pop_front() {
        if ix >= data.3.len() {
            continue;
        }
        let r = run(data, Some(a), Some(ix));
        if r.is_err() {
            continue;
        }
        let r = r.unwrap();
        if r.len() <= ix {
            continue;
        }
        if r[..=ix] == data.3[..=ix] {
            if ix + 1 == data.3.len() && (a >> (3 * (ix + 1))) == 0 {
                res.push(a);
            }
            let m = 2i64.pow((3 * ix + 8) as u32) - 1;
            for i in 0..=255 {
                let aaa = i << (8 + (3 * ix));
                let a = (a & m) | aaa;
                if seen.insert((a, ix + 1)) {
                    todo.push_back((a, ix + 1));
                }
            }
        }
    }
    let m = *res.iter().min().unwrap();
    m
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    let regs: Vec<_> = parts[0]
        .iter()
        .map(|x| aoc::split_ch(x, ':')[1].parse().unwrap())
        .collect();
    let prog = aoc::things(aoc::split_ch(parts[1][0], ':')[1]);
    (regs[0], regs[1], regs[2], prog)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
