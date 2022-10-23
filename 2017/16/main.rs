use std::collections::HashMap;
use std::iter::*;
use std::time::Instant;

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
enum Move {
    #[display("s{0}")]
    Spin(usize),
    #[display("x{0}/{1}")]
    Exchange(usize, usize),
    #[display("p{0}/{1}")]
    Partner(char, char),
}

type ParsedItem = Move;
type Parsed = Vec<ParsedItem>;
type Answer = String;

fn dance(moves: &[ParsedItem], programs: &mut [char; 16]) {
    for m in moves {
        match m {
            Move::Spin(x) => {
                programs.rotate_right(*x);
            }
            Move::Exchange(a, b) => {
                programs.swap(*a, *b);
            }
            Move::Partner(a, b) => {
                let ai = programs.iter().position(|c| c == a).unwrap();
                let bi = programs.iter().position(|c| c == b).unwrap();
                programs.swap(ai, bi);
            }
        }
    }
}

fn part1(moves: &[ParsedItem]) -> Answer {
    let mut programs = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];
    dance(moves, &mut programs);
    programs.into_iter().collect()
}

fn part2(moves: &[ParsedItem]) -> Answer {
    let mut programs = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];
    let mut seen = HashMap::new();
    seen.insert(programs, 0);
    let mut i = 0;
    let mut end = 1_000_000_000;
    while i < end {
        dance(moves, &mut programs);
        i += 1;
        if end == 1_000_000_000 {
            if let Some(ix) = seen.insert(programs, i) {
                // Found loop, fix end condition
                let loop_len = i - ix;
                end = i + ((end - i) % loop_len);
            }
        }
    }
    programs.into_iter().collect()
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_ch(&lines[0], ',')
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
