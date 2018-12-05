use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn is_pair(a: u8, b: u8) -> bool {
   (a as i8 - b as i8).abs() == 32
}

fn solve(path: &Path) -> i64 {
    let mut input = File::open(path).unwrap();
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer).unwrap();

    let mut out = Vec::new();
    let mut i = 0;
    while i < buffer.len() {
        if i + 1 < buffer.len() {
            if is_pair(buffer[i], buffer[i + 1]) {
                i += 2;
                continue;
            }
        }
        let n = out.len();
        if n > 0 {
            if is_pair(out[n-1], buffer[i]) {
                out.pop();
                i += 1;
                continue;
            }
        }
        if buffer[i] == '\r' as u8 || buffer[i] == '\n' as u8 {
            i += i;
            continue;
        }
        out.push(buffer[i]);
        i += 1;
    }
    let len = out.len();
    let s = String::from_utf8(out);
    println!("{:?}", s);
    return len as i64;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{}", result);
}
