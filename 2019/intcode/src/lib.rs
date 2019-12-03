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
    ip: usize
}

impl Machine {
    pub fn new(memory: &Vec<i64>) -> Machine {
        Machine { memory: memory.clone(), ip: 0 }
    }

    pub fn write(&mut self, pos: usize, value: i64) {
        self.memory[pos] = value;
    }

    fn read_operand(&self, pos: usize) -> Option<&i64> {
        self.memory.get(*self.memory.get(pos)? as usize)
    }

    fn write_operand(&mut self, pos: usize, value: i64) {
        // TODO: error handling
        let addr = self.memory[pos];
        self.memory[addr as usize] = value;
    }

    // Returns instruction size
    pub fn step(&mut self) -> Option<usize> {
        let pos = self.ip;
        let op = self.memory.get(pos).and_then(|&x| Op::from_i64(x))?;
        match op {
            Op::ADD => {
                let v1 = self.read_operand(pos + 1)?;
                let v2 = self.read_operand(pos + 2)?;
                let res = v1 + v2;
                self.write_operand(pos + 3, res);
                Some(4)
            },
            Op::MUL => {
                let v1 = self.read_operand(pos + 1)?;
                let v2 = self.read_operand(pos + 2)?;
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
}

#[cfg(test)]
mod tests {
    use super::Machine;

    #[test]
    fn test() {
        let input = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let mut m = Machine::new(&input);
        assert_eq!(m.run(), Some(3500));
    }
}
