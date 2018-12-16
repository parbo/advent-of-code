use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
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
    regs: [i64;4],
    pc: i64
}

impl Machine {
    fn new() -> Machine {
        Machine { regs: [0, 0, 0, 0], pc: 0 }
    }

    fn set_r(&mut self, r: i64, v: i64) -> Result<(), MachineError> {
        if r >= 0 && r < 4 {
            self.regs[r as usize] = v;
            Ok(())
        } else {
            Err(MachineError::RegError)
        }
    }

    fn get_r(&self, r: i64) -> Result<i64, MachineError> {
        if r >= 0 && r < 4 {
            Ok(self.regs[r as usize])
        } else {
            Err(MachineError::RegError)
        }
    }

    fn execute(&mut self, op: &Ops, a: i64, b: i64, c: i64) -> Result<(), MachineError> {
        match op {
            Ops::Addr => {
                let res = self.get_r(a)? + self.get_r(b)?;
                self.set_r(c, res)
            },
            Ops::Addi => {
                let res = self.get_r(a)? + b;
                self.set_r(c, res)
            },
            Ops::Mulr => {
                let res = self.get_r(a)? * self.get_r(b)?;
                self.set_r(c, res)
            },
            Ops::Muli => {
                let res = self.get_r(a)? * b;
                self.set_r(c, res)
            },
            Ops::Banr => {
                let res = self.get_r(a)? & self.get_r(b)?;
                self.set_r(c, res)
            },
            Ops::Bani => {
                let res = self.get_r(a)? & b;
                self.set_r(c, res)
            },
            Ops::Borr => {
                let res = self.get_r(a)? | self.get_r(b)?;
                self.set_r(c, res)
            },
            Ops::Bori => {
                let res = self.get_r(a)? | b;
                self.set_r(c, res)
            },
            Ops::Setr => {
                let res = self.get_r(a)?;
                self.set_r(c, res)
            },
            Ops::Seti => {
                let res = a;
                self.set_r(c, res)
            },
            Ops::Gtir => {
                let res = if a > self.get_r(b)? { 1 } else { 0 };
                self.set_r(c, res)
            },
            Ops::Gtri => {
                let res = if self.get_r(a)? > b { 1 } else { 0 };
                self.set_r(c, res)
            },
            Ops::Gtrr => {
                let res = if self.get_r(a)? > self.get_r(b)? { 1 } else { 0 };
                self.set_r(c, res)
            },
            Ops::Eqir => {
                let res = if a == self.get_r(b)? { 1 } else { 0 };
                self.set_r(c, res)
            },
            Ops::Eqri => {
                let res = if self.get_r(a)? == b { 1 } else { 0 };
                self.set_r(c, res)
            },
            Ops::Eqrr => {
                let res = if self.get_r(a)? == self.get_r(b)? { 1 } else { 0 };
                self.set_r(c, res)
            },
        }
    }
}

fn reject(c: &HashMap<i64, Ops>, runs: &Vec<(Vec<i64>, Vec<i64>, Vec<i64>)>) -> Option<HashMap<i64, Ops>> {
    let mut unmatched : HashSet<&Ops> = HashSet::from_iter(&[Ops::Addr,
                                                             Ops::Addi,
                                                             Ops::Mulr,
                                                             Ops::Muli,
                                                             Ops::Banr,
                                                             Ops::Bani,
                                                             Ops::Borr,
                                                             Ops::Bori,
                                                             Ops::Setr,
                                                             Ops::Seti,
                                                             Ops::Gtir,
                                                             Ops::Gtri,
                                                             Ops::Gtrr,
                                                             Ops::Eqir,
                                                             Ops::Eqri,
                                                             Ops::Eqrr]);
    for (i, op) in c {
        unmatched.remove(&op);
    }
    let mut matches = HashMap::new();
    for (ins, bef, aft) in runs {
        if let Some(op) = c.get(&ins[0]) {
            let mut m = Machine::new();
            m.regs[0] =  bef[0];
            m.regs[1] =  bef[1];
            m.regs[2] =  bef[2];
            m.regs[3] =  bef[3];
            if let Ok(_) = m.execute(&op, ins[1], ins[2], ins[3]) {
                if m.regs[0] == aft[0] && m.regs[1] == aft[1] && m.regs[2] == aft[2] && m.regs[3] == aft[3] {
                } else {
                    // reject if the locked ones doä't match
                    return None;
                }
            }
        } else {
            let mut any_match = false;
            for op in &unmatched {
                let mut m = Machine::new();
                m.regs[0] =  bef[0];
                m.regs[1] =  bef[1];
                m.regs[2] =  bef[2];
                m.regs[3] =  bef[3];
                if let Ok(_) = m.execute(&op, ins[1], ins[2], ins[3]) {
                    if m.regs[0] == aft[0] && m.regs[1] == aft[1] && m.regs[2] == aft[2] && m.regs[3] == aft[3] {
                        matches.entry(ins[0]).or_insert(HashSet::new()).insert(*op);
                        any_match = true;
                    }
                }
            }
            if !any_match {
                return None;
            }
        }
    }
    let mut unambiguous : HashMap<i64, Ops> = HashMap::new();
    for (opix, ops) in matches {
        if ops.len() == 1 {
            unambiguous.insert(opix, *ops.iter().cloned().next().unwrap());
        }
    }
    return Some(unambiguous);
}

