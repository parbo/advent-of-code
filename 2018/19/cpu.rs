use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;
use std::error::Error as StdError;
use std::fmt;

/**
Addition:

    addr (add register) stores into register C the result of adding register A and register B.
    addi (add immediate) stores into register C the result of adding register A and value B.

Multiplication:

    mulr (multiply register) stores into register C the result of multiplying register A and register B.
    muli (multiply immediate) stores into register C the result of multiplying register A and value B.

Bitwise AND:

    banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
    bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.

Bitwise OR:

    borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
    bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.

Assignment:

    setr (set register) copies the contents of register A into register C. (Input B is ignored.)
    seti (set immediate) stores value A into register C. (Input B is ignored.)

Greater-than testing:

    gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.

Equality testing:

    eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
**/

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Ops {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr
}

#[derive(Debug)]
enum MachineError {
    RegError,
}

impl fmt::Display for MachineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return f.write_str(self.description());
    }
}

impl StdError for MachineError {
    fn description(&self) -> &str {
        match *self {
            MachineError::RegError => "RegError",
        }
    }
}

#[derive(Debug)]
struct Machine {
    regs: [i64;6],
    pc_reg: i64
}

impl Machine {
    fn new(pc_reg: i64, reg_0: i64) -> Machine {
        Machine { regs: [reg_0, 0, 0, 0, 0, 0], pc_reg: pc_reg }
    }

    fn set_r(&mut self, r: i64, v: i64) -> Result<(), MachineError> {
        if r >= 0 && r < 6 {
            self.regs[r as usize] = v;
            Ok(())
        } else {
            Err(MachineError::RegError)
        }
    }

    fn get_r(&self, r: i64) -> Result<i64, MachineError> {
        if r >= 0 && r < 6 {
            Ok(self.regs[r as usize])
        } else {
            Err(MachineError::RegError)
        }
    }

    fn execute(&mut self, op: &Ops, a: i64, b: i64, c: i64) -> Result<(i64), MachineError> {
        match op {
            Ops::Addr => {
                let res = self.get_r(a)? + self.get_r(b)?;
                self.set_r(c, res)?;
            },
            Ops::Addi => {
                let res = self.get_r(a)? + b;
                self.set_r(c, res)?;
            },
            Ops::Mulr => {
                let res = self.get_r(a)? * self.get_r(b)?;
                self.set_r(c, res)?;
            },
            Ops::Muli => {
                let res = self.get_r(a)? * b;
                self.set_r(c, res)?;
            },
            Ops::Banr => {
                let res = self.get_r(a)? & self.get_r(b)?;
                self.set_r(c, res)?;
            },
            Ops::Bani => {
                let res = self.get_r(a)? & b;
                self.set_r(c, res)?;
            },
            Ops::Borr => {
                let res = self.get_r(a)? | self.get_r(b)?;
                self.set_r(c, res)?;
            },
            Ops::Bori => {
                let res = self.get_r(a)? | b;
                self.set_r(c, res)?;
            },
            Ops::Setr => {
                let res = self.get_r(a)?;
                self.set_r(c, res)?;
            },
            Ops::Seti => {
                let res = a;
                self.set_r(c, res)?;
            },
            Ops::Gtir => {
                let res = if a > self.get_r(b)? { 1 } else { 0 };
                self.set_r(c, res)?;
            },
            Ops::Gtri => {
                let res = if self.get_r(a)? > b { 1 } else { 0 };
                self.set_r(c, res)?;
            },
            Ops::Gtrr => {
                let res = if self.get_r(a)? > self.get_r(b)? { 1 } else { 0 };
                self.set_r(c, res)?;
            },
            Ops::Eqir => {
                let res = if a == self.get_r(b)? { 1 } else { 0 };
                self.set_r(c, res)?;
            },
            Ops::Eqri => {
                let res = if self.get_r(a)? == b { 1 } else { 0 };
                self.set_r(c, res)?;
            },
            Ops::Eqrr => {
                let res = if self.get_r(a)? == self.get_r(b)? { 1 } else { 0 };
                self.set_r(c, res)?;
            },
        }
        self.regs[self.pc_reg as usize] += 1;
        Ok(self.regs[self.pc_reg as usize])
    }
}

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut program = vec![];
    let mut pc_reg = 0;
    let insmap : HashMap<&str, Ops> = [("addr", Ops::Addr),
                                       ("addi", Ops::Addi),
                                       ("mulr", Ops::Mulr),
                                       ("muli", Ops::Muli),
                                       ("banr", Ops::Banr),
                                       ("bani", Ops::Bani),
                                       ("borr", Ops::Borr),
                                       ("bori", Ops::Bori),
                                       ("setr", Ops::Setr),
                                       ("seti", Ops::Seti),
                                       ("gtir", Ops::Gtir),
                                       ("gtri", Ops::Gtri),
                                       ("gtrr", Ops::Gtrr),
                                       ("eqir", Ops::Eqir),
                                       ("eqri", Ops::Eqri),
                                       ("eqrr", Ops::Eqrr)].iter().cloned().collect();
    for line in lines {
        if line[0..3] == *"#ip" {
            pc_reg = line[4..].parse::<i64>().unwrap();
        } else {
            let op = *insmap.get(&line[0..4]).unwrap();
            let args : Vec<i64> = line[5..].split(|c| c == ' ').map(|s| s.trim().parse::<i64>().unwrap()).collect();
            program.push((op, args));
        }
    }

    for r0 in &[0, 1] {
        println!("problem: {}", r0);
        println!("-----------");
        let mut m = Machine::new(pc_reg, *r0);
        let illegal = program.len() as i64;
        let mut i = 0;
        let mut pc = 0;
        loop {
            let (op, args) = &program[pc];
            i += 1;
            match m.execute(&op, args[0], args[1], args[2]) {
                Ok(new_pc) => {
                    if new_pc < 0 || new_pc >= illegal {
                        break;
                    }
                    pc = new_pc as usize;
                },
                _ => panic!()
            }
        }
        println!("{}, {:?}", i, m);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
