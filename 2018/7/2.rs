use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

struct Work {
    ongoing : HashMap<char, i64>,
    done : HashSet<char>,
    reqs : HashMap<char, Vec<char>>
}

impl Work {
    fn new(rows: Vec<(char, char)>) -> Work {
        let mut reqs = HashMap::new();
        for (before, after) in rows {
            reqs.entry(after).or_insert(Vec::new()).push(before);
            reqs.entry(before).or_insert(Vec::new());
        }
        Work {
            ongoing: HashMap::<char, i64>::new(),
            done: HashSet::<char>::new(),
            reqs: reqs
        }
    }

    fn pull(&mut self) -> bool {
        let mut noreqs : Vec<char> = self.reqs.iter()
            .filter(|r| !self.ongoing.contains_key(r.0))  // filter ongoing ops
            .filter(|r| r.1.iter().filter(|c| !self.done.contains(&c)).count() == 0)  // filter done reqs
            .map(|r| *r.0).collect();
        noreqs.sort_unstable();
        if noreqs.len() > 0 {
            let op = noreqs[0];
            self.ongoing.insert(op, (op as i64) - ('A' as i64) + 60);
            return true;
        }
        return false;
    }

    fn work(&mut self) -> Vec<char> {
        let mut d = vec![];
        let og = &mut self.ongoing;
        for w in og {
            if *w.1 == 0 {
                d.push(*w.0);
            } else {
                *w.1 -= 1;
            }
        }
        return d;
    }

    fn done(&mut self, d: Vec<char>) {
        for c in &d {
            self.done.insert(*c);
            self.ongoing.remove(c);
            self.reqs.remove(c);
        }
    }
}

fn solve(path: &Path) -> i64 {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut rows = vec![];
    for line in lines {
        let t : Vec<char> = line.split(|c| c == ' ').filter(|s| s.len() == 1).map(|s| s.chars().next().unwrap()).collect();
        rows.push((t[0], t[1]));
    }
    let mut w = Work::new(rows);
    let mut t = 0;
    let workers = 5;
    while w.reqs.len() > 0 {
        while w.ongoing.len() < workers {
            if !w.pull() {
                break;
            }
        }
        let d = w.work();
        w.done(d);
        t += 1;
    }
    return t;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{}", result);
}
