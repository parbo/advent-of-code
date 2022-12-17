use std::iter::*;

type ParsedItem = char;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    let rock1 = vec![[0, 0], [1, 0], [2, 0], [3, 0]];
    let rock2 = vec![[1, 0], [0, 1], [1, 1], [2, 1], [1, 2]];
    let rock3 = vec![[2, 0], [2, 1], [0, 2], [1, 2], [2, 2]];
    let rock4 = vec![[0, 0], [0, 1], [0, 2], [0, 3]];
    let rock5 = vec![[0, 0], [1, 0], [0, 1], [1, 1]];
    let rocks: Vec<Vec<Point>> = vec![rock1, rock2, rock3, rock4, rock5];
    println!("{:?}", rocks);
    0
}

fn part2(_: &Parsed) -> Answer {
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines[0].chars().collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
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
