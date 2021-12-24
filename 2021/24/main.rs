use std::iter::*;
use std::time::Instant;

#[derive(
    parse_display::Display, parse_display::FromStr, Debug, Copy, Clone, PartialEq, Eq, Hash,
)]
#[display(style = "lowercase")]
enum Var {
    W,
    X,
    Y,
    Z,
    #[display("{0}")]
    Val(i64),
}

#[derive(
    parse_display::Display, parse_display::FromStr, Debug, Copy, Clone, PartialEq, Eq, Hash,
)]
#[display(style = "lowercase")]
enum Ops {
    #[display("inp {0}")]
    Inp(Var),
    #[display("add {0} {1}")]
    Add(Var, Var),
    #[display("mul {0} {1}")]
    Mul(Var, Var),
    #[display("div {0} {1}")]
    Div(Var, Var),
    #[display("mod {0} {1}")]
    Mod(Var, Var),
    #[display("eql {0} {1}")]
    Eql(Var, Var),
}

type ParsedItem = Ops;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

#[derive(Debug)]
struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    curr_input: usize,
    inputs: Vec<i64>,
}

impl Alu {
    pub fn new() -> Alu {
        Alu {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            curr_input: 0,
            inputs: vec![],
        }
    }

    pub fn step(&mut self, op: Ops) {
        match op {
            Ops::Inp(a) => {
                if let Some(i) = self.input() {
                    self.store(a, i)
                } else {
                    panic!();
                }
            }
            Ops::Add(a, b) => {
                let av = self.read(a);
                let bv = self.read(b);
                self.store(a, av + bv);
            }
            Ops::Mul(a, b) => {
                let av = self.read(a);
                let bv = self.read(b);
                self.store(a, av * bv);
            }
            Ops::Div(a, b) => {
                let av = self.read(a);
                let bv = self.read(b);
                self.store(a, av / bv);
            }
            Ops::Mod(a, b) => {
                let av = self.read(a);
                let bv = self.read(b);
                self.store(a, av % bv);
            }
            Ops::Eql(a, b) => {
                let av = self.read(a);
                let bv = self.read(b);
                self.store(a, if av == bv { 1 } else { 0 });
            }
        }
    }

    pub fn store(&mut self, x: Var, v: i64) {
        match x {
            Var::W => self.w = v,
            Var::X => self.x = v,
            Var::Y => self.y = v,
            Var::Z => self.z = v,
            Var::Val(_) => panic!(),
        }
    }

    pub fn read(&self, x: Var) -> i64 {
        match x {
            Var::W => self.w,
            Var::X => self.x,
            Var::Y => self.y,
            Var::Z => self.z,
            Var::Val(v) => v,
        }
    }

    fn input(&mut self) -> Option<i64> {
	println!("read input");
        if self.curr_input < self.inputs.len() {
            let res = self.inputs[self.curr_input];
            self.curr_input += 1;
            Some(res)
        } else {
            None
        }
    }

    pub fn add_input(&mut self, input: i64) {
        self.inputs.push(input);
    }

    // pub fn add_inputs(&mut self, inputs: &[i64]) {
    //     self.inputs.extend(inputs);
    // }

    // pub fn input_len(&self) -> usize {
    // 	self.inputs.len() - self.curr_input
    // }
}

fn check_monad_alu(m: i64) -> (i64, i64, i64, i64) {
    let input = include_str!("input.txt")
        .lines()
        .map(|x| x.into())
        .collect::<Vec<_>>();
    let program = parse(&input);
    let mut alu = Alu::new();
    let mut n = m;
    for i in (0..14).rev() {
        let id = n / 10_i64.pow(i);
        if id == 0 {
            panic!();
        }
        alu.add_input(id);
        n /= 10;
    }
    for p in program {
        alu.step(p);
	if let Ops::Inp(_) = p {
	    println!("alu: {:?}", alu);
	}
    }
    (alu.w, alu.x, alu.y, alu.z)
}

fn check_monad_reversed(m: i64) -> (i64, i64, i64, i64) {
    let mut alu = Alu::new();
    let mut n = m;
    let vals = [
        (1, 11, 16),
        (1, 12, 11),
        (1, 13, 12),
        (26, -5, 12),
        (26, -3, 12),
        (1, 14, 2),
        (1, 15, 11),
        (26, -16, 4),
        (1, 14, 12),
        (1, 15, 9),
        (26, -7, 10),
        (26, -11, 11),
        (26, -6, 6),
        (26, -11, 15),
    ];
    for i in (0..14).rev() {
        let id = n / 10_i64.pow(i);
        if id == 0 {
            panic!();
        }
        alu.w = id;
        alu.x = alu.z % 26;
        alu.z /= vals[i as usize].0;
        alu.x += vals[i as usize].1;
        alu.x = (alu.x != alu.w) as i64;
        alu.z *= 25 * alu.x + 1;
	if alu.x != alu.w {
            alu.y = alu.w + vals[i as usize].2;
	}
        alu.z += alu.y;
	println!("rev: {:?}", alu);
        n /= 10;
    }
    (alu.w, alu.x, alu.y, alu.z)
}

