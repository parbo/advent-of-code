use std::collections::HashSet;

type Parsed = Vec<i128>;

fn init_network(program: &[i128], len: i128) -> Vec<intcode::Machine> {
    let mut machines = Vec::new();
    for i in 0..len {
        let mut m = intcode::Machine::new(program);
        m.run_to_next_input();
        m.add_input(i);
        m.step();
        machines.push(m);
    }
    machines
}

fn run_network(
    machines: &mut [intcode::Machine],
    callback: &mut dyn FnMut((i128, i128)) -> bool,
) -> bool {
    let mut any_sent = false;
    let mut input_count = 0;
    for i in 0..machines.len() {
        let state = machines[i].run_to_next_io();
        match state {
            intcode::State::Output => {
                any_sent = true;
                let outputs = machines[i].outputs();
                let to = outputs[0];
                let x = machines[i].run_to_next_output().unwrap();
                let y = machines[i].run_to_next_output().unwrap();
                if to == 255 {
                    if !(callback)((x, y)) {
                        break;
                    }
                } else {
                    machines[to as usize].add_input(x);
                    machines[to as usize].add_input(y);
                }
            }
            intcode::State::Input => {
                input_count += 1;
                machines[i].add_input(-1);
            }
            intcode::State::Halted => break,
            _ => panic!(),
        }
    }
    // Idle?
    !any_sent && input_count == 50
}

fn part1(program: &Parsed) -> i128 {
    let mut machines = init_network(program, 50);
    let mut ans = None;
    loop {
        run_network(&mut machines, &mut |(_x, y)| {
            ans = Some(y);
            false
        });
        if ans.is_some() {
            break;
        }
    }
    ans.unwrap()
}

fn part2(program: &Parsed) -> i128 {
    let mut machines = init_network(program, 50);
    let mut nat = (0, 0);
    let mut seen = HashSet::new();
    loop {
        let idle = run_network(&mut machines, &mut |m| {
            nat = m;
            true
        });
        if idle {
            if seen.insert(nat.1) {
                // send nat to 0
                machines[0].add_input(nat.0);
                machines[0].add_input(nat.1);
            } else {
                return nat.1;
            }
        }
    }
}

fn main() {
    aoc::run_main(intcode::parse_intcode, part1, part2);
}
