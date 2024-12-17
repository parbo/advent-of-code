use std::{collections::VecDeque, iter::*};

type Parsed = (i64, i64, i64, Vec<i64>);

fn run(data: &Parsed) -> Result<Vec<i64>, ()> {
    let mut a = data.0;
    let mut b = data.1;
    let mut c = data.2;
    let prog = data.3.clone();
    let mut ip = 0usize;
    let mut out = vec![];
    while ip + 1 < prog.len() {
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
    println!("{:?}", run(data));
    0
}

fn step(a: i64) -> i64 {
    let b = a & 7;
    let b = b ^ 1;
    let c = a >> b;
    let b = b ^ c;
    let b = b ^ 4;
    b & 7
}

fn part2(data: &Parsed) -> i64 {
    let mut todo: VecDeque<(i64, usize)> = (0..=7).map(|x| (x, 0)).collect();
    let mut res = vec![];
    while let Some((a, ix)) = todo.pop_front() {
        let v = step(a);
        if v == data.3[data.3.len() - ix - 1] {
            if ix + 1 == data.3.len() {
                res.push(a);
                continue;
            }
            for i in 0..=7 {
                let a = (a << 3) | i;
                todo.push_back((a, ix + 1));
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
