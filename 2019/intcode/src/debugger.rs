use crate::machine::*;
use std::collections::HashMap;
use std::collections::HashSet;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use ansi_term::Colour;

pub struct Debugger<'a> {
    machine: &'a mut Machine,
    breakpoints: HashSet<usize>,
    watches: HashMap<String, usize>,
}

fn config() {
    #[cfg(target_os = "windows")]
    let _ = ansi_term::enable_ansi_support();
}

impl Debugger<'_> {
    pub fn new(machine: &mut Machine) -> Debugger<'_> {
        config();
        Debugger {
            machine,
            breakpoints: HashSet::new(),
            watches: HashMap::new(),
        }
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
                        Op::SP => {
                            if let Some(Arg::Immediate { value }) = &x.read().first() {
                                if value.is_positive() {
                                    sp = *value;
                                    sp_addr = a;
                                } else if *value == -sp {
                                    sp = 0;
                                }
                            }
                        }
                        Op::JIT => {
                            if let Some(Arg::Immediate { value: 1 }) = &x.read().first() {
                                if let Some(Arg::Relative { base: _, offset: 0 }) = &x.read().get(1)
                                {
                                    println!("found function {} -> {}", sp_addr, a);
                                }
                            }
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
                print!(
                    "{} SP:{}, IP:",
                    if self.breakpoints.contains(&a) {
                        "*"
                    } else {
                        " "
                    },
                    Colour::Green.paint(format!("{:04}", self.machine.sp()))
                );
                print!("{}", x);
                print!(" ; ");
                for r in x.read() {
                    print!("{}, ", self.machine.read_arg(r));
                }
                if !x.write().is_empty() {
                    print!("-> ");
                }
                for w in x.write() {
                    print!("{}, ", self.machine.read_arg(w));
                }
                println!();
                if print_stack {
                    if self.machine.sp() > 0 {
                        self.print_memory(self.machine.sp(), 16);
                    }
                    self.list_watches();
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
        if !count.is_multiple_of(8) {
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

    fn set_breakpoint(&mut self, address: usize) {
        if self.breakpoints.insert(address) {
            println!("Breakpoint set on address {}", address);
        } else {
            println!("Breakpoint already set on address {}", address);
        }
    }

    fn list_breakpoints(&self) {
        let mut bp: Vec<_> = self.breakpoints.iter().collect();
        bp.sort();
        for addr in bp {
            println!("b {}", addr);
        }
    }

    fn clear_breakpoint(&mut self, address: usize) {
        if self.breakpoints.remove(&address) {
            println!("Breakpoint removed on address {}", address);
        } else {
            println!("No breakpoint on address {}", address);
        }
    }

    fn set_watch(&mut self, name: &str, address: usize) {
        if let Some(v) = self.watches.insert(name.into(), address) {
            println!(
                "Set address {} for watch {} (previous address: {})",
                address, name, v
            );
        } else {
            println!("Added watch {} on address {}", name, address);
        }
    }

    fn list_watches(&self) {
        for (name, a) in &self.watches {
            let v = *self.machine.memory().get(*a).unwrap_or(&0);
            println!("w: {:8} - {:>04}: {}", name, a, v);
        }
    }

    fn clear_watch(&mut self, name: &str) {
        if let Some(v) = self.watches.remove(name) {
            println!("Watch {} - {} removed", name, v);
        } else {
            println!("No watch with name {}", name);
        }
    }

    fn print_output(&mut self) {
        for o in self.machine.outputs() {
            if (0..256).contains(&o) {
                print!("{}", std::char::from_u32(o as u32).unwrap());
            } else {
                println!("value: {}", o);
            }
        }
    }

    fn handle_io(&mut self, rl: &mut Editor<()>, state: State) {
        if state == State::Output {
            self.print_output();
        }
        if state == State::Input {
            if let Ok(s) = rl.readline("INP >> ") {
                let x = s.trim();
                for c in x.chars() {
                    self.machine.add_input(c as i128);
                }
                self.machine.add_input(10);
            }
        }
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
                    #[allow(clippy::unnecessary_unwrap)]
                    if line.is_empty() && last.is_some() {
                        line = last.unwrap();
                    }
                    if line == "s" {
                        let state = self.machine.step();
                        self.handle_io(&mut rl, state);
                        if state == State::Halted {
                            println!("Program halted");
                        } else {
                            let _ = self.print_instruction(self.machine.ip(), true);
                        }
                    } else if line.starts_with('b') {
                        if line == "bl" {
                            self.list_breakpoints();
                        } else if let Some(stripped) = line.strip_prefix("bc") {
                            if let Ok(addr) = stripped.trim().parse::<usize>() {
                                self.clear_breakpoint(addr);
                            } else {
                                self.clear_breakpoint(self.machine.ip());
                            }
                        } else if let Some(stripped) = line.strip_prefix('b') {
                            if let Ok(addr) = stripped.trim().parse::<usize>() {
                                self.set_breakpoint(addr);
                            } else {
                                self.set_breakpoint(self.machine.ip());
                            }
                        }
                    } else if line.starts_with('w') {
                        if line.starts_with("wr ") {
                            let parts: Vec<_> = line.split(' ').map(|x| x.trim()).collect();
                            let addr = parts[1].parse::<usize>().unwrap();
                            let val = parts[2].parse::<i128>().unwrap();
                            if let Some(x) = self.machine.memory_mut().get_mut(addr) {
                                *x = val;
                            } else {
                                println!("Invalid address!");
                            }
                        } else if line == "wl" {
                            self.list_watches();
                        } else if let Some(stripped) = line.strip_prefix("wc") {
                            self.clear_watch(stripped);
                        } else {
                            let parts: Vec<_> = line.split(' ').map(|x| x.trim()).collect();
                            let name = parts[1];
                            let addr = parts[2].parse::<usize>().unwrap();
                            self.set_watch(name, addr);
                        }
                    } else if line == "c" {
                        loop {
                            let state = self.machine.step();
                            self.handle_io(&mut rl, state);
                            if self.breakpoints.contains(&self.machine.ip()) {
                                println!("Breakpoint reached");
                                break;
                            }
                            if state == State::Halted {
                                println!("Program halted");
                                break;
                            }
                        }
                        let _ = self.print_instruction(self.machine.ip(), true);
                    } else if let Some(stripped) = line.strip_prefix('p') {
                        if let Ok(addr) = stripped.trim().parse::<usize>() {
                            self.print_memory(addr, 8);
                        } else {
                            self.print_memory(self.machine.ip(), 8);
                        }
                    } else if line == "m" {
                        self.print_memory(self.machine.ip(), 8);
                    } else if line == "ds" {
                        self.machine.dump(5);
                    } else if let Some(stripped) = line.strip_prefix('l') {
                        if let Ok(lines) = stripped.trim().parse::<usize>() {
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
