use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

enum Op {
    ADD = 1,
    MUL = 2,
    INP = 3,
    OUT = 4,
    HLT = 99,
}

impl Op {
    fn from_i64(value: i64) -> Option<Op> {
        let v = value % 100;
        match v {
            1 => Some(Op::ADD),
            2 => Some(Op::MUL),
            3 => Some(Op::INP),
            4 => Some(Op::OUT),
            99 => Some(Op::HLT),
            _ => None,
        }
    }
}

fn mode(value: i64, pos: i64) -> bool {
    (value / (100 * pos)) != 0
}

pub struct Machine {
    memory: Vec<i64>,
    ip: usize,
    input: i64,
    executes: HashMap<usize, usize>,
    reads: HashMap<usize, usize>,
    writes: HashMap<usize, usize>,
}

impl Machine {
    pub fn new(memory: &Vec<i64>, input: i64) -> Machine {
        Machine {
            memory: memory.clone(),
            ip: 0,
            input: input,
            executes: HashMap::new(),
            reads: HashMap::new(),
            writes: HashMap::new(),
        }
    }

    pub fn write(&mut self, pos: usize, value: i64) {
        self.memory[pos] = value;
    }

    fn read_immediate(&mut self, pos: usize) -> Option<&i64> {
        *self.reads.entry(pos).or_insert(0) += 1;
        self.memory.get(pos)
    }

    fn write_immediate(&mut self, pos: usize, value: i64) {
        // TODO: error handling
        *self.writes.entry(pos).or_insert(0) += 1;
        self.memory[pos] = value;
    }

    fn read_position(&mut self, pos: usize) -> Option<&i64> {
        let addr = *self.memory.get(pos)? as usize;
        *self.reads.entry(addr).or_insert(0) += 1;
        self.memory.get(addr)
    }

    fn write_position(&mut self, pos: usize, value: i64) {
        // TODO: error handling
        let addr = self.memory[pos] as usize;
        *self.writes.entry(addr).or_insert(0) += 1;
        self.memory[addr] = value;
    }

    // Returns instruction size
    pub fn step(&mut self) -> Option<usize> {
        let pos = self.ip;
        *self.executes.entry(pos).or_insert(0) += 1;
        let val = *self.memory.get(pos)?;
        let op = Op::from_i64(val)?;
        match op {
            Op::ADD => {
                let v1 = if mode(val, 1) { *self.read_immediate(pos + 1)? } else { *self.read_position(pos + 1)? };
                let v2 = if mode(val, 2) { *self.read_immediate(pos + 2)? } else { *self.read_position(pos + 2)? };
                let res = v1 + v2;
                if mode(val, 3) { self.write_immediate(pos + 3, res); } else { self.write_position(pos + 3, res); }
                Some(4)
            }
            Op::MUL => {
                let v1 = if mode(val, 1) { *self.read_immediate(pos + 1)? } else { *self.read_position(pos + 1)? };
                let v2 = if mode(val, 2) { *self.read_immediate(pos + 2)? } else { *self.read_position(pos + 2)? };
                let res = v1 * v2;
                if mode(val, 3) { self.write_immediate(pos + 3, res); } else { self.write_position(pos + 3, res); }
                Some(4)
            }
            Op::INP => {
                let res = self.input;
                if mode(val, 1) { self.write_immediate(pos + 1, res); } else { self.write_position(pos + 1, res); }
                Some(2)
            }
            Op::OUT => {
                let v1 = if mode(val, 1) { *self.read_immediate(pos + 1)? } else { *self.read_position(pos + 1)? };
                println!("OUT: {}", v1);
                Some(2)
            }
            Op::HLT => None,
        }
    }

    pub fn run(&mut self) -> Option<i64> {
        loop {
            if let Some(inc) = self.step() {
                self.ip += inc;
            } else {
                break;
            }
        }
        Some(*self.memory.get(0)?)
    }

