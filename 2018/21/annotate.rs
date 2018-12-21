use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut program = vec![];
    let mut pc_reg = 0;
    for line in lines {
        if line[0..3] == *"#ip" {
            pc_reg = line[4..].parse::<usize>().unwrap();
        } else {
            let op = line[0..4].trim().to_string();
            let args : Vec<usize> = line[5..].split(|c| c == ' ').map(|s| s.trim().parse::<usize>().unwrap()).collect();
            program.push((op, args[0], args[1], args[2]));
        }
    }

    println!("#include <stdio.h>");
    println!("int main() {{");
    println!("  int pc = 0, a = 0, b = 0, c = 0, d = 0, e = 0, f = 0;");
    println!("  do {{");
    println!("    switch (pc) {{");
    let mut vars = vec!["a", "b", "c", "d", "e"];
    vars.insert(pc_reg, "pc");
    for (r, (op, a, b, c)) in program.iter().enumerate() {
        print!("      case {}: ", r);
        match &op[..] {
            "addr" => {
                println!("{} = {} + {};", vars[*c], vars[*a], vars[*b]);
            },
            "addi" => {
                println!("{} = {} + {};", vars[*c], vars[*a], b);
            },
            "mulr" => {
                println!("{} = {} * {};", vars[*c], vars[*a], vars[*b]);
            },
            "muli" => {
                println!("{} = {} * {};", vars[*c], vars[*a], b);
            },
            "banr" => {
                println!("{} = {} & {};", vars[*c], vars[*a], vars[*b]);
            },
            "bani" => {
                println!("{} = {} & {};", vars[*c], vars[*a], b);
            },
            "borr" => {
                println!("{} = {} | {};", vars[*c], vars[*a], vars[*b]);
            },
            "bori" => {
                println!("{} = {} | {};", vars[*c], vars[*a], b);
            },
            "setr" => {
                println!("{} = {};", vars[*c], vars[*a]);
            },
            "seti" => {
                println!("{} = {};", vars[*c], a);
            },
            "gtir" => {
                println!("{} = {} > {} ? 1 : 0;", vars[*c], a, vars[*b]);
            },
            "gtri" => {
                println!("{} = {} > {} ? 1 : 0;", vars[*c], vars[*a], b);
            },
            "gtrr" => {
                println!("{} = {} > {} ? 1 : 0;", vars[*c], vars[*a], vars[*b]);
            },
            "eqir" => {
                println!("{} = {} == {} ? 1 : 0;", vars[*c], a, vars[*b]);
            },
            "eqri" => {
                println!("{} = {} == {} ? 1 : 0;", vars[*c], vars[*a], b);
            },
            "eqrr" => {
                println!("{} = {} == {} ? 1 : 0;", vars[*c], vars[*a], vars[*b]);
            }
            _ => {
            }
        }
        println!("        break;");
    }
    println!("    }}");
    println!("    ++pc;");
    println!("  }} while (pc < {});", program.len());
    println!("  printf(\"{}: %d\\n\", {});", vars[pc_reg], vars[pc_reg]);
    for i in 0..6 {
        if i == pc_reg {
            continue;
        }
        println!("  printf(\"{}: %d\\n\", {});", vars[i], vars[i]);
    }
    println!("}}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
