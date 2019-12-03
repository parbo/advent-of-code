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

fn read_operand(memory: &[i64], pos: usize) -> Option<&i64> {
    memory.get(*memory.get(pos)? as usize)
}

fn write_operand(memory: &mut [i64], pos: usize, value: i64) {
    // TODO: error handling
    let addr = memory[pos];
    memory[addr as usize] = value;
}

pub fn run(memory: &mut Vec<i64>) -> Option<i64> {
    let mut pos : usize = 0;
    loop {
        if let Some(op) = memory.get(pos).and_then(|&x| Op::from_i64(x)) {
            let size = match op {
                Op::ADD => {
                    let v1 = read_operand(memory, pos + 1)?;
                    let v2 = read_operand(memory, pos + 2)?;
                    let res = v1 + v2;
                    write_operand(memory, pos + 3, res);
                    Some(4)
                },
                Op::MUL => {
                    let v1 = read_operand(memory, pos + 1)?;
                    let v2 = read_operand(memory, pos + 2)?;
                    let res = v1 * v2;
                    write_operand(memory, pos + 3, res);
                    Some(4)
               }
                Op::HLT => {
                    None
                }
            };
            if let Some(increment) = size {
                pos += increment;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    Some(memory[0])
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test() {
        let mut input = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        assert_eq!(run(&mut input), Some(3500));
    }
}
