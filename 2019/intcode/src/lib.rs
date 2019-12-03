use std::collections::HashMap;

enum Op {
    ADD = 1,
    MUL = 2,
    HLT = 99
}


impl Op {
    fn from_i64(value: i64) -> Option<Op> {
        match value {
            1 => Some(Op::ADD),
            2 => Some(Op::MUL),
            99 => Some(Op::HLT),
            _ => None
        }
    }
}

pub struct Machine {
    memory: Vec<i64>,
    ip: usize,
    executes: HashMap<usize, usize>,
    reads: HashMap<usize, usize>,
    writes: HashMap<usize, usize>,
}

impl Machine {
    pub fn new(memory: &Vec<i64>) -> Machine {
        Machine { memory: memory.clone(), ip: 0, executes: HashMap::new(), reads: HashMap::new(), writes: HashMap::new() }
    }

    pub fn write(&mut self, pos: usize, value: i64) {
        self.memory[pos] = value;
    }

    fn read_operand(&mut self, pos: usize) -> Option<&i64> {
        let addr = *self.memory.get(pos)? as usize;
        *self.reads.entry(addr).or_insert(0) += 1;
        self.memory.get(addr)
    }

    fn write_operand(&mut self, pos: usize, value: i64) {
        // TODO: error handling
        let addr = self.memory[pos] as usize;
        *self.writes.entry(addr).or_insert(0) += 1;
        self.memory[addr] = value;
    }

    // Returns instruction size
    pub fn step(&mut self) -> Option<usize> {
        let pos = self.ip;
        *self.executes.entry(pos).or_insert(0) += 1;
        let op = self.memory.get(pos).and_then(|&x| Op::from_i64(x))?;
        match op {
            Op::ADD => {
                let v1 = *self.read_operand(pos + 1)?;
                let v2 = *self.read_operand(pos + 2)?;
                let res = v1 + v2;
                self.write_operand(pos + 3, res);
                Some(4)
            },
            Op::MUL => {
                let v1 = *self.read_operand(pos + 1)?;
                let v2 = *self.read_operand(pos + 2)?;
                let res = v1 * v2;
                self.write_operand(pos + 3, res);
                Some(4)
            }
            Op::HLT => {
                None
            }
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

    pub fn dump(&self, n: usize) {
        let mut exec_vec: Vec<(&usize, &usize)> = self.executes.iter().collect();
        exec_vec.sort_by(|a, b| b.1.cmp(a.1));
        let mut read_vec: Vec<(&usize, &usize)> = self.reads.iter().collect();
        read_vec.sort_by(|a, b| b.1.cmp(a.1));
        let mut write_vec: Vec<(&usize, &usize)> = self.writes.iter().collect();
        write_vec.sort_by(|a, b| b.1.cmp(a.1));
        println!("Executed:");
        exec_vec.iter().take(n).for_each(|x| println!("  {} - {}", x.0, x.1));
        println!("Read:");
        read_vec.iter().take(n).for_each(|x| println!("  {} - {}", x.0, x.1));
        println!("Written:");
        write_vec.iter().take(n).for_each(|x| println!("  {} - {}", x.0, x.1));
    }
}

#[cfg(test)]
mod tests {
    use super::Machine;

    #[test]
    fn test() {
        let input = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let mut m = Machine::new(&input);
        assert_eq!(m.run(), Some(3500));
        m.dump(10);
    }
}
