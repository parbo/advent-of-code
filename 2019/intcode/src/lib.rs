use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

enum Op {
    ADD,
    MUL,
    INP,
    OUT,
    JIT,
    JIF,
    LTN,
    EQL,
    HLT,
}

impl Op {
    fn from_i64(value: i64) -> Option<Op> {
        let v = value % 100;
        match v {
            1 => Some(Op::ADD),
            2 => Some(Op::MUL),
            3 => Some(Op::INP),
            4 => Some(Op::OUT),
            5 => Some(Op::JIT),
            6 => Some(Op::JIF),
            7 => Some(Op::LTN),
            8 => Some(Op::EQL),
            99 => Some(Op::HLT),
            _ => None,
        }
    }

    fn definition(&self) -> (&str, usize, usize) {
        match self {
            Op::ADD => ("ADD", 2, 1),
            Op::MUL => ("MUL", 2, 1),
            Op::INP => ("INP", 0, 1),
            Op::OUT => ("OUT", 1, 0),
            Op::JIT => ("JIT", 2, 0),
            Op::JIF => ("JIF", 2, 0),
            Op::LTN => ("LTN", 2, 1),
            Op::EQL => ("EQL", 2, 1),
            Op::HLT => ("HLT", 0, 0),
        }
    }
}

enum Mode {
    Position,
    Immediate,
}

fn mode(value: i64, pos: usize) -> Mode {
    if ((value / (100 * (pos as i64))) % 10) == 0 {
        Mode::Position
    } else {
        Mode::Immediate
    }
}

fn mode_str(value: i64, pos: usize) -> &'static str {
    match mode(value, pos) {
        Mode::Position => "%",
        Mode::Immediate => "$",
    }
}

