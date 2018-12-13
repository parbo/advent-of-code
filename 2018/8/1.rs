use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

#[derive(Debug)]
struct Node {
    id: i64,
    children: Vec<i64>,
    metadata: Vec<i64>
}

fn read_nodes(i: i64, numbers: &Vec<i64>) -> (i64, Vec<Node>) {
    let num_children = numbers[i as usize];
    let num_metadata = numbers[(i + 1) as usize];
    let mut x = i + 2;
    let mut nodes = vec![];
    let mut children = vec![];
    for _ in 0..num_children {
        children.push(x);
        let res = read_nodes(x, numbers);
        nodes.extend(res.1);
        x = res.0;
    }
    let mut metadata = vec![];
    for z in x..(x+num_metadata) {
        metadata.push(numbers[z as usize]);
    }
    nodes.push(Node {id: i, children: children, metadata: metadata});
    return (x+num_metadata, nodes);
}

fn solve(path: &Path) -> i64 {
    let mut input = File::open(path).unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let numbers : Vec<i64> = buffer.split(|c| c == ' ').map(|s| s.trim()).map(|v| v.parse::<i64>().unwrap()).collect();

    let (_, nodes) = read_nodes(0, &numbers);
    return nodes.iter().flat_map(|n| n.metadata.iter()).sum();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{}", result);
}
