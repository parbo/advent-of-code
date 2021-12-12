use std::collections::BTreeSet;
use std::iter::*;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
enum Node {
    Start,
    End,
    Small(String),
    Large(String),
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Node {
    type Err = aoc::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Node::Start),
            "end" => Ok(Node::End),
            x => {
                if x.chars().all(|c| c.is_ascii_lowercase()) {
                    Ok(Node::Small(x.into()))
                } else if x.chars().all(|c| c.is_ascii_uppercase()) {
                    Ok(Node::Large(x.into()))
                } else {
                    Err(aoc::ParseError::Generic)
                }
            }
        }
    }
}

type Edge = (Node, Node);
type ParsedItem = Edge;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn make_graph(edges: &[Edge]) -> aoc::UnGraphMap<&Node, i64> {
    let mut graph = aoc::UnGraphMap::new();
    for edge in edges {
        let a = graph.add_node(&edge.0);
        let b = graph.add_node(&edge.1);
        graph.add_edge(a, b, 1);
    }
    graph
}

// Lifted from petgraph's all_simple_paths code
// and modified to add the conditions..
fn all_paths<'a, TargetColl>(
    graph: &'a aoc::UnGraphMap<&'a Node, i64>,
    from: &'a Node,
    to: &'a Node,
    allow_two: bool,
) -> impl Iterator<Item = TargetColl> + 'a
where
    TargetColl: FromIterator<(&'a Node, usize)> + Clone + std::cmp::Ord + std::fmt::Debug + 'a,
{
    // set of paths
    let mut paths = BTreeSet::new();
    // list of visited small nodes
    // Tuple of node and "num double-visits"
    let mut visited = vec![(from, 0)];
    // list of childs of currently exploring path nodes,
    // last elem is list of childs of last visited node
    let mut stack = vec![graph.neighbors_directed(from, aoc::Outgoing)];

    from_fn(move || {
        while let Some(children) = stack.last_mut() {
            if let Some(child) = children.next() {
                if child == to {
                    let path = visited
                        .iter()
                        .cloned()
                        .chain(Some((to, 0)))
                        .collect::<TargetColl>();
                    if !paths.contains(&path) {
                        paths.insert(path.clone());
                        return Some(path);
                    }
                } else {
                    match child {
                        Node::Large(_) => {
                            let path = visited
                                .iter()
                                .cloned()
                                .chain(Some((child, 0)))
                                .collect::<TargetColl>();
                            if !paths.contains(&path) {
                                visited.push((child, 0));
                                stack.push(graph.neighbors_directed(child, aoc::Outgoing));
                            }
                        }
                        node => {
                            // How many visits to this child=
                            let num = visited.iter().filter(|(c, _i)| c == &child).count();
                            let ok = if allow_two && *node != Node::Start && *node != Node::End {
                                // Allow num > 0 if there was no previous double-visits
                                num == 0 || visited.iter().map(|(_, c)| c).sum::<usize>() == 0
                            } else {
                                num == 0
                            };
                            if ok {
                                let path = visited
                                    .iter()
                                    .cloned()
                                    .chain(Some((child, num)))
                                    .collect::<TargetColl>();
                                if !paths.contains(&path) {
                                    visited.push((child, num));
                                    stack.push(graph.neighbors_directed(child, aoc::Outgoing));
                                }
                            }
                        }
                    };
                }
            } else {
                stack.pop();
                let path = visited.iter().cloned().collect::<TargetColl>();
                paths.insert(path);
                visited.pop();
            }
        }
        None
    })
}

fn part1(edges: &[ParsedItem]) -> Answer {
    let g = make_graph(edges);
    let paths: Vec<Vec<(&Node, usize)>> = all_paths(&g, &Node::Start, &Node::End, false).collect();
    paths.len() as Answer
}

fn part2(edges: &[ParsedItem]) -> Answer {
    let g = make_graph(edges);
    let paths: Vec<Vec<(&Node, usize)>> = all_paths(&g, &Node::Start, &Node::End, true).collect();
    paths.len() as Answer
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| aoc::split_ch(x, '-'))
        .map(|x| (x[0].parse().unwrap(), x[1].parse().unwrap()))
        .collect()
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
    use super::*;

    fn example1() -> Vec<String> {
        vec![
            "start-A".into(),
            "start-b".into(),
            "A-c".into(),
            "A-b".into(),
            "b-d".into(),
            "A-end".into(),
            "b-end".into(),
        ]
    }

    fn example2() -> Vec<String> {
        vec![
            "dc-end".into(),
            "HN-start".into(),
            "start-kj".into(),
            "dc-start".into(),
            "dc-HN".into(),
            "LN-dc".into(),
            "HN-end".into(),
            "kj-sa".into(),
            "kj-HN".into(),
            "kj-dc".into(),
        ]
    }

    fn example3() -> Vec<String> {
        vec![
            "fs-end".into(),
            "he-DX".into(),
            "fs-he".into(),
            "start-DX".into(),
            "pj-DX".into(),
            "end-zg".into(),
            "zg-sl".into(),
            "zg-pj".into(),
            "pj-he".into(),
            "RW-he".into(),
            "fs-DX".into(),
            "pj-RW".into(),
            "zg-RW".into(),
            "start-pj".into(),
            "he-WI".into(),
            "zg-he".into(),
            "pj-fs".into(),
            "start-RW".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example1())), 10);
        assert_eq!(part1(&parse(&example2())), 19);
        assert_eq!(part1(&parse(&example3())), 226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example1())), 36);
        assert_eq!(part2(&parse(&example2())), 103);
        assert_eq!(part2(&parse(&example3())), 3509);
    }
}