    pub fn debug(&mut self) {
        // `()` can be used when no completer is required
        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    if line == "s" {
                        if let Some(inc) = self.step() {
                            self.ip += inc;
                        } else {
                            println!("Program halted");
                        }
                    } else if line == "c" {
                        let _ = self.run();
                        println!("Program halted");
                    } else if line.starts_with("p ") {
                        if let Ok(addr) = line[2..].trim().parse::<usize>() {
                            self.memory
                                .iter()
                                .enumerate()
                                .skip(addr)
                                .take(8)
                                .for_each(|(a, &v)| println!("{:>04}, {}", a, v));
                        } else {
                            println!("Invalid command: {}", line);
                        }
                    } else if line.starts_with("w ") {
                        let parts : Vec<_> = line.split(' ').map(|x| x.trim()).collect();
                        let addr = parts[1].parse::<usize>().unwrap();
                        let val = parts[2].parse::<i64>().unwrap();
                        self.write(addr, val);
                    } else if line == "l" {
                        self.memory
                            .iter()
                            .enumerate()
                            .skip(self.ip)
                            .take(8)
                            .for_each(|(a, &v)| println!("{:>04}, {}", a, v));
                    } else if line == "ds" {
                        self.dump(5);
                    } else if line == "dis" {
                        let mut addr = self.ip;
                        loop {
                            let val = *self.memory.get(addr).unwrap();
                            let op = Op::from_i64(val);
                            let modes = val / 100;
                            let inc = match op {
                                Some(Op::ADD) => {
                                    println!(
                                        "{:>04} ADD {} {} {} {}",
                                        addr,
                                        modes,
                                        self.memory.get(addr + 1).unwrap_or(&-1),
                                        self.memory.get(addr + 2).unwrap_or(&-1),
                                        self.memory.get(addr + 3).unwrap_or(&-1)
                                    );
                                    4
                                }
                                Some(Op::MUL) => {
                                    println!(
                                        "{:>04} MUL {} {} {} {}",
                                        addr,
                                        modes,
                                        self.memory.get(addr + 1).unwrap_or(&-1),
                                        self.memory.get(addr + 2).unwrap_or(&-1),
                                        self.memory.get(addr + 3).unwrap_or(&-1)
                                    );
                                    4
                                }
                                Some(Op::INP) => {
                                    println!(
                                        "{:>04} INP {} {}",
                                        addr,
                                        modes,
                                        self.memory.get(addr + 1).unwrap_or(&-1)
                                    );
                                    2
                                }
                                Some(Op::OUT) => {
                                    println!(
                                        "{:>04} OUT {} {}",
                                        addr,
                                        modes,
                                        self.memory.get(addr + 1).unwrap_or(&-1)
                                    );
                                    2
                                }
                                Some(Op::HLT) => {
                                    println!("{:>04} HLT", addr);
                                    1
                                }
                                None => {
                                    println!(
                                        "{:>04} {}",
                                        addr,
                                        self.memory.get(addr).unwrap_or(&-1)
                                    );
                                    1
                                }
                            };
                            addr += inc;
                            if addr - self.ip > 18 {
                                break;
                            }
                        }
                    } else {
                        println!("Invalid command: {}", line);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        rl.save_history("history.txt").unwrap();
    }

    pub fn dump(&self, n: usize) {
        let mut exec_vec: Vec<(&usize, &usize)> = self.executes.iter().collect();
        exec_vec.sort_by(|a, b| b.1.cmp(a.1));
        let mut read_vec: Vec<(&usize, &usize)> = self.reads.iter().collect();
        read_vec.sort_by(|a, b| b.1.cmp(a.1));
        let mut write_vec: Vec<(&usize, &usize)> = self.writes.iter().collect();
        write_vec.sort_by(|a, b| b.1.cmp(a.1));
        println!("Executed:");
        exec_vec
            .iter()
            .take(n)
            .for_each(|x| println!("  {} - {}", x.0, x.1));
        println!("Read:");
        read_vec
            .iter()
            .take(n)
            .for_each(|x| println!("  {} - {}", x.0, x.1));
        println!("Written:");
        write_vec
            .iter()
            .take(n)
            .for_each(|x| println!("  {} - {}", x.0, x.1));
    }
}

#[cfg(test)]
mod tests {
    use super::Machine;

    #[test]
    fn test() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut m = Machine::new(&input, 0);
        assert_eq!(m.run(), Some(3500));
        m.dump(10);
    }
}
