use std::fmt;
use std::iter::*;


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

fn as_stack(mut c: i64) -> Vec<i64> {
    let mut chrs = vec![];
    loop {
        chrs.push(c % 26);
        if c <= 26 {
            break;
        }
        c /= 26;
    }
    chrs.into_iter().rev().collect()
}

impl fmt::Display for Alu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "w: {}, x: {}, y: {}, z: {}, {:?}",
            self.w,
            self.x,
            self.y,
            self.z,
            as_stack(self.z)
        )
    }
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
}

fn gen_nums(program: &[ParsedItem], ix: usize, digs: &[i64], max: &mut i64, min: &mut i64) {
    if ix == 14 {
        let mut alu = Alu::new();
        let mut m = 0;
        assert!(digs.len() == 14, "{:?}", digs);
        for (i, d) in digs.iter().enumerate() {
            m += *d * 10_i64.pow((13 - i) as u32);
            alu.add_input(*d);
        }
        for p in program {
            alu.step(*p);
        }
        if alu.z == 0 {
            *max = (*max).max(m);
            *min = (*min).min(m);
        }
        return;
    }
    // Reverse engineered program constraints
    let vals = [
        (1, 11, 16, 0),   // 0, push
        (1, 12, 11, 0),   // 1, push
        (1, 13, 12, 0),   // 2, push
        (26, -5, 12, 2),  // 3, pop
        (26, -3, 12, 1),  // 4, pop
        (1, 14, 2, 0),    // 5, push
        (1, 15, 11, 0),   // 6, push
        (26, -16, 4, 6),  // 7, pop
        (1, 14, 12, 0),   // 8, push
        (1, 15, 9, 0),    // 9, push
        (26, -7, 10, 9),  // 10, pop
        (26, -11, 11, 8), // 11, pop
        (26, -6, 6, 5),   // 12, pop
        (26, -11, 15, 0), // 13, pop
    ];
    for r in 1..=9 {
        let mut d = digs.to_owned();
        d.push(r);
        let (_a, b, _c, row) = vals[ix];
        if b < 0 && d[row] + vals[row].2 != d[ix] - b {
            continue;
        }
        gen_nums(program, ix + 1, &d, max, min);
    }
}

fn part1(program: &Parsed) -> Answer {
    let mut max = 0;
    let mut min = 0;
    gen_nums(program, 0, &[], &mut max, &mut min);
    max
}

fn part2(program: &Parsed) -> Answer {
    let mut max = 0;
    let mut min = i64::MAX;
    gen_nums(program, 0, &[], &mut max, &mut min);
    min
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
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
}
