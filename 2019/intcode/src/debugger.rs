use crate::machine::*;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Debugger<'a> {
    machine: &'a mut Machine
}

impl Debugger<'_> {
    pub fn new<'a>(machine: &'a mut Machine) -> Debugger<'a> {
        Debugger { machine }
    }

    fn print_instruction(&self, a: usize) -> usize {
        match self.machine.get_disassembly(a) {
            Disassembly::Instruction(x) => {
                println!("{}", x);
                a + x.increment()
            },
            Disassembly::MemoryValue(x) => {
                println!("{}", x);
                a + 1
            },
        }
    }

    fn print_memory(&self, address: usize, count: usize) {
        self.machine.memory()
            .iter()
            .enumerate()
            .skip(address)
            .take(count)
            .for_each(|(a, &v)| println!("{:>04}, {}", a, v));
    }

    fn print_instructions(&self, address: usize, count: usize) {
        let mut addr = address;
        for _ in 0..count {
            addr = self.print_instruction(addr);
        }
    }

    pub fn debug(&mut self) {
        // `()` can be used when no completer is required
        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        let _ = self.print_instruction(self.machine.ip());
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    if line == "s" {
                        if !self.machine.step() {
                            println!("Program halted");
                        } else {
                            let _ = self.print_instruction(self.machine.ip());
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
