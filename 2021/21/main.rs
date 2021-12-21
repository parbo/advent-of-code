use aoc::Itertools;
use std::collections::HashMap;
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

fn roll(d: i64) -> i64 {
    if d + 1 > 100 {
        1
    } else {
        d + 1
    }
}

fn part1(players: &[ParsedItem]) -> Answer {
    let mut p = players.to_owned();
    let mut s = vec![0; p.len()];
    let mut die = 0;
    let mut rolls = 0;
    'outer: loop {
        for i in 0..2 {
            for _ in 0..3 {
                die = roll(die);
                rolls += 1;
                p[i] += die;
                p[i] = ((p[i] - 1) % 10) + 1;
            }
            s[i] += p[i];
            if s[i] >= 1000 {
                break 'outer;
            }
            println!("{}, {}, {}, {}", die, p[i], s[i], rolls);
        }
    }
    rolls * s.iter().min().unwrap()
}

fn part2(players: &[ParsedItem]) -> Answer {
    let mut steps = HashMap::new();
    for a in 1..=3 {
	for b in 1..=3 {
	    for c in 1..=3 {
		let sum = a + b + c;
		*steps.entry(sum).or_insert(0) += 1;
	    }
	}
    }
    println!("{:?}", steps);
    let mut wins: Vec<i64> = vec![0; players.len()];
    let mut games = vec![];
    for combs in (3..=9).combinations_with_replacement(7) {
        for perms in combs.iter().copied().permutations(7) {
	    games.push(perms);
	}
    }
    for draws in games.iter().combinations_with_replacement(2) {
        let mut p = players.to_owned();
        let mut s = vec![0; p.len()];
        let mut any_win = false;
	let mut w = vec![0; p.len()];
	let mut ws = vec![1; p.len()];
        for i in 0..2 {
            'outer: for d in draws[i] {
		ws[i] *= steps.get(d).unwrap();
                p[i] += d;
                p[i] = ((p[i] - 1) % 10) + 1;
                s[i] += p[i];
                if s[i] >= 21 {
                    w[i] += 1;
                    any_win = true;
                    break 'outer;
                }
            }
	}
	for i in 0..wins.len() {
	    wins[i] += w[i] * ws[i];
	}
	// println!("wins: {:?}, ws: {:?}", wins, ws);
	// if !any_win {
	//     println!("{:?}, {:?}, {:?}", perms, p, s);
	// }
        assert!(any_win);
    }
    println!("wins: {:?}", wins);
    *wins.iter().max().unwrap()
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| aoc::split_ch(x, ':')[1].parse().unwrap())
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
        vec![
            "Player 1 starting position: 4".into(),
            "Player 2 starting position: 8".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 739785);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 444356092776315);
    }
}
