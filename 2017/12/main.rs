use std::iter::*;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Connection {
    from: String,
    to: Vec<String>,
}

type ParsedItem = Connection;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn get_groups(connections: &[ParsedItem]) -> Vec<Vec<&String>> {
    let mut graph = aoc::UnGraphMap::new();
    for conn in connections {
        graph.add_node(&conn.from);
        for c in &conn.to {
            graph.add_node(c);
            graph.add_edge(&conn.from, c, 1);
        }
    }
    aoc::algo::kosaraju_scc(&graph)
}

fn part1(connections: &[ParsedItem]) -> Answer {
    let groups = get_groups(connections);
    for c in groups {
        if c.contains(&&"0".to_string()) {
            return c.len() as i64;
        }
    }
    panic!()
}

fn part2(connections: &[ParsedItem]) -> Answer {
    let groups = get_groups(connections);
    groups.len() as i64
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let s = aoc::split_w(x);
            let from = s[0].to_string();
            let to = s[2..]
                .iter()
                .map(|x| x.replace(',', ""))
                .collect::<Vec<String>>();
            Connection { from, to }
        })
        .collect()
}

fn main() {
    let start_time = Instant::now();
    let (part, lines) = aoc::read_lines();
    let io_time = Instant::now();
    let parsed = parse(&lines);
    let parse_time = Instant::now();
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    let done_time = Instant::now();
    println!(
        "read: {:?}, parse: {:?}, solve: {:?}\n",
        io_time.duration_since(start_time),
        parse_time.duration_since(io_time),
        done_time.duration_since(parse_time)
    );
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn example() -> Vec<String> {
    // 	   vec![
    //         "0".into()
    //     ]
    // }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&parse(&example())), 0);
    // }
}
