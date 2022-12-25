use std::iter::*;

use aoc::{Grid, GridDrawer};

#[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
#[display(style = "snake_case")]
enum Instruction {
    Noop,
    #[display("addx {0}")]
    Addx(i64),
}

type ParsedItem = Instruction;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    let mut cycle = 1;
    let mut pc = 0;
    let mut ins_cycle = 0;
    let mut x = 1;
    let mut signal_strength = 0;
    let mut crt = vec![vec![' '; 40]; 6];
    while pc < data.len() {
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            signal_strength += cycle * x;
        }
        let yp = (cycle - 1) / 40;
        let xp = (cycle - 1) % 40;
        if x - 1 <= xp && xp <= x + 1 {
            crt.set_value([xp, yp], '#');
        }
        ins_cycle += 1;
        match data[pc] {
            Instruction::Noop => {
                pc += 1;
                ins_cycle = 0;
            }
            Instruction::Addx(v) => {
                if ins_cycle == 2 {
                    x += v;
                    pc += 1;
                    ins_cycle = 0;
                }
            }
        }
        cycle += 1;
    }
    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    gd.draw(&crt);
    signal_strength
}

fn part2(_: &Parsed) -> Answer {
    0
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "addx 15".into(),
            "addx -11".into(),
            "addx 6".into(),
            "addx -3".into(),
            "addx 5".into(),
            "addx -1".into(),
            "addx -8".into(),
            "addx 13".into(),
            "addx 4".into(),
            "noop".into(),
            "addx -1".into(),
            "addx 5".into(),
            "addx -1".into(),
            "addx 5".into(),
            "addx -1".into(),
            "addx 5".into(),
            "addx -1".into(),
            "addx 5".into(),
            "addx -1".into(),
            "addx -35".into(),
            "addx 1".into(),
            "addx 24".into(),
            "addx -19".into(),
            "addx 1".into(),
            "addx 16".into(),
            "addx -11".into(),
            "noop".into(),
            "noop".into(),
            "addx 21".into(),
            "addx -15".into(),
            "noop".into(),
            "noop".into(),
            "addx -3".into(),
            "addx 9".into(),
            "addx 1".into(),
            "addx -3".into(),
            "addx 8".into(),
            "addx 1".into(),
            "addx 5".into(),
            "noop".into(),
            "noop".into(),
            "noop".into(),
            "noop".into(),
            "noop".into(),
            "addx -36".into(),
            "noop".into(),
            "addx 1".into(),
            "addx 7".into(),
            "noop".into(),
            "noop".into(),
            "noop".into(),
            "addx 2".into(),
            "addx 6".into(),
            "noop".into(),
            "noop".into(),
            "noop".into(),
            "noop".into(),
            "noop".into(),
            "addx 1".into(),
            "noop".into(),
            "noop".into(),
            "addx 7".into(),
            "addx 1".into(),
            "noop".into(),
            "addx -13".into(),
            "addx 13".into(),
            "addx 7".into(),
            "noop".into(),
            "addx 1".into(),
            "addx -33".into(),
            "noop".into(),
            "noop".into(),
            "noop".into(),
            "addx 2".into(),
            "noop".into(),
            "noop".into(),
            "noop".into(),
            "addx 8".into(),
            "noop".into(),
            "addx -1".into(),
            "addx 2".into(),
            "addx 1".into(),
            "noop".into(),
            "addx 17".into(),
            "addx -9".into(),
            "addx 1".into(),
            "addx 1".into(),
            "addx -3".into(),
            "addx 11".into(),
            "noop".into(),
            "noop".into(),
            "addx 1".into(),
            "noop".into(),
            "addx 1".into(),
            "noop".into(),
            "noop".into(),
            "addx -13".into(),
            "addx -19".into(),
            "addx 1".into(),
            "addx 3".into(),
            "addx 26".into(),
            "addx -30".into(),
            "addx 12".into(),
            "addx -1".into(),
            "addx 3".into(),
            "addx 1".into(),
            "noop".into(),
            "noop".into(),
            "noop".into(),
            "addx -9".into(),
            "addx 18".into(),
            "addx 1".into(),
            "addx 2".into(),
            "noop".into(),
            "noop".into(),
            "addx 9".into(),
            "noop".into(),
            "noop".into(),
            "noop".into(),
            "addx -1".into(),
            "addx 2".into(),
            "addx -37".into(),
            "addx 1".into(),
            "addx 3".into(),
            "noop".into(),
            "addx 15".into(),
            "addx -21".into(),
            "addx 22".into(),
            "addx -6".into(),
            "addx 1".into(),
            "noop".into(),
            "addx 2".into(),
            "addx 1".into(),
            "noop".into(),
            "addx -10".into(),
            "noop".into(),
            "noop".into(),
            "addx 20".into(),
            "addx 1".into(),
            "addx 2".into(),
            "addx 2".into(),
            "addx -6".into(),
            "addx -11".into(),
            "noop".into(),
            "noop".into(),
            "noop".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 13140);
    }
}
