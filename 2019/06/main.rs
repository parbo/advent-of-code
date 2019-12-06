use aoc;
use aoc::Tree;
use std::iter::*;

fn part1(tree: &Tree) -> i64 {
    tree.things()
        .map(|t| tree.depth_from_to("COM", &t))
        .sum::<Option<i64>>()
        .unwrap()
}

fn part2(tree: &Tree) -> i64 {
    tree.things()
        .filter_map(|t| {
            [tree.depth_from_to(t, "YOU"), tree.depth_from_to(t, "SAN")].iter().map(|&x| x).sum::<Option<i64>>()
        })
        .min()
        .unwrap()
        - 2
}

fn parse(lines: &Vec<String>) -> Tree {
    let mut tree = Tree::new();
    lines
        .iter()
        .map(|line| line.split(')').collect())
        .for_each(|orbit: Vec<_>| tree.insert(orbit[0], orbit[1]));
    tree
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
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
