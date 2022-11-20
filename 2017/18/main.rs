use std::{collections::VecDeque, iter::*, str::FromStr};

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
    Snd(Arg),      // snd X plays a sound with a frequency equal to the value of X.
    Set(Arg, Arg), // set X Y sets register X to the value of Y.
    Add(Arg, Arg), // add X Y increases register X by the value of Y.
    Mul(Arg, Arg), // mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
    Mod(Arg, Arg), // mod X Y sets register X to the remainder of dividing the value contained in register X by the value of Y (that is, it sets X to the result of X modulo Y).
    Rcv(Arg), // rcv X recovers the frequency of the last sound played, but only when the value of X is not zero. (If it is zero, the command does nothing.)
    Jgz(Arg, Arg), // jgz X Y jumps with an offset of the value of Y, but only if the value of X is greater than zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
}

impl FromStr for Instruction {
    type Err = aoc::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = aoc::split_w(s);
        match parts[..] {
            ["snd", x] => Ok(Instruction::Snd(x.parse()?)),
            ["set", x, y] => Ok(Instruction::Set(x.parse()?, y.parse()?)),
            ["add", x, y] => Ok(Instruction::Add(x.parse()?, y.parse()?)),
            ["mul", x, y] => Ok(Instruction::Mul(x.parse()?, y.parse()?)),
            ["mod", x, y] => Ok(Instruction::Mod(x.parse()?, y.parse()?)),
            ["rcv", x] => Ok(Instruction::Rcv(x.parse()?)),
            ["jgz", x, y] => Ok(Instruction::Jgz(x.parse()?, y.parse()?)),
            _ => Err(aoc::ParseError::Generic),
        }
    }
}

#[derive(Debug)]
struct Duet {
    program: Vec<Instruction>,
    registers: [i64; 256],
    pc: i64,
    sent: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Ready,
    WaitForRcv,
    Terminated,
}

impl Duet {
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
    fn step(&mut self, snd: &mut VecDeque<i64>, rcv: &mut VecDeque<i64>) -> Outcome {
        if self.pc < 0 || self.pc as usize >= self.program.len() {
            return Outcome::Terminated;
        }
        match self.program[self.pc as usize] {
            Instruction::Snd(x) => {
                snd.push_back(self.value(x));
                self.pc += 1;
                self.sent += 1;
            }
            Instruction::Set(x, y) => {
                let y_val = self.value(y);
                *self.reg(x) = y_val;
                self.pc += 1;
            }
            Instruction::Add(x, y) => {
                let y_val = self.value(y);
                *self.reg(x) += y_val;
                self.pc += 1;
            }
            Instruction::Mul(x, y) => {
                let y_val = self.value(y);
                *self.reg(x) *= y_val;
                self.pc += 1;
            }
            Instruction::Mod(x, y) => {
                let y_val = self.value(y);
                *self.reg(x) %= y_val;
                self.pc += 1;
            }
            Instruction::Rcv(x) => {
                if let Some(v) = rcv.pop_front() {
                    *self.reg(x) = v;
                } else {
                    return Outcome::WaitForRcv;
                }
                self.pc += 1;
            }
            Instruction::Jgz(x, y) => {
                if self.value(x) > 0 {
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
    let mut duet = Duet {
        program: data.to_vec(),
        pc: 0,
        registers: [0; 256],
        sent: 0,
    };
    let mut snd = VecDeque::new();
    let mut rcv = VecDeque::new();
    loop {
        match duet.step(&mut snd, &mut rcv) {
            Outcome::WaitForRcv => break *snd.back().unwrap(),
            Outcome::Terminated => panic!(),
            Outcome::Ready => (),
        }
    }
}

fn part2(data: &Parsed) -> Answer {
    let mut p0 = Duet {
        program: data.to_vec(),
        pc: 0,
        registers: [0; 256],
        sent: 0,
    };
    let mut p1 = Duet {
        program: data.to_vec(),
        pc: 0,
        registers: [0; 256],
        sent: 0,
    };
    p1.registers['p' as usize] = 1;
    let mut q0 = VecDeque::new();
    let mut q1 = VecDeque::new();
    loop {
        while p0.step(&mut q1, &mut q0) == Outcome::Ready {}
        while p1.step(&mut q0, &mut q1) == Outcome::Ready {}
        if q0.is_empty() && q1.is_empty() {
            break;
        }
    }
    p1.sent
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

    fn example() -> Vec<String> {
        vec![
            "snd 1".into(),
            "snd 2".into(),
            "snd p".into(),
            "rcv a".into(),
            "rcv b".into(),
            "rcv c".into(),
            "rcv d".into(),
        ]
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 3);
    }
}
