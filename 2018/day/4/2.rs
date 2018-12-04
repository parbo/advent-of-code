use std::collections::HashMap;
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
        let t : Vec<i64> = line[1..17].split(|c| ['-', ' ', ':'].contains(&c)).filter(|s| s.len() > 0).map(|v| v.parse::<i64>().unwrap()).collect();
        rows.push((t, line[19..].to_string()));
    }
    rows.sort_unstable();
    let mut sleeps = HashMap::new();
    let mut guard = -1;
    let mut sleep = -1;
    for r in rows {
        let (t, line) = r;
        if let Some(_) = line.find("wakes") {
            let wake = t[4];
            let m = sleeps.entry(guard).or_insert(HashMap::<i64, i64>::new());
            for i in sleep..wake {
                *m.entry(i).or_insert(0) += 1;
            }
        } else if let Some(_) = line.find("falls") {
            sleep = t[4];
        } else if let Some(_) = line.find("Guard") {
            let g : Vec<i64> = line.split(|c| [' ', '#'].contains(&c)).filter(|s| s.len() > 0).skip(1).take(1).map(|v| v.parse::<i64>().unwrap()).collect();
            guard = g[0];
        }
    }
    let mut max_minute = -1;
    let mut max_count = -1;
    let mut max_guard = -1;
    for (guard, s) in &sleeps {
        for (minute, count) in s {
            if count > &max_count {
                max_count = *count;
                max_minute = *minute;
                max_guard = *guard;
            }
        }
    }
    println!("{:?}, {:?}, {:?}", max_guard, max_minute, max_count);
    return max_guard * max_minute;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{}", result);
}
