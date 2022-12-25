use std::iter::*;
use std::str::FromStr;
use std::time::Instant;

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
fn num_paths(
    graph: &aoc::UnGraphMap<&Node, i64>,
    from: &Node,
    to: &Node,
    allow_two: bool,
) -> usize {
    // list of visited small nodes
    // Tuple of node and "num double-visits"
    let mut visited = vec![(from, false)];
    // list of childs of currently exploring path nodes,
    // last elem is list of childs of last visited node
    let mut stack = vec![graph.neighbors_directed(from, aoc::Outgoing)];

    let iter = from_fn(move || {
        while let Some(children) = stack.last_mut() {
            if let Some(child) = children.next() {
                if child == to {
                    return Some(true);
                } else {
                    let (add, visited_before) = match child {
                        Node::Large(_) => (true, false),
                        Node::Start | Node::End => (!visited.contains(&(child, false)), false),
                        Node::Small(_) => {
                            let visited_before = visited.iter().any(|(c, _)| c == &child);
                            // Allow visited_before if there was no previous double-visits
                            let add =
                                !visited_before || (allow_two && !visited.iter().any(|(_, c)| *c));
                            (add, visited_before)
                        }
                    };
                    if add {
                        visited.push((child, visited_before));
                        stack.push(graph.neighbors_directed(child, aoc::Outgoing));
                    }
                }
            } else {
                stack.pop();
                visited.pop();
            }
        }
        None
    });
    iter.count()
}

fn part1(edges: &Parsed) -> Answer {
    let g = make_graph(edges);
    num_paths(&g, &Node::Start, &Node::End, false) as Answer
}

fn part2(edges: &Parsed) -> Answer {
    let g = make_graph(edges);
    num_paths(&g, &Node::Start, &Node::End, true) as Answer
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| aoc::split_ch(x, '-'))
        .map(|x| (x[0].parse().unwrap(), x[1].parse().unwrap()))
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
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
