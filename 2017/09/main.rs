use std::iter::*;
use std::ops::Range;
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

type Answer = i64;

fn parse_garbage(stream: &[char], rg: Range<usize>) -> Option<(usize, usize)> {
    let mut ignore_next = false;
    let mut count = 0;
    for ix in rg {
        if ignore_next {
            ignore_next = false;
            continue;
        }
        match stream[ix] {
            '>' => {
                return Some((ix + 1, count));
            }
            '!' => {
                ignore_next = true;
            }
            _ => {
                count += 1;
            }
        }
    }
    None
}

fn parse_group(stream: &[char], rg: Range<usize>, level: usize) -> (usize, usize, usize) {
    let mut ix = rg.start;
    let mut score = level;
    let mut garbage = 0;
    while ix < rg.end {
        match stream[ix] {
            '{' => {
                let (end, s, g) = parse_group(stream, (ix + 1)..rg.end, level + 1);
                ix = end;
                score += s;
                garbage += g;
            }
            '}' => {
                return (ix + 1, score, garbage);
            }
            '<' => {
                let (nix, g) = parse_garbage(stream, (ix + 1)..rg.end).unwrap();
                ix = nix;
                garbage += g;
            }
            ',' => {
                ix += 1;
            }
            _ => panic!(),
        }
    }
    (rg.end, score, garbage)
}

fn part1(stream: &[char]) -> Answer {
    let (_end, score, _garbage) = parse_group(stream, 0..stream.len(), 0);
    score as i64
}

fn part2(stream: &[char]) -> Answer {
    let (_end, _score, garbage) = parse_group(stream, 0..stream.len(), 0);
    garbage as i64
}

fn parse(lines: &[String]) -> Vec<char> {
    lines[0].chars().collect()
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

    #[test]
    fn test_garbage() {
        let g: Vec<String> = vec![
            "<>".into(),
            "<random characters>".into(),
            "<<<<>".into(),
            "<{!>}>".into(),
            "<!!>".into(),
            "<!!!>>".into(),
            "<{o\"i!a,<{i<a>".into(),
        ];
        for s in g {
            let l = s.len();
            println!("{}", s);
            let res = parse_garbage(&parse(&[s]), 1..l);
            assert!(res.is_some());
            assert_eq!(res.unwrap().0, l);
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&["{}".into()])), 1);
        assert_eq!(part1(&parse(&["{{{}}}".into()])), 6);
        assert_eq!(part1(&parse(&["{{},{}}".into()])), 5);
        assert_eq!(part1(&parse(&["{{{},{},{{}}}}".into()])), 16);
        assert_eq!(part1(&parse(&["{<a>,<a>,<a>,<a>}".into()])), 1);
        assert_eq!(part1(&parse(&["{{<ab>},{<ab>},{<ab>},{<ab>}}".into()])), 9);
        assert_eq!(part1(&parse(&["{{<!!>},{<!!>},{<!!>},{<!!>}}".into()])), 9);
        assert_eq!(part1(&parse(&["{{<a!>},{<a!>},{<a!>},{<ab>}}".into()])), 3);
    }
}
