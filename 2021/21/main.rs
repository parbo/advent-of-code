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
    let mut pp = vec![p];
    for (j, d) in perms.iter().enumerate() {
	p += d;
	p = ((p - 1) % 10) + 1;
	pp.push(p);
	s += p;
	if s >= 21 {
	    return Some(j)
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
    let mut steps : HashMap<i64, i64> = HashMap::new();
    for v in (0..3).map(|_| (1..=3)).multi_cartesian_product() {
	let sum = v.iter().sum();
	*steps.entry(sum).or_insert(0) += 1;
    }
    println!("{:?}", steps);
    // All the possible winning games
    let mut possible_games_0 = HashMap::new();
    run_games(players[0], &[], &mut possible_games_0);
    let mut possible_games_1 = HashMap::new();
    run_games(players[1], &[], &mut possible_games_1);
    let mut games_0 = HashMap::new();
    for (perms, j) in possible_games_0 {
	// The number of possible games with this step sequence
	let c = perms.iter().map(|d| steps.get(d).unwrap()).product();
	games_0.insert(perms.to_owned(), (j, c));
    }
    let mut games_1 = HashMap::new();
    for (perms, j) in possible_games_1 {
	// The number of possible games with this step sequence
	let c = perms.iter().map(|d| steps.get(d).unwrap()).product();
	games_1.insert(perms.to_owned(), (j, c));
    }
    println!("games: {}", games_0.len());
    println!("games: {}", games_1.len());
    // All winning games
    let mut winning_games = HashMap::new();
    for (p, (j, c)) in games_0 {
	let (jj, cc) = winning_games.entry(p).or_insert((j, c));
	if j < *jj {
	    *jj = j;
	    *cc = c;
	}
    }
    for (p, (j, c)) in games_1 {
	let (jj, cc) = winning_games.entry(p).or_insert((j, c));
	if j < *jj {
	    *jj = j;
	    *cc = c;
	}
    }
    // How many games end in x steps?
    println!("games: {}", winning_games.len());
    let mut game_steps : HashMap<usize, i64> = HashMap::new();
    for (_draws, (steps, c)) in &winning_games {
	*game_steps.entry(*steps).or_insert(0) += c;
    }
    println!("game_steps: {:?}", game_steps);
    // For all the combinations of games
    let mut wins: Vec<i64> = vec![0; players.len()];
    let start = *game_steps.keys().min().unwrap();
    let end = *game_steps.keys().max().unwrap();
    for v in (0..2).map(|_| (start..=end)).multi_cartesian_product() {
	println!("v: {:?}", v);
	if v[0] <= v[1] {
	    wins[0] += *game_steps.get(&v[0]).unwrap();
	} else {
	    wins[1] += *game_steps.get(&v[0]).unwrap();
	}
	println!("wins: {:?}", wins);
    }
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
