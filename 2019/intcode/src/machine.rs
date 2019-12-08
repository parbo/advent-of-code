use std::collections::HashMap;
use std::fmt;

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

#[derive(Copy, Clone)]
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

pub struct MemoryValue {
    address: usize,
    value: i64,
}

impl fmt::Display for MemoryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>04} {}", self.address, self.value)
    }
}

pub enum Arg {
    Immediate(i64),
    Position(MemoryValue),
}

pub struct Instruction {
    address: usize,
    op: Op,
    read: Vec<Arg>,
    write: Vec<MemoryValue>,
}

impl Instruction {
    pub fn name(&self) -> &str {
        self.op.definition().0
    }

    pub fn increment(&self) -> usize {
        let def = self.op.definition();
        1 + def.1 + def.2
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>04} {} ", self.address, self.name())?;
        for r in &self.read {
            match r {
                Arg::Immediate(val) => write!(f, "${} ", val)?,
                Arg::Position(mv) => write!(f, "%{} ", mv.address)?,
            }
        }
        if self.write.len() > 0 {
            write!(f, "-> ")?;
        }
        for w in &self.write {
            write!(f, "%{} ", w.address)?;
        }
        write!(f, "; ")?;
        for r in &self.read {
            match r {
                Arg::Immediate(val) => write!(f, "{} ", val)?,
                Arg::Position(mv) => write!(f, "{} ", mv.value)?,
            }
        }
        if self.write.len() > 0 {
            write!(f, "-> ")?;
        }
        for w in &self.write {
            write!(f, "%{} ({}) ", w.address, w.value)?;
        }
        Ok(())
    }
}

pub enum Disassembly {
    Instruction(Instruction),
    MemoryValue(MemoryValue),
}

pub struct Machine {
    memory: Vec<i64>,
    ip: usize,
    inputs: Vec<i64>,
    curr_input: usize,
    outputs: Vec<i64>,
    executes: HashMap<usize, usize>,
    reads: HashMap<usize, usize>,
    writes: HashMap<usize, usize>,
}

impl Machine {
    pub fn new(memory: &Vec<i64>, inputs: &Vec<i64>) -> Machine {
        Machine {
            memory: memory.clone(),
            ip: 0,
            inputs: inputs.clone(),
            curr_input: 0,
            outputs: Vec::new(),
            executes: HashMap::new(),
            reads: HashMap::new(),
            writes: HashMap::new(),
        }
    }

    pub fn outputs(&mut self) -> Vec<i64> {
        let o = self.outputs.clone();
        self.outputs.clear();
        o
    }

    pub fn memory(&self) -> &[i64] {
        self.memory.as_slice()
    }

    pub fn memory_mut(&mut self) -> &mut [i64] {
        self.memory.as_mut_slice()
    }

    pub fn ip(&self) -> usize {
        self.ip
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

    fn input(&mut self) -> i64 {
        let res = self.inputs[self.curr_input];
        self.curr_input += 1;
        res
    }

    pub fn add_inputs(&mut self, inputs: &Vec<i64>) {
        self.inputs.extend(inputs);
    }

    // Returns instruction size
    pub fn step(&mut self) -> bool {
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
            Op::INP => Some(self.input()),
            Op::OUT => {
                self.outputs.push(vals[0]);
                //                println!("OUT: {}", vals[0]);
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

    pub fn get_current_disassembly(&self) -> Disassembly {
        self.get_disassembly(self.ip)
    }

    pub fn get_disassembly(&self, address: usize) -> Disassembly {
        let val = *self.memory.get(address).unwrap();
        if let Some(op) = Op::from_i64(val) {
            let def = op.definition();
            let mut read = vec![];
            for r in 0..def.1 {
                let m = mode(val, 1 + r);
                let v = self.read_mode(address + 1 + r, m).unwrap_or(&-1);
                match &m {
                    Mode::Immediate => {
                        read.push(Arg::Immediate(*v));
                    },
                    Mode::Position => {
                        let mv = MemoryValue{ address: address + 1 + r, value: *v };
                        read.push(Arg::Position(mv));
                    }
                }
            }
            let mut write = vec![];
            for w in 0..def.2 {
                let addr2 = *self.memory.get(address + 1 + def.1 + w).unwrap_or(&0) as usize;
                let val = self.memory.get(addr2).unwrap_or(&-1);
                write.push(MemoryValue{ address: addr2, value: *val });
            }
            Disassembly::Instruction(Instruction{address, op, read, write})
        } else {
            Disassembly::MemoryValue(MemoryValue{address: address, value: val})
        }
    }

    pub fn run_to_next_output(&mut self) -> Option<i64> {
        let res = loop {
            let cont = self.step();
            if let Some(v) = self.outputs.last() {
                break Some(*v);
            }
            if !cont {
                break None;
            }
        };
        self.outputs.clear();
        res
    }

    pub fn run(&mut self) -> Option<i64> {
        loop {
            if !self.step() {
                break;
            }
        }
        Some(*self.memory.get(0)?)
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
        let mut m = Machine::new(&input, &vec![0]);
        assert_eq!(m.run(), Some(3500));
        m.dump(10);
    }

    #[test]
    fn test_example_1() {
        let input = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut m = Machine::new(&input, &vec![6]);
        let _ = m.run();
        assert_eq!(m.outputs[0], 0);
        let mut m2 = Machine::new(&input, &vec![8]);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1);
    }

    #[test]
    fn test_example_2() {
        let input = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut m = Machine::new(&input, &vec![6]);
        let _ = m.run();
        assert_eq!(m.outputs[0], 1);
        let mut m2 = Machine::new(&input, &vec![8]);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 0);
    }

    #[test]
    fn test_example_3() {
        let input = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut m = Machine::new(&input, &vec![42]);
        let _ = m.run();
        assert_eq!(m.outputs[0], 0);
        let mut m2 = Machine::new(&input, &vec![8]);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1);
    }

    #[test]
    fn test_example_4() {
        let input = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut m = Machine::new(&input, &vec![6]);
        let _ = m.run();
        assert_eq!(m.outputs[0], 1);
        let mut m2 = Machine::new(&input, &vec![8]);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 0);
    }

    #[test]
    fn test_jump_1() {
        let input = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut m = Machine::new(&input, &vec![0]);
        let _ = m.run();
        assert_eq!(m.outputs[0], 0);
        let mut m2 = Machine::new(&input, &vec![42]);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1);
    }

    #[test]
    fn test_jump_2() {
        let input = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut m = Machine::new(&input, &vec![0]);
        let _ = m.run();
        assert_eq!(m.outputs[0], 0);
        let mut m2 = Machine::new(&input, &vec![42]);
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
        let mut m = Machine::new(&input, &vec![6]);
        let _ = m.run();
        assert_eq!(m.outputs[0], 999);
        let mut m2 = Machine::new(&input, &vec![8]);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1000);
        let mut m3 = Machine::new(&input, &vec![14]);
        let _3 = m3.run();
        assert_eq!(m3.outputs[0], 1001);
    }
}
