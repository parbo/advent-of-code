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
    SP,
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
            9 => Some(Op::SP),
            99 => Some(Op::HLT),
            _ => None,
        }
    }

    pub fn definition(&self) -> (&'static str, usize, usize) {
        match self {
            Op::ADD => ("ADD", 2, 1),
            Op::MUL => ("MUL", 2, 1),
            Op::INP => ("INP", 0, 1),
            Op::OUT => ("OUT", 1, 0),
            Op::JIT => ("JIT", 2, 0),
            Op::JIF => ("JIF", 2, 0),
            Op::LTN => ("LTN", 2, 1),
            Op::EQL => ("EQL", 2, 1),
            Op::SP => ("SP", 1, 0),
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
    let mut div = 100;
    for _ in 1..pos {
        div = div * 10;
    }
    let m = (value / div) % 10;
    match m {
        0 => Mode::Position,
        1 => Mode::Immediate,
        2 => Mode::Relative,
        _ => panic!("OH NOES: {}", m),
    }
}

#[derive(Debug)]
pub struct MemoryValue {
    address: usize,
    value: i128,
}

impl fmt::Display for MemoryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>04} {}", self.address, self.value)
    }
}

#[derive(Copy, Clone)]
pub enum Arg {
    Immediate { value: i128 },
    Position { address: usize },
    Relative { base: i128, offset: i128 },
}

pub struct Instruction {
    pub address: usize,
    pub op: Op,
    args: [Arg;4],
    read: usize,
    write: usize,
}

impl Instruction {
    pub fn new(address: usize, op: Op) -> Instruction {
	Instruction {
	    address,
	    op,
	    args: [Arg::Immediate { value: 0 };4],
	    read: 0,
	    write: 0,
	}
    }

    pub fn add_read(&mut self, arg: Arg) {
	self.args[self.read] = arg;
	self.read += 1;
    }

    pub fn add_write(&mut self, arg: Arg) {
	self.args[self.read + self.write] = arg;
	self.write += 1;
    }

    pub fn read<'a>(&'a self) -> &'a [Arg] {
	&self.args[0..self.read]
    }

    pub fn write<'a>(&'a self) -> &'a [Arg] {
	&self.args[self.read..(self.read+self.write)]
    }

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
        for r in self.read() {
            match r {
                Arg::Immediate { value } => write!(f, "{}, ", value)?,
                Arg::Position { address } => write!(f, "[{}], ", address)?,
                Arg::Relative { base: _, offset } => write!(
                    f,
                    "[SP{}{}], ",
                    if offset.is_positive() { "+" } else { "" },
                    offset
                )?,
            }
        }
        if self.write().len() > 0 {
            write!(f, "-> ")?;
        }
        for w in self.write() {
            match w {
                Arg::Immediate { value } => write!(f, "{}, ", value)?,
                Arg::Position { address } => write!(f, "[{}], ", address)?,
                Arg::Relative { base: _, offset } => write!(
                    f,
                    "[SP{}{}], ",
                    if offset.is_positive() { "+" } else { "" },
                    offset
                )?,
            }
        }
        Ok(())
    }
}

pub enum Disassembly {
    Instruction(Instruction),
    MemoryValue(MemoryValue),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    Running,
    Input,
    Output,
    Halted,
    Invalid,
}

#[derive(Clone)]
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
    stats: bool,
}

impl Machine {
    pub fn with_input(memory: &[i128], inputs: &[i128]) -> Machine {
        Machine {
            memory: memory.to_vec(),
            ip: 0,
            inputs: inputs.to_vec(),
            curr_input: 0,
            outputs: Vec::new(),
            executes: HashMap::new(),
            reads: HashMap::new(),
            writes: HashMap::new(),
            relative_base: 0,
	    stats: false,
        }
    }

    pub fn set_stats(&mut self, s: bool) {
	self.stats = s;
    }

