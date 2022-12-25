use aoc;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::*;

// TODO: improve and generalize
pub struct Tree {
    things: HashSet<String>,
    children: HashMap<String, Vec<String>>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            things: HashSet::new(),
            children: HashMap::new(),
        }
    }

    pub fn things(&self) -> impl Iterator<Item = &String> {
        self.things.iter()
    }

    pub fn insert(&mut self, parent: &str, child: &str) {
        self.things.insert(parent.to_string());
        self.things.insert(child.to_string());
        self.children
            .entry(parent.to_string())
            .or_insert(Vec::new())
            .push(child.to_string());
    }

    pub fn depth_from_to(&self, from: &str, to: &str) -> Option<i64> {
        self.depth_from_to_recursive(from, to, 0)
    }

    fn depth_from_to_recursive(&self, from: &str, to: &str, depth: i64) -> Option<i64> {
        if from == to {
            return Some(depth);
        }
        if let Some(v) = self.children.get(from) {
            for t in v {
                if let Some(x) = self.depth_from_to_recursive(t, to, depth + 1) {
                    return Some(x);
                }
            }
        }
        return None;
    }
}

fn part1(tree: &Tree) -> i64 {
    tree.things()
        .map(|t| tree.depth_from_to("COM", &t))
        .sum::<Option<i64>>()
        .unwrap()
}

fn part2(tree: &Tree) -> i64 {
    tree.things()
        .filter_map(|t| {
            [tree.depth_from_to(t, "YOU"), tree.depth_from_to(t, "SAN")]
                .iter()
                .map(|&x| x)
                .sum::<Option<i64>>()
        })
        .min()
        .unwrap()
        - 2
}

fn parse(lines: &[String]) -> Tree {
    let mut tree = Tree::new();
    lines
        .iter()
        .map(|line| line.split(')').collect())
        .for_each(|orbit: Vec<_>| tree.insert(orbit[0], orbit[1]));
    tree
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::{parse, part1, part2};

    #[test]
    fn test_part1() {
        let lines: Vec<String> = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ]
        .iter()
        .map(|&x| x.into())
        .collect();
        let d = parse(&lines);
        assert_eq!(part1(&d), 42);
    }

    #[test]
    fn test_part2() {
        let lines: Vec<String> = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ]
        .iter()
        .map(|&x| x.into())
        .collect();
        let d = parse(&lines);
        assert_eq!(part2(&d), 4);
    }
}
