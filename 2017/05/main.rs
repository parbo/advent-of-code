use std::iter::*;


type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    let mut data = data.to_vec();
    let mut ix = 0;
    let mut jumps = 0;
    while ix >= 0 && ix < data.len() as i64 {
        let jump = &mut data[ix as usize];
        ix += *jump;
        *jump += 1;
        jumps += 1;
    }
    jumps
}

fn part2(data: &Parsed) -> Answer {
    let mut data = data.to_vec();
    let mut ix = 0;
    let mut jumps = 0;
    while ix >= 0 && ix < data.len() as i64 {
        let jump = &mut data[ix as usize];
        ix += *jump;
        if *jump >= 3 {
            *jump -= 1;
        } else {
            *jump += 1;
        }
        jumps += 1;
    }
    jumps
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
