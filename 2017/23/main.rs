use std::{iter::*, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Arg {
    Register(u8),
    Number(i64),
}

impl FromStr for Arg {
    type Err = aoc::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(c) = s.parse::<i64>() {
            Ok(Arg::Number(c))
        } else if s.len() == 1 {
            let c = s.chars().next().unwrap();
            if c.is_ascii_alphabetic() {
                Ok(Arg::Register(c as u8))
            } else {
                Err(aoc::ParseError::Generic)
            }
        } else {
            Err(aoc::ParseError::Generic)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Set(Arg, Arg), // set X Y sets register X to the value of Y.
    Sub(Arg, Arg), // sub X Y decreases register X by the value of Y.
    Mul(Arg, Arg), // mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
    Jnz(Arg, Arg), // jnz X Y jumps with an offset of the value of Y, but only if the value of X is not zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
}

impl FromStr for Instruction {
    type Err = aoc::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = aoc::split_w(s);
        match parts[..] {
            ["set", x, y] => Ok(Instruction::Set(x.parse()?, y.parse()?)),
            ["sub", x, y] => Ok(Instruction::Sub(x.parse()?, y.parse()?)),
            ["mul", x, y] => Ok(Instruction::Mul(x.parse()?, y.parse()?)),
            ["jnz", x, y] => Ok(Instruction::Jnz(x.parse()?, y.parse()?)),
            _ => Err(aoc::ParseError::Generic),
        }
    }
}

#[derive(Debug)]
struct Cpu {
    program: Vec<Instruction>,
    registers: [i64; 256],
    pc: i64,
    muls: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Ready,
    Terminated,
}

impl Cpu {
    fn value(&self, arg: Arg) -> i64 {
        match arg {
            Arg::Register(c) => self.registers[c as usize],
            Arg::Number(n) => n,
        }
    }
    fn reg(&mut self, arg: Arg) -> &mut i64 {
        match arg {
            Arg::Register(c) => self.registers.get_mut(c as usize).unwrap(),
            _ => panic!(),
        }
    }
    fn step(&mut self) -> Outcome {
        if self.pc < 0 || self.pc as usize >= self.program.len() {
            return Outcome::Terminated;
        }
        match self.program[self.pc as usize] {
            Instruction::Set(x, y) => {
                let y_val = self.value(y);
                *self.reg(x) = y_val;
                self.pc += 1;
            }
            Instruction::Sub(x, y) => {
                let y_val = self.value(y);
                *self.reg(x) -= y_val;
                self.pc += 1;
            }
            Instruction::Mul(x, y) => {
                let y_val = self.value(y);
                *self.reg(x) *= y_val;
                self.pc += 1;
                self.muls += 1;
            }
            Instruction::Jnz(x, y) => {
                if self.value(x) != 0 {
                    self.pc += self.value(y);
                } else {
                    self.pc += 1;
                }
            }
        }
        Outcome::Ready
    }
}

type ParsedItem = Instruction;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    let mut cpu = Cpu {
        program: data.to_vec(),
        pc: 0,
        registers: [0; 256],
        muls: 0,
    };
    loop {
        match cpu.step() {
            Outcome::Terminated => break cpu.muls,
            Outcome::Ready => (),
        }
    }
}

fn part2(_data: &Parsed) -> Answer {
    let mut b = 106500;
    let c = 123500;
    let mut h = 0;

    loop {
        let mut prime = true;
        for i in 2..b {
            if b % i == 0 {
                prime = false;
                break;
            }
        }
        if !prime {
            h += 1;
        }
        if b == c {
            break;
        }
        b += 17;
    }
    h
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
