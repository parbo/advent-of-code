use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

fn solve(path: &Path) -> i64 {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut rows = vec![];
    for line in lines {
        let t : Vec<char> = line.split(|c| c == ' ').filter(|s| s.len() == 1).map(|s| s.chars().next().unwrap()).collect();
        rows.push((t[0], t[1]));
    }
    let mut reqs = HashMap::new();
    let mut letters = HashSet::new();
    for (before, after) in rows {
        reqs.entry(after).or_insert(Vec::new()).push(before);
        letters.insert(before);
        letters.insert(after);
    }
    for l in &letters {
        reqs.entry(*l).or_insert(Vec::<char>::new());
    }
    let mut res : Vec<char> = vec![];
    while reqs.len() > 0 {
        let mut noreqs : Vec<char> = reqs.iter().filter(|r| r.1.iter().filter(|c| !res.contains(&c)).count() == 0).map(|r| *r.0).collect();
        noreqs.sort_unstable();
        println!("{:?}", reqs);
        println!("{:?}", noreqs);
        if noreqs.len() == 0 {
            break;
        }
        let op = noreqs[0];
        res.push(op);
        reqs.remove(&op);
    }
    let s : String = res.into_iter().collect();
    println!("{}", s);
    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{}", result);
}
