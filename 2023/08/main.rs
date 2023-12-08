use std::iter::*;

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display("{from} = ({left}, {right})")]
struct Node {
    from: String,
    left: String,
    right: String,
}

fn get_next(nodes: &[Node], s: &str, d: char) -> String {
    for n in nodes {
        if n.from == s {
            return match d {
                'L' => n.left.clone(),
                'R' => n.right.clone(),
                _ => panic!(),
            };
        }
    }
    panic!()
}

type Parsed = (Vec<char>, Vec<Node>);

fn part1(data: &Parsed) -> i64 {
    let mut pos = "AAA".to_string();
    let mut ix = 0;
    while pos != "ZZZ" {
        pos = get_next(&data.1, &pos, data.0[ix % data.0.len()]);
        ix += 1;
    }
    ix as i64
}

fn part2(data: &Parsed) -> i64 {
    let mut pos: Vec<String> = data
        .1
        .iter()
        .filter_map(|x| {
            if x.from.ends_with('A') {
                Some(x.from.clone())
            } else {
                None
            }
        })
        .collect();
    let mut ix = 0;
    let mut periods = vec![];
    while periods.len() != pos.len() {
        pos = pos
            .iter()
            .map(|p| get_next(&data.1, p, data.0[ix % data.0.len()]))
            .collect();
        periods.extend(pos.iter().filter_map(|pp| {
            if pp.ends_with('Z') {
                Some((ix + 1) as i64)
            } else {
                None
            }
        }));
        ix += 1;
    }
    aoc::lcm_arr(&periods)
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    let dirs: Vec<char> = parts[0][0].chars().collect();
    (dirs, parts[1].iter().map(|x| x.parse().unwrap()).collect())
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "LLR".into(),
            "".into(),
            "AAA = (BBB, BBB)".into(),
            "BBB = (AAA, ZZZ)".into(),
            "ZZZ = (ZZZ, ZZZ)".into(),
        ]
    }

    fn example2() -> Vec<String> {
        vec![
            "LR".into(),
            "".into(),
            "11A = (11B, XXX)".into(),
            "11B = (XXX, 11Z)".into(),
            "11Z = (11B, XXX)".into(),
            "22A = (22B, XXX)".into(),
            "22B = (22C, 22C)".into(),
            "22C = (22Z, 22Z)".into(),
            "22Z = (22B, 22B)".into(),
            "XXX = (XXX, XXX)".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example2())), 6);
    }
}
