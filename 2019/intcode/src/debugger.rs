use crate::machine::*;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Debugger<'a> {
    machine: &'a mut Machine,
}

impl Debugger<'_> {
    pub fn new<'a>(machine: &'a mut Machine) -> Debugger<'a> {
        Debugger { machine }
    }

    pub fn analyze(&self) {
        let mut a = 0;
        let mut sp = 0;
        let mut sp_addr = 0;
        loop {
            a = match self.machine.get_disassembly(a) {
                Disassembly::Instruction(x) => {
                    match x.op {
                        // Find function
                        Op::SP => match &x.read[0] {
                            Arg::Immediate { value } => {
                                if value.is_positive() {
                                    sp = *value;
                                    sp_addr = a;
                                } else {
                                    if *value == -1 * sp {
                                        sp = 0;
                                    }
                                }
                            },
                            _ => {}
                        },
                        Op::JIT => match &x.read[0] {
                            Arg::Immediate { value: 1 } => {
                                match &x.read[1] {
                                    Arg::Relative { base: _, offset: 0 } => {
                                        println!("found function {} -> {}", sp_addr, a);
                                    },
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                        _ => {}
                    }
                    a + x.increment()
                }
                Disassembly::MemoryValue(_x) => {
                    println!("variable at {}", a);
                    a + 1
                }
            };
            if a >= self.machine.memory().len() {
                break;
            }
        }
    }

    fn print_instruction(&self, a: usize, print_stack: bool) -> usize {
        match self.machine.get_disassembly(a) {
            Disassembly::Instruction(x) => {
                print!("SP:{:04}, IP:", self.machine.sp());
                print!("{}", x);
                print!(" ; ");
                for r in &x.read {
                    print!("{}, ", self.machine.read_arg(r));
                }
                if x.write.len() > 0 {
                    print!("-> ");
                }
                for w in &x.write {
                    print!("{}, ", self.machine.read_arg(w));
                }
                println!();
                if self.machine.sp() > 0 && print_stack {
                    self.print_memory(self.machine.sp(), 16);
                }
                a + x.increment()
            }
            Disassembly::MemoryValue(x) => {
                println!("{}", x);
                a + 1
            }
        }
    }

    fn print_memory(&self, address: usize, count: usize) {
        for (i, a) in (address..(address + count)).enumerate() {
            let v = *self.machine.memory().get(a).unwrap_or(&0);
            if (i % 8) == 0 {
                print!("{:>8}: ", a);
            }
            print!("{:>8},", v);
            if (i % 8) == 7 {
                println!();
            }
        }
        if (count % 8) != 0 {
            println!();
        }
    }

    fn print_instructions(&self, address: usize, count: usize) {
        let mut addr = address;
        for _ in 0..count {
            addr = self.print_instruction(addr, false);
        }
        self.print_memory(self.machine.sp(), 16);
    }

    pub fn debug(&mut self) {
        // `()` can be used when no completer is required
        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        let _ = self.print_instruction(self.machine.ip(), false);
        let mut last = None;
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(l) => {
                    rl.add_history_entry(l.as_str());
                    let mut line = l;
                    if line == "" && last.is_some() {
                        line = last.unwrap();
                    }
                    if line == "s" {
                        if !self.machine.step() {
                            println!("Program halted");
                        } else {
                            let _ = self.print_instruction(self.machine.ip(), true);
                        }
                    } else if line == "c" {
                        let _ = self.machine.run();
                        println!("Program halted");
                    } else if line.starts_with("p") {
                        if let Ok(addr) = line[1..].trim().parse::<usize>() {
                            self.print_memory(addr, 8);
                        } else {
                            self.print_memory(self.machine.ip(), 8);
                        }
                    } else if line.starts_with("w ") {
                        let parts: Vec<_> = line.split(' ').map(|x| x.trim()).collect();
                        let addr = parts[1].parse::<usize>().unwrap();
                        let val = parts[2].parse::<i128>().unwrap();
                        if let Some(x) = self.machine.memory_mut().get_mut(addr) {
                            *x = val;
                        } else {
                            println!("Invalid address!");
                        }
                    } else if line == "m" {
                        self.print_memory(self.machine.ip(), 8);
                    } else if line == "ds" {
                        self.machine.dump(5);
                    } else if line.starts_with("l") {
                        if let Ok(lines) = line[1..].trim().parse::<usize>() {
                            self.print_instructions(self.machine.ip(), lines);
                        } else {
                            self.print_instructions(self.machine.ip(), 8);
                        }
                    } else {
                        println!("Invalid command: {}", line);
                    }
                    last = Some(line);
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
}