    pub fn new(memory: &[i128]) -> Machine {
        Machine::with_input(memory, &[])
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

    pub fn sp(&self) -> usize {
        self.relative_base as usize
    }

    pub fn read_arg(&self, arg: &Arg) -> i128 {
        match arg {
            Arg::Immediate { value } => *value,
            Arg::Position { address } => *self.memory.get(*address).unwrap_or(&0),
            Arg::Relative { base, offset } => {
                *self.memory.get((base + offset) as usize).unwrap_or(&0)
            }
        }
    }

    fn write_arg(&mut self, arg: &Arg, value: i128) {
        let address = match arg {
            Arg::Position { address } => *address,
            Arg::Relative { base, offset } => (base + offset) as usize,
            _ => panic!("can't write immediate"),
        };
        if address >= self.memory.len() {
            self.memory.resize(address + 1, 0);
        }
        self.memory[address] = value;
    }

    fn input(&mut self) -> Option<i128> {
        if self.curr_input < self.inputs.len() {
            let res = self.inputs[self.curr_input];
            self.curr_input += 1;
            Some(res)
        } else {
            None
        }
    }

    pub fn add_input(&mut self, input: i128) {
        self.inputs.push(input);
    }

    pub fn add_inputs(&mut self, inputs: &[i128]) {
        self.inputs.extend(inputs);
    }

    pub fn input_len(&self) -> usize {
	self.inputs.len() - self.curr_input
    }

    pub fn step(&mut self) -> State {
        match self.get_disassembly(self.ip) {
            Disassembly::Instruction(x) => {
                // println!("{}", x);
                let mut next_pos = self.ip + x.increment();
                let mut state = State::Running;
                let v = match x.op {
                    Op::ADD => Some(self.read_arg(&x.read()[0]) + self.read_arg(&x.read()[1])),
                    Op::MUL => Some(self.read_arg(&x.read()[0]) * self.read_arg(&x.read()[1])),
                    Op::INP => {
                        if let Some(x) = self.input() {
                            Some(x)
                        } else {
                            return State::Input;
                        }
                    }
                    Op::OUT => {
                        state = State::Output;
                        self.outputs.push(self.read_arg(&x.read()[0]));
                        // println!("OUT: {}", self.read_arg(x.read[0]));
                        None
                    }
                    Op::JIT => {
                        if self.read_arg(&x.read()[0]) != 0 {
                            next_pos = self.read_arg(&x.read()[1]) as usize;
                        }
                        None
                    }
                    Op::JIF => {
                        if self.read_arg(&x.read()[0]) == 0 {
                            next_pos = self.read_arg(&x.read()[1]) as usize;
                        }
                        None
                    }
                    Op::LTN => Some(if self.read_arg(&x.read()[0]) < self.read_arg(&x.read()[1]) {
                        1
                    } else {
                        0
                    }),
                    Op::EQL => Some(if self.read_arg(&x.read()[0]) == self.read_arg(&x.read()[1]) {
                        1
                    } else {
                        0
                    }),
                    Op::SP => {
                        self.relative_base = self.relative_base + self.read_arg(&x.read()[0]);
                        None
                    }
                    Op::HLT => {
                        state = State::Halted;
                        None
                    }
                };
                if let Some(out) = v {
                    assert!(x.op.definition().2 == 1);
                    self.write_arg(&x.write()[0], out);
                } else {
                    if x.op.definition().2 != 0 {
                        println!("{:?}", x.op.definition());
                        assert!(x.op.definition().2 == 0);
                    }
                }
                // Update stats
                for r in x.read() {
                    match r {
                        Arg::Position { address } => {
                            *self.reads.entry(*address).or_insert(0) += 1;
                        }
                        Arg::Relative { base, offset } => {
                            *self.reads.entry((base + offset) as usize).or_insert(0) += 1;
                        }
                        _ => {}
                    }
                }
                for w in x.write() {
                    match w {
                        Arg::Position { address } => {
                            *self.writes.entry(*address).or_insert(0) += 1;
                        }
                        Arg::Relative { base, offset } => {
                            *self.writes.entry((base + offset) as usize).or_insert(0) += 1;
                        }
                        _ => {}
                    }
                }
                *self.executes.entry(self.ip).or_insert(0) += 1;
                // Update ip
                self.ip = next_pos;
                state
            }
            Disassembly::MemoryValue(_) => {
                return State::Invalid;
            }
        }
    }

    pub fn get_disassembly(&self, address: usize) -> Disassembly {
        let val = *self.memory.get(address).unwrap_or(&0);
        if let Some(op) = Op::from_i128(val) {
            let def = op.definition();
	    let mut ins = Instruction::new(address, op);
            for r in 0..def.1 {
                let m = mode(val, 1 + r);
                let v = self.memory.get(address + 1 + r).unwrap_or(&0);
                match &m {
                    Mode::Immediate => {
			ins.add_read(Arg::Immediate { value: *v });
                    }
                    Mode::Position => {
                        ins.add_read(Arg::Position {
                            address: *v as usize,
                        });
                    }
                    Mode::Relative => {
                        ins.add_read(Arg::Relative {
                            base: self.relative_base,
                            offset: *v,
                        });
                    }
                }
            }
            for w in 0..def.2 {
                let m = mode(val, 1 + def.1 + w);
                let v = self.memory.get(address + 1 + def.1 + w).unwrap_or(&0);
                match &m {
                    Mode::Position => {
                        ins.add_write(Arg::Position {
                            address: *v as usize,
                        });
                    }
                    Mode::Relative => {
                        ins.add_write(Arg::Relative {
                            base: self.relative_base,
                            offset: *v,
                        });
                    }
                    _ => panic!("OH NOES"),
                }
            }
            Disassembly::Instruction(ins)
        } else {
            Disassembly::MemoryValue(MemoryValue {
                address: address,
                value: val,
            })
        }
    }

    pub fn run_to_next_output(&mut self) -> Option<i128> {
        let res = loop {
            let cont = self.step();
            if let Some(v) = self.outputs.last() {
                assert!(cont == State::Output);
                break Some(*v);
            }
            if cont == State::Halted {
                break None;
            }
        };
        self.outputs.clear();
        res
    }

    pub fn drain_output(&mut self) -> Vec<i128> {
        loop {
            let cont = self.step();
            if cont != State::Output {
                break;
            }
        }
        self.outputs()
    }

    pub fn run_to_next_input(&mut self) -> State {
        loop {
            let s = self.step();
            match s {
                State::Halted => break s,
                State::Input => break s,
                _ => {}
            }
        }
    }

    pub fn run_to_next_io(&mut self) -> State {
        loop {
            let s = self.step();
            match s {
                State::Halted => break s,
                State::Input => break s,
                State::Output => break s,
                _ => {}
            }
        }
    }

    pub fn run(&mut self) {
        while self.step() != State::Halted {}
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
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut m = Machine::with_input(&program, &[0]);
        m.run();
        assert_eq!(m.memory.get(0), Some(&3500));
        m.dump(10);
    }

    #[test]
    fn test_example_1() {
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut m = Machine::with_input(&program, &[6]);
        m.run();
        assert_eq!(m.outputs[0], 0);
        let mut m2 = Machine::with_input(&program, &[8]);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1);
    }

