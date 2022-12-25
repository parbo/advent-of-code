use std::collections::HashMap;
use std::iter::*;

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

fn part1(moves: &Parsed) -> Answer {
    let mut programs = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];
    dance(moves, &mut programs);
    programs.into_iter().collect()
}

fn part2(moves: &Parsed) -> Answer {
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
    aoc::run_main(parse, part1, part2);
}
