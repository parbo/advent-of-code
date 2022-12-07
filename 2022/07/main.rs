use std::{
    collections::{HashMap, HashSet},
    iter::*,
    path::PathBuf,
};

type Parsed = (HashMap<PathBuf, HashSet<String>>, HashMap<PathBuf, i64>);
type Answer = i64;

fn acc_size(
    dirs: &HashMap<PathBuf, HashSet<String>>,
    files: &HashMap<PathBuf, i64>,
    acc: &mut HashMap<PathBuf, i64>,
    curr: PathBuf,
) -> i64 {
    let mut sz = 0;
    if let Some(contents) = dirs.get(&curr) {
        for c in contents {
            let p = curr.join(c);
            if let Some(fsz) = files.get(&p) {
                sz += fsz;
            } else {
                // Recurse
                sz += acc_size(dirs, files, acc, p);
            }
        }
    }
    acc.insert(curr, sz);
    sz
}

fn part1(data: &Parsed) -> Answer {
    let (dirs, files) = data;
    let mut acc: HashMap<PathBuf, i64> = HashMap::new();
    acc_size(dirs, files, &mut acc, PathBuf::from("/"));
    acc.values().filter(|x| **x <= 100000).sum()
}

fn part2(data: &Parsed) -> Answer {
    let (dirs, files) = data;
    let mut acc: HashMap<PathBuf, i64> = HashMap::new();
    acc_size(dirs, files, &mut acc, PathBuf::from("/"));
    let used = acc.get(&PathBuf::from("/")).unwrap();
    let free = 70000000 - used;
    let to_free = 30000000 - free;
    acc.into_iter()
        .filter(|(_p, sz)| *sz >= to_free)
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1
}

fn parse(lines: &[String]) -> Parsed {
    let mut dirs: HashMap<PathBuf, HashSet<String>> = HashMap::new();
    let mut files: HashMap<PathBuf, i64> = HashMap::new();
    let mut currdir = PathBuf::from("/");
    dirs.insert(currdir.clone(), HashSet::new());
    let mut in_ls = false;
    for line in lines {
        if in_ls {
            if line.starts_with('$') {
                in_ls = false;
            } else if line.starts_with("dir") {
                let d = &line[4..];
                dirs.entry(currdir.clone()).or_default().insert(d.to_string());
                dirs.entry(PathBuf::from(d)).or_default();
            } else {
                let parts = aoc::split_w(line);
                dirs.entry(currdir.clone())
                    .or_default()
                    .insert(parts[1].to_string());
                let fp = currdir.join(parts[1]);
                dirs.entry(fp.clone()).or_default();
                files.insert(fp, parts[0].parse::<i64>().unwrap());
            }
        }

        if !in_ls {
            if line.starts_with("$ cd") {
                let dest = &line[5..];
                if dest == ".." {
                    currdir.pop();
                } else if dest == "/" {
                    currdir = PathBuf::from("/");
                } else {
                    dirs.entry(currdir.clone())
                        .or_default()
                        .insert(dest.to_string());
                    currdir.push(dest);
                }
            } else if line.starts_with("$ ls") {
                in_ls = true;
            } else {
                unreachable!();
            }
        }
    }
    (dirs, files)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "$ cd /".into(),
            "$ ls".into(),
            "dir a".into(),
            "14848514 b.txt".into(),
            "8504156 c.dat".into(),
            "dir d".into(),
            "$ cd a".into(),
            "$ ls".into(),
            "dir e".into(),
            "29116 f".into(),
            "2557 g".into(),
            "62596 h.lst".into(),
            "$ cd e".into(),
            "$ ls".into(),
            "584 i".into(),
            "$ cd ..".into(),
            "$ cd ..".into(),
            "$ cd d".into(),
            "$ ls".into(),
            "4060174 j".into(),
            "8033020 d.log".into(),
            "5626152 d.ext".into(),
            "7214296 k".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 95437);
    }
}