    #[test]
    fn test_example_2() {
        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut m = Machine::with_input(&program, &[6]);
        m.run();
        assert_eq!(m.outputs[0], 1);
        let mut m2 = Machine::with_input(&program, &[8]);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 0);
    }

    #[test]
    fn test_example_3() {
        let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut m = Machine::with_input(&program, &[42]);
        m.run();
        assert_eq!(m.outputs[0], 0);
        let mut m2 = Machine::with_input(&program, &[8]);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1);
    }

    #[test]
    fn test_example_4() {
        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut m = Machine::with_input(&program, &[6]);
        m.run();
        assert_eq!(m.outputs[0], 1);
        let mut m2 = Machine::with_input(&program, &[8]);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 0);
    }

    #[test]
    fn test_jump_1() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut m = Machine::with_input(&program, &[0]);
        m.run();
        assert_eq!(m.outputs[0], 0);
        let mut m2 = Machine::with_input(&program, &[42]);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1);
    }

    #[test]
    fn test_jump_2() {
        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut m = Machine::with_input(&program, &[0]);
        m.run();
        assert_eq!(m.outputs[0], 0);
        let mut m2 = Machine::with_input(&program, &[42]);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1);
    }

    #[test]
    fn test_large_example_1() {
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut m = Machine::with_input(&program, &[6]);
        m.run();
        assert_eq!(m.outputs[0], 999);
        let mut m2 = Machine::with_input(&program, &[8]);
        let _2 = m2.run();
        assert_eq!(m2.outputs[0], 1000);
        let mut m3 = Machine::with_input(&program, &[14]);
        let _3 = m3.run();
        assert_eq!(m3.outputs[0], 1001);
    }

    #[test]
    fn test_quine() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut m = Machine::new(&program);
        m.run();
        assert_eq!(m.outputs, program);
    }

    #[test]
    fn test_bignum() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut m = Machine::new(&program);
        m.run();
        assert_eq!(m.outputs[0], 1219070632396864);
    }

    #[test]
    fn test_bignum2() {
        let program = vec![104, 1125899906842624, 99];
        let mut m = Machine::new(&program);
        m.run();
        assert_eq!(m.outputs[0], 1125899906842624);
    }
}