fn accept(c: &HashMap<i64, Ops>, runs: &Vec<(Vec<i64>, Vec<i64>, Vec<i64>)>) -> bool {
    if c.len() != 16 {
        return false;
    }
    let mut matches = 0;
    for (ins, bef, aft) in runs {
        let mut m = Machine::new();
        m.regs[0] =  bef[0];
        m.regs[1] =  bef[1];
        m.regs[2] =  bef[2];
        m.regs[3] =  bef[3];
        if let Ok(_) = m.execute(c.get(&ins[0]).unwrap(), ins[1], ins[2], ins[3]) {
            if m.regs[0] == aft[0] && m.regs[1] == aft[1] && m.regs[2] == aft[2] && m.regs[3] == aft[3] {
                matches += 1;
            } else {
                break;
            }
        }
    }
    if matches != runs.len() {
        return false;
    }
    return true;
}

fn next(c: &HashMap<i64, Ops>, lock: (i64, Ops)) -> HashMap<i64, Ops> {
    let mut n = c.clone();
    n.insert(lock.0, lock.1);
    n
}

fn bt(c: &HashMap<i64, Ops>, runs: &Vec<(Vec<i64>, Vec<i64>, Vec<i64>)>) -> bool {
    let unambiguous = reject(&c, runs);
    if unambiguous.is_none() {
        return false;
    }
    if accept(&c, runs) {
        println!("{:?}", c);
        return true;
    }
    let mut cc = unambiguous.unwrap();
    cc.extend(c.into_iter().map(|(k, v)| (k.clone(), v.clone())));
    for opix in 0..16 {
        if cc.contains_key(&opix) {
            continue;
        }
        for op in &[Ops::Addr,
                    Ops::Addi,
                    Ops::Mulr,
                    Ops::Muli,
                    Ops::Banr,
                    Ops::Bani,
                    Ops::Borr,
                    Ops::Bori,
                    Ops::Setr,
                    Ops::Seti,
                    Ops::Gtir,
                    Ops::Gtri,
                    Ops::Gtrr,
                    Ops::Eqir,
                    Ops::Eqri,
                    Ops::Eqrr] {
            if let Some(_) = cc.values().find(|x| *x == op) {
                continue;
            }
            let x = next(&cc, (opix, *op));
            if bt(&x, runs) {
                return true;
            }
        }
    }
    return false;
}

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut runs = vec![];
    let mut before_regs : Vec<i64> = vec![];
    let mut instruction : Vec<i64> = vec![];
    let mut program = vec![];
    for (row, line) in lines.iter().enumerate() {
        if row < 3189 {
            if line.len() > 0 {
                if line[0..1] == *"B" {
                    before_regs = line[9..19].split(|c| c == ',').map(|s| s.trim().parse::<i64>().unwrap()).collect();
                } else if line[0..1] == *"A" {
                    let after_regs : Vec<i64> = line[9..19].split(|c| c == ',').map(|s| s.trim().parse::<i64>().unwrap()).collect();
                    runs.push((instruction.clone(), before_regs.clone(), after_regs));
                } else {
                    instruction = line.split(|c| c == ' ').map(|c| c.parse::<i64>().unwrap()).collect();
                }
            }
        } else {
            // program
            if line.len() > 0 {
                let instruction : Vec<i64> = line.split(|c| c == ' ').map(|c| c.parse::<i64>().unwrap()).collect();
                program.push(instruction);
            }
        }
    }

    let pt = 1;

    if pt == 0 {
        let mut gt3_matching = 0;
        for (ins, bef, aft) in &runs {
            let mut matching = 0;
            for op in &[Ops::Addr,
                        Ops::Addi,
                        Ops::Mulr,
                        Ops::Muli,
                        Ops::Banr,
                        Ops::Bani,
                        Ops::Borr,
                        Ops::Bori,
                        Ops::Setr,
                        Ops::Seti,
                        Ops::Gtir,
                        Ops::Gtri,
                        Ops::Gtrr,
                        Ops::Eqir,
                        Ops::Eqri,
                        Ops::Eqrr] {
                let mut m = Machine::new();
                m.regs[0] =  bef[0];
                m.regs[1] =  bef[1];
                m.regs[2] =  bef[2];
                m.regs[3] =  bef[3];
                println!("{:?}, {:?}, {:?}, {:?}", ins, bef, aft, op);
                if let Ok(_) = m.execute(op, ins[1], ins[2], ins[3]) {
                    if m.regs[0] == aft[0] && m.regs[1] == aft[1] && m.regs[2] == aft[2] && m.regs[3] == aft[3] {
                        matching += 1;
                        if matching >= 3 {
                            gt3_matching += 1;
                            break;
                        }
                    }
                }
            }
        }
        println!("{}", gt3_matching);
    } else if pt == 1 {
        let c : HashMap<i64, Ops> = HashMap::new();
        bt(&c, &runs);
    } else {
        // mapping is
        let insmap : HashMap<i64, Ops> = [(10, Ops::Bori),
                                          (12, Ops::Eqir),
                                          (6, Ops::Borr),
                                          (14, Ops::Gtrr),
                                          (4, Ops::Gtri),
                                          (7, Ops::Eqri),
                                          (15, Ops::Gtir),
                                          (8, Ops::Seti),
                                          (0, Ops::Bani),
                                          (13, Ops::Muli),
                                          (9, Ops::Eqrr),
                                          (1, Ops::Addr),
                                          (2, Ops::Mulr),
                                          (3, Ops::Addi),
                                          (5, Ops::Banr),
                                          (11, Ops::Setr)].iter().cloned().collect();
        let mut m = Machine::new();
        for p in program {
            match m.execute(insmap.get(&p[0]).unwrap(), p[1], p[2], p[3]) {
                Ok(_) => {},
                _ => panic!()
            }
        }
        println!("{:?}", m);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
