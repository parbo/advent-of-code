use aoc;
use intcode;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (program, input) = if args.len() > 1 {
        let filename = &args[1];
        let f = File::open(Path::new(filename)).unwrap();
        let buffered = BufReader::new(f);
        (
            aoc::parse_intcode(
                &buffered
                    .lines()
                    .filter_map(Result::ok)
                    .map(|x| x.trim().to_string())
                    .collect(),
            ),
            if args.len() > 2 {
                args[2]
                    .split(|c| c == ',')
                    .map(|s| s.trim())
                    .map(|v| v.parse::<i128>().unwrap())
                    .collect()
            } else {
                vec![]
            },
        )
    } else {
        (
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![6],
        )
    };
    let mut m = intcode::Machine::new(&program, &input);
    let mut debugger = intcode::Debugger::new(&mut m);
    debugger.debug();
}
