use std::collections::HashMap;
use std::fmt;

pub enum Op {
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
    fn from_i128(value: i128) -> Option<Op> {
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

    pub fn definition(&self) -> (&str, usize, usize) {
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

#[derive(Copy, Clone, Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

fn mode(value: i128, pos: usize) -> Mode {
    let mut v = value / 100;
    for _ in 1..pos {
	v = v / 10;
    }
    let m = v % 10;
    match m {
        0 => Mode::Position,
        1 => Mode::Immediate,
        2 => Mode::Relative,
        _ => panic!("OH NOES: {}", m),
    }
}

pub struct MemoryValue {
    address: usize,
    value: i128,
}

impl fmt::Display for MemoryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>04} {}", self.address, self.value)
    }
}

pub enum Arg {
    Immediate(i128),
    Position(MemoryValue),
    Relative(MemoryValue),
}

impl Arg {
    pub fn value(&self) -> i128 {
        match self {
            Arg::Immediate(v) => *v,
            Arg::Position(mv) => mv.value,
            Arg::Relative(mv) => mv.value,
        }
    }
}

pub struct Instruction {
    pub address: usize,
    pub op: Op,
    pub read: Vec<Arg>,
    pub write: Vec<Arg>,
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
                Arg::Relative(mv) => write!(f, "~{} ", mv.address)?,
            }
        }
        if self.write.len() > 0 {
            write!(f, "-> ")?;
        }
        for w in &self.write {
            match w {
                Arg::Immediate(val) => write!(f, "${} ", val)?,
                Arg::Position(mv) => write!(f, "%{} ", mv.address)?,
                Arg::Relative(mv) => write!(f, "~{} ", mv.address)?,
            }
        }
        write!(f, "; ")?;
        for r in &self.read {
            match r {
                Arg::Immediate(val) => write!(f, "{} ", val)?,
                Arg::Position(mv) => write!(f, "{} ", mv.value)?,
                Arg::Relative(mv) => write!(f, "{} ", mv.value)?,
            }
        }
        if self.write.len() > 0 {
            write!(f, "-> ")?;
        }
        for w in &self.write {
            match w {
                Arg::Position(mv) => write!(f, "%{} ({}) ", mv.address, mv.value)?,
                Arg::Relative(mv) => write!(f, "%{} ({}) ", mv.address, mv.value)?,
                _ => panic!("OH NOES"),
            }
        }
        Ok(())
    }
}

pub enum Disassembly {
    Instruction(Instruction),
    MemoryValue(MemoryValue),
}

pub struct Machine {
    memory: Vec<i128>,
    ip: usize,
    inputs: Vec<i128>,
    curr_input: usize,
    outputs: Vec<i128>,
    executes: HashMap<usize, usize>,
    reads: HashMap<usize, usize>,
    writes: HashMap<usize, usize>,
    relative_base: i128,
}

impl Machine {
    pub fn new(memory: &Vec<i128>, inputs: &Vec<i128>) -> Machine {
        Machine {
            memory: memory.clone(),
            ip: 0,
            inputs: inputs.clone(),
            curr_input: 0,
            outputs: Vec::new(),
            executes: HashMap::new(),
            reads: HashMap::new(),
            writes: HashMap::new(),
            relative_base: 0,
        }
    }

    pub fn outputs(&mut self) -> Vec<i128> {
        let o = self.outputs.clone();
        self.outputs.clear();
        o
    }

    pub fn memory(&self) -> &[i128] {
        self.memory.as_slice()
    }

    pub fn memory_mut(&mut self) -> &mut [i128] {
        self.memory.as_mut_slice()
    }

    pub fn ip(&self) -> usize {
        self.ip
    }

    fn read_immediate(&self, pos: usize) -> Option<&i128> {
        self.memory.get(pos)
    }

    fn read_position(&self, pos: usize) -> Option<&i128> {
        let addr = *self.memory.get(pos)? as usize;
        self.memory.get(addr)
    }

    fn read_relative(&self, pos: usize) -> Option<&i128> {
        let addr = (self.relative_base + *self.memory.get(pos)?) as usize;
        self.memory.get(addr)
    }

    fn read_mode(&self, pos: usize, mode: Mode) -> Option<&i128> {
        match mode {
            Mode::Immediate => self.read_immediate(pos),
            Mode::Position => self.read_position(pos),
            Mode::Relative => self.read_relative(pos),
        }
    }

    fn write(&mut self, address: usize, value: i128) {
        // TODO: error handling
        *self.writes.entry(address).or_insert(0) += 1;
        self.memory[address] = value;
    }

    fn input(&mut self) -> i128 {
        let res = self.inputs[self.curr_input];
        self.curr_input += 1;
        res
    }