fn part1(program: &[ParsedItem]) -> Answer {
    let mut max = 0;
    // 'outer: for m in 11111111111111..=99999999999999 {
    // 	let mut alu = Alu::new();
    // 	let mut n = m;
    // 	for i in (0..14).rev() {
    // 	    let id = n / 10_i64.pow(i);
    // 	    if id == 0 {
    // 		continue 'outer;
    // 	    }
    // 	    alu.add_input(id);
    // 	    n /= 10;
    // 	}
    // 	for p in program {
    // 	    alu.step(*p);
    // 	}
    // 	if alu.z == 0 {
    // 	    println!("{} is valid", m);
    // 	    max = max.max(m);
    // 	}
    // }

    'outer: for m in 11111111111111..=99999999999999 {
        let mut alu = Alu::new();
        let mut n = m;
        let vals = [
            (1, 11, 16),
            (1, 12, 11),
            (1, 13, 12),
            (26, -5, 12),
            (26, -3, 12),
            (1, 14, 2),
            (1, 15, 11),
            (26, -16, 4),
            (1, 14, 12),
            (1, 15, 9),
            (26, -7, 10),
            (26, -11, 11),
            (26, -6, 6),
            (26, -11, 15),
        ];
        for i in (0..14).rev() {
            let id = n / 10_i64.pow(i);
            if id == 0 {
                continue 'outer;
            }
            alu.w = id;
            alu.x = alu.z % 26;
            alu.z /= vals[i as usize].0;
            alu.x += vals[i as usize].1;
            alu.x = (alu.x != alu.w) as i64;
            alu.y = 25 * alu.x + 1;
            alu.z *= alu.y;
            alu.y = alu.x * (alu.w + vals[i as usize].2);
            alu.z += alu.y;
            n /= 10;
        }
        if alu.z == 0 {
            println!("{} is valid", m);
            max = max.max(m);
        }
    }
    max
}

fn part2(_: &[ParsedItem]) -> Answer {
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let start_time = Instant::now();
    let (part, lines) = aoc::read_lines();
    let io_time = Instant::now();
    let parsed = parse(&lines);
    let parse_time = Instant::now();
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    let done_time = Instant::now();
    println!(
        "read: {:?}, parse: {:?}, solve: {:?}\n",
        io_time.duration_since(start_time),
        parse_time.duration_since(io_time),
        done_time.duration_since(parse_time)
    );
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prog_1() {
        let prog_str = vec!["inp x".to_string(), "mul x -1".to_string()];
        let prog = parse(&prog_str);
        let mut alu = Alu::new();
        alu.add_input(7);
        for p in prog {
            alu.step(p);
        }
        assert_eq!(alu.w, 0);
        assert_eq!(alu.x, -7);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.z, 0);
    }

    #[test]
    fn test_prog_2() {
        let prog_str = vec![
            "inp z".to_string(),
            "inp x".to_string(),
            "mul z 3".to_string(),
            "eql z x".to_string(),
        ];
        let prog = parse(&prog_str);
        for (a, b) in [(10, 30), (10, 31)] {
            let mut alu = Alu::new();
            alu.add_input(a);
            alu.add_input(b);
            for p in &prog {
                alu.step(*p);
            }
            assert_eq!(alu.z, (b == 3 * a) as i64);
        }
    }

    #[test]
    fn test_prog_3() {
        let prog_str = vec![
            "inp w".to_string(),
            "add z w".to_string(),
            "mod z 2".to_string(),
            "div w 2".to_string(),
            "add y w".to_string(),
            "mod y 2".to_string(),
            "div w 2".to_string(),
            "add x w".to_string(),
            "mod x 2".to_string(),
            "div w 2".to_string(),
            "mod w 2".to_string(),
        ];
        let prog = parse(&prog_str);
        for a in 0..20 {
            let mut alu = Alu::new();
            alu.add_input(a);
            for p in &prog {
                alu.step(*p);
            }
            assert_eq!(alu.w, (a & 0x8) >> 3);
            assert_eq!(alu.x, (a & 0x4) >> 2);
            assert_eq!(alu.y, (a & 0x2) >> 1);
            assert_eq!(alu.z, a & 0x1);
        }
    }

    #[test]
    fn test_reverse() {
        assert_eq!(
            check_monad_alu(13579246899999),
            check_monad_reversed(13579246899999)
        );
    }
}
