mod cling;
mod debugger;
mod machine;

pub use cling::*;
pub use debugger::Debugger;
pub use machine::Machine;
pub use machine::State;

pub fn parse_intcode(lines: &Vec<String>) -> Vec<i128> {
    let result: Vec<i128> = lines[0]
        .split(|c| c == ',')
        .map(|s| s.trim())
        .map(|v| v.parse::<i128>().unwrap())
        .collect();
    result
}