    pub fn add_inputs(&mut self, inputs: &Vec<i128>) {
        self.inputs.extend(inputs);
    }

    // Returns instruction size
    pub fn step(&mut self) -> bool {
        *self.executes.entry(self.ip).or_insert(0) += 1;
        match self.get_disassembly(self.ip) {
            Some(Disassembly::Instruction(x)) => {
                let mut next_pos = self.ip + x.increment();
                let v = match x.op {
                    Op::ADD => Some(x.read[0].value() + x.read[1].value()),
                    Op::MUL => Some(x.read[0].value() * x.read[1].value()),
                    Op::INP => Some(self.input()),
                    Op::OUT => {
                        self.outputs.push(x.read[0].value());
                        //                println!("OUT: {}", vals[0]);
                        None
                    }
                    Op::JIT => {
                        if x.read[0].value() != 0 {
                            next_pos = x.read[1].value() as usize;
                        }
                        None
                    }
                    Op::JIF => {
                        if x.read[0].value() == 0 {
                            next_pos = x.read[1].value() as usize;
                        }
                        None
                    }
                    Op::LTN => Some(if x.read[0].value() < x.read[1].value() {
                        1
                    } else {
                        0
                    }),
                    Op::EQL => Some(if x.read[0].value() == x.read[1].value() {
                        1
                    } else {
                        0
                    }),
                    Op::HLT => return false,
                };
                if let Some(out) = v {
                    assert!(x.op.definition().2 == 1);
                    match &x.write[0] {
                        Arg::Position(mv) => self.write(mv.address, out),
                        Arg::Relative(mv) => self.write(mv.address, out),
                        _ => panic!("OH NOES"),
                    }
                } else {
                    assert!(x.op.definition().2 == 0);
                }
                // Update stats
                for r in &x.read {
                    match r {
                        Arg::Position(mv) => {
                            *self.reads.entry(mv.address).or_insert(0) += 1;
                        }
                        Arg::Relative(mv) => {
                            *self.reads.entry(mv.address).or_insert(0) += 1;
                        }
                        _ => {}
                    }
                }
                for w in &x.write {
                    match w {
                        Arg::Position(mv) => {
                            *self.writes.entry(mv.address).or_insert(0) += 1;
                        }
                        Arg::Relative(mv) => {
                            *self.writes.entry(mv.address).or_insert(0) += 1;
                        }
                        _ => {}
                    }
                }
                *self.executes.entry(self.ip).or_insert(0) += 1;
                // Update ip
                self.ip = next_pos;
                true
            }
            _ => false,
        }
    }

    pub fn get_disassembly(&self, address: usize) -> Option<Disassembly> {
        let val = *self.memory.get(address)?;
        if let Some(op) = Op::from_i128(val) {
            let def = op.definition();
            let mut read = vec![];
            for r in 0..def.1 {
                let m = mode(val, 1 + r);
                let v = self.read_mode(address + 1 + r, m)?;
                match &m {
                    Mode::Immediate => {
                        read.push(Arg::Immediate(*v));
                    }
                    Mode::Position => {
                        let mv = MemoryValue {
                            address: address + 1 + r,
                            value: *v,
                        };
                        read.push(Arg::Position(mv));
                    }
                    Mode::Relative => {
                        let mv = MemoryValue {
                            address: (self.relative_base as usize) + address + 1 + r,
                            value: *v,
                        };
                        read.push(Arg::Relative(mv));
                    }
                }
            }
            let mut write = vec![];
            for w in 0..def.2 {
                let m = mode(val, 1 + def.1 + w);
                let v = self.read_mode(address + 1 + def.1 + w, m)?;
                match &m {
                    Mode::Position => {
                        let addr2 = *self.memory.get(address + 1 + def.1 + w)? as usize;
                        let mv = MemoryValue {
                            address: addr2,
                            value: *v,
                        };
                        write.push(Arg::Position(mv));
                    }
                    Mode::Relative => {
                        let addr2 = *self
                            .memory
                            .get((self.relative_base as usize) + address + 1 + def.1 + w)?
                            as usize;
                        let mv = MemoryValue {
                            address: addr2,
                            value: *v,
                        };
                        write.push(Arg::Relative(mv));
                    }
                    _ => panic!("OH NOES"),
                }
            }
            Some(Disassembly::Instruction(Instruction {
                address,
                op,
                read,
                write,
            }))
        } else {
            Some(Disassembly::MemoryValue(MemoryValue {
                address: address,
                value: val,
            }))
        }
    }

    pub fn run_to_next_output(&mut self) -> Option<i128> {
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

    pub fn run(&mut self) -> Option<i128> {
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
