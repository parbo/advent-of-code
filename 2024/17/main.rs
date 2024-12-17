use std::iter::*;

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
    'outer: for i in 281474976710656..i64::MAX {
        if i % 100000000 == 0 {
            dbg!(i);
        }
        let mut a = i;
        for v in &data.3 {
            let x = ((((a & 7) ^ 1) ^ (a >> 5)) ^ 4) & 7;
            a /= 8;
            if x != *v || a == 0 {
                continue 'outer;
            }
        }
        assert!(a == 0);
        return i;
    }
    0
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
