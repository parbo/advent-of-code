use std::{collections::VecDeque, iter::*};

type Parsed = (i64, i64, i64, Vec<i64>);

fn run(data: &Parsed, alt_a: Option<i64>) -> Result<Vec<i64>, ()> {
    let mut a = alt_a.unwrap_or(data.0);
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
    println!("{:?}", run(data, None));
    0
}

fn part2(data: &Parsed) -> i64 {
    let mut todo: VecDeque<(i64, usize)> = (0..=255).map(|x| (x, 0)).collect();
    let mut seen = aoc::FxHashSet::default();
    let mut res = None;
    let rev: Vec<_> = data.3.iter().copied().rev().collect();
    while let Some((a, ix)) = todo.pop_front() {
        let aa = a >> (ix * 3);
        let x = ((((aa & 7) ^ 1) ^ (aa >> 5)) ^ 4) & 7;
        if x == data.3[ix] {
            // println!("{:#064b}, {:08b}, {}, {}, {}", a, aa, x, ix, data.3[ix]);
            if ix + 1 == data.3.len() {
                res = Some(a);
                break;
            }
            for i in 0..=32 {
                let aaa = (aa & 0x7) | (i << 3);
                let m = 2i64.pow(3 * (ix + 1) as u32) - 1;
                println!("{:#064b}, {:#064b}, {:#08b}", a, m, aaa);
                let a = (a & m) | (aaa << (3 * ix));
                println!("{:#064b}", a);
                if seen.insert((a, ix + 1)) {
                    todo.push_back((a, ix + 1));
                }
            }
        }
    }
    println!("{:?}, {:?}", data.3, run(data, res));
    res.unwrap()
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