pub struct Machine {
    memory: Vec<i64>,
    ip: usize,
    input: i64,
    outputs: Vec<i64>,
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
            outputs: Vec::new(),
            executes: HashMap::new(),
            reads: HashMap::new(),
            writes: HashMap::new(),
        }
    }

    pub fn outputs(&self) -> Vec<i64> {
        let o = self.outputs.clone();
        o
    }

    pub fn write(&mut self, pos: usize, value: i64) {
        self.memory[pos] = value;
    }

    fn read_immediate_mut(&mut self, pos: usize) -> Option<&i64> {
        *self.reads.entry(pos).or_insert(0) += 1;
        self.memory.get(pos)
    }

    fn read_position_mut(&mut self, pos: usize) -> Option<&i64> {
        let addr = *self.memory.get(pos)? as usize;
        *self.reads.entry(addr).or_insert(0) += 1;
        self.memory.get(addr)
    }

    fn read_immediate(&self, pos: usize) -> Option<&i64> {
        self.memory.get(pos)
    }

    fn read_position(&self, pos: usize) -> Option<&i64> {
        let addr = *self.memory.get(pos)? as usize;
        self.memory.get(addr)
    }

    fn read_mode_mut(&mut self, pos: usize, mode: Mode) -> Option<&i64> {
        match mode {
            Mode::Immediate => self.read_immediate_mut(pos),
            Mode::Position => self.read_position_mut(pos),
        }
    }

    fn read_mode(&self, pos: usize, mode: Mode) -> Option<&i64> {
        match mode {
            Mode::Immediate => self.read_immediate(pos),
            Mode::Position => self.read_position(pos),
        }
    }

    fn write_position(&mut self, pos: usize, value: i64) {
        // TODO: error handling
        let addr = self.memory[pos] as usize;
        *self.writes.entry(addr).or_insert(0) += 1;
        self.memory[addr] = value;
    }

    // Returns instruction size
    pub fn step(&mut self) -> bool {
        // let _ = self.print_instruction(self.ip);
        let mut pos = self.ip;
        *self.executes.entry(pos).or_insert(0) += 1;
        let val = match self.memory.get(pos) {
            None => return false,
            Some(v) => *v,
        };
        let op = match Op::from_i64(val) {
            None => return false,
            Some(v) => v,
        };
        let def = op.definition();
        pos += 1;
        // Read input
        let mut vals = [0; 4];
        for r in 0..def.1 {
            vals[r] = match self.read_mode_mut(pos, mode(val, r + 1)) {
                None => return false,
                Some(v) => *v,
            };
            pos += 1;
        }
        let mut next_pos = pos + def.2;
        let v = match op {
            Op::ADD => Some(vals[0] + vals[1]),
            Op::MUL => Some(vals[0] * vals[1]),
            Op::INP => Some(self.input),
            Op::OUT => {
                self.outputs.push(vals[0]);
                println!("OUT: {}", vals[0]);
                None
            }
            Op::JIT => {
                if vals[0] != 0 {
                    next_pos = vals[1] as usize;
                }
                None
            }
            Op::JIF => {
                if vals[0] == 0 {
                    next_pos = vals[1] as usize;
                }
                None
            }
            Op::LTN => Some(if vals[0] < vals[1] { 1 } else { 0 }),
            Op::EQL => Some(if vals[0] == vals[1] { 1 } else { 0 }),
            Op::HLT => return false,
        };
        if let Some(out) = v {
            assert!(def.2 == 1, "{:?}", def);
            self.write_position(pos, out);
        } else {
            assert!(def.2 == 0, "{:?}", def);
        }
        self.ip = next_pos;
        true
    }

    fn print_instruction(&self, a: usize) -> usize {
        let mut addr = a;
        let val = *self.memory.get(addr).unwrap();
        if let Some(op) = Op::from_i64(val) {
            let def = op.definition();
            print!("{:>04} {} ", addr, def.0);
            for r in 0..4 {
                if r < def.1 + def.2 {
                    print!(
                        "{}{:<10}",
                        mode_str(val, 1 + r),
                        self.memory.get(addr + 1 + r).unwrap_or(&-1)
                    );
                } else {
                    print!("           ");
                }
            }
            print!("; ");
            for r in 0..def.1 {
                print!(
                    "{} ",
                    self.read_mode(addr + 1 + r, mode(val, 1 + r))
                        .unwrap_or(&-1)
                );
            }
            if def.2 > 0 {
                print!("-> ");
            }
            for w in 0..def.2 {
                let addr2 = *self.memory.get(addr + 1 + def.1 + w).unwrap_or(&0) as usize;
                let val = self.memory.get(addr2).unwrap_or(&-1);
                print!("%{} ({})", addr2, val);
            }
            println!();
            addr += 1 + def.1 + def.2;
        } else {
            println!("{:>04} {}", addr, self.memory.get(addr).unwrap_or(&-1));
            addr += 1;
        };
        addr
    }

    pub fn run(&mut self) -> Option<i64> {
        loop {
            if !self.step() {
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
        let _ = self.print_instruction(self.ip);
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    if line == "s" {
                        if !self.step() {
                            println!("Program halted");
                        } else {
                            let _ = self.print_instruction(self.ip);
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
                        let parts: Vec<_> = line.split(' ').map(|x| x.trim()).collect();
                        let addr = parts[1].parse::<usize>().unwrap();
                        let val = parts[2].parse::<i64>().unwrap();
                        self.write(addr, val);
                    } else if line == "m" {
                        self.memory
                            .iter()
                            .enumerate()
                            .skip(self.ip)
                            .take(8)
                            .for_each(|(a, &v)| println!("{:>04}, {}", a, v));
                    } else if line == "ds" {
                        self.dump(5);
                    } else if line == "l" {
                        let mut addr = self.ip;
                        loop {
                            addr = self.print_instruction(addr);
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

    #[test]
    fn test_example_1() {
        let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut m = Machine::new(&input, 6);
        let _ = m.run();
        assert_eq!(m.outputs[0], 0);
        let mut m2 = Machine::new(&input, 8);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1);
    }

    #[test]
    fn test_example_2() {
        let input = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut m = Machine::new(&input, 6);
        let _ = m.run();
        assert_eq!(m.outputs[0], 1);
        let mut m2 = Machine::new(&input, 8);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 0);
    }

    #[test]
    fn test_example_3() {
        let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut m = Machine::new(&input, 42);
        let _ = m.run();
        assert_eq!(m.outputs[0], 0);
        let mut m2 = Machine::new(&input, 8);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1);
    }

    #[test]
    fn test_example_4() {
        let input = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut m = Machine::new(&input, 6);
        let _ = m.run();
        assert_eq!(m.outputs[0], 1);
        let mut m2 = Machine::new(&input, 8);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 0);
    }

    #[test]
    fn test_jump_1() {
        let input = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut m = Machine::new(&input, 0);
        let _ = m.run();
        assert_eq!(m.outputs[0], 0);
        let mut m2 = Machine::new(&input, 42);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1);
    }

    #[test]
    fn test_jump_2() {
        let input = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut m = Machine::new(&input, 0);
        let _ = m.run();
        assert_eq!(m.outputs[0], 0);
        let mut m2 = Machine::new(&input, 42);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1);
    }

    #[test]
    fn test_large_example_1() {
        let input = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut m = Machine::new(&input, 6);
        let _ = m.run();
        assert_eq!(m.outputs[0], 999);
        let mut m2 = Machine::new(&input, 8);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1000);
        let mut m3 = Machine::new(&input, 14);
        let _3 = m3.run();
        assert_eq!(m3.outputs[0], 1001);
    }
}
