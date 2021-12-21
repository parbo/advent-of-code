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

fn run_game(mut p: i64, perms: &[i64]) -> Option<usize> {
    // Compute in which step each player wins for this step sequence
    let mut s = 0;
    for (j, d) in perms.iter().enumerate() {
        p += d;
        p = ((p - 1) % 10) + 1;
        s += p;
        if s >= 21 {
            return Some(j);
        }
    }
    None
}

fn run_games(p: i64, draws: &[i64], wins: &mut HashMap<Vec<i64>, usize>) {
    if let Some(j) = run_game(p, draws) {
        wins.insert(draws.to_owned(), j);
    } else {
        // recurse
        for i in 3..=9 {
            let mut d = draws.to_owned();
            d.push(i);
            run_games(p, &d, wins);
        }
    }
}

fn part2(players: &[ParsedItem]) -> Answer {
    // Possible outcomes of three rolls
    let mut steps: HashMap<i64, i64> = HashMap::new();
    for v in (0..3).map(|_| (1..=3)).multi_cartesian_product() {
        let sum = v.iter().sum();
        *steps.entry(sum).or_insert(0) += 1;
    }
    println!("{:?}", steps);
    // All the possible winning games
    let mut possible_games_0 = HashMap::new();
    possible_games_0.reserve(90000);
    run_games(players[0], &[], &mut possible_games_0);
    let mut possible_games_1 = HashMap::new();
    possible_games_1.reserve(90000);
    run_games(players[1], &[], &mut possible_games_1);
    println!("games: {}", possible_games_0.len());
    println!("games: {}", possible_games_1.len());
    let mut games_0: HashMap<Vec<i64>, (usize, i64)> = HashMap::new();
    games_0.reserve(possible_games_0.len());
    for (draws, s) in &possible_games_0 {
        games_0.insert(
            draws.to_owned(),
            (*s, draws
                .iter()
                .map(|d| steps.get(d).unwrap())
                .product::<i64>()));
    }
    let mut games_1: HashMap<Vec<i64>, (usize, i64)> = HashMap::new();
    games_1.reserve(possible_games_1.len());
    for (draws, s) in &possible_games_1 {
        games_1.insert(
            draws.to_owned(),
            (*s, draws
                .iter()
                .map(|d| steps.get(d).unwrap())
                .product::<i64>()));
    }
    // For all the combinations of games
    let mut wins: Vec<i64> = vec![0; players.len()];
    for (sa, ca) in games_0.values() {
        for (sb, cb) in games_1.values() {
            if sa <= sb {
                wins[0] += ca * cb;
            } else {
                wins[1] += ca * cb;
            }
        }
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
