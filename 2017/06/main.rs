use std::collections::HashSet;
use std::iter::*;
use std::time::Instant;

// #[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
// #[display("{thing}: {al}-{ah} or {bl}-{bh}")]
// struct Rule {
//     thing: String,
//     al: i64,
//     ah: i64,
//     bl: i64,
//     bh: i64,
// }

type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn solve(data: &[ParsedItem]) -> (Answer, usize) {
    let mut data = data.to_vec();
    let mut seen = HashSet::new();
    let mut cycles = 0;
    let mut looping = false;
    let mut loop_count = 0;
    loop {
        if !seen.insert(data.clone()) {
            if looping {
                break;
            }
            looping = true;
            seen.clear();
            seen.insert(data.clone());
        }
        // Rust iterator max finds the last one, so custom loop
        let mut ix = 0;
        let mut val = 0;
        for (i, v) in data.iter().enumerate() {
            if *v > val {
                ix = i;
                val = *v;
            }
        }
        // Redistribute
        data[ix] = 0;
        for _ in 0..val {
            ix = (ix + 1) % data.len();
            data[ix] += 1;
        }
        if looping {
            loop_count += 1;
        } else {
            cycles += 1;
        }
    }
    (cycles, loop_count)
}

fn part1(data: &[ParsedItem]) -> Answer {
    solve(data).0
}

fn part2(data: &[ParsedItem]) -> Answer {
    solve(data).1 as i64
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_w(&lines[0])
        .iter()
        .map(|x| x.parse().unwrap())
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
    use super::*;

    fn example() -> Vec<String> {
        vec!["0 2 7 0".into()]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 4);
    }
}
