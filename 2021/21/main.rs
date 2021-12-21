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
    // Possible outcomes of three rolls
    let mut steps: HashMap<i64, i64> = HashMap::new();
    for v in (0..3).map(|_| (1..=3)).multi_cartesian_product() {
        let sum = v.iter().sum();
        *steps.entry(sum).or_insert(0) += 1;
    }

    let mut games = HashMap::new();
    games.insert((players[0], players[1], 0, 0), 1);

    loop {
	let mut g = HashMap::new();

	let mut done = 0;
	for ((pa, pb, sa, sb), c) in &games {
	    if *sa >= 21 || *sb >= 21 {
		done += 1;
		g.insert((*pa, *pb, *sa, *sb), *c);
		continue;
	    }
	    for da in 3..=9 {
		let na = steps.get(&da).unwrap();
		let mut new_pa = pa + da;
		new_pa = ((new_pa - 1) % 10) + 1;
		let new_sa = sa + new_pa;
		if new_sa >= 21 {
		    let new_c = c * na;
		    *g.entry((new_pa, *pb, new_sa, *sb)).or_insert(0) += new_c;
		} else {
		    for db in 3..=9 {
			let nb = steps.get(&db).unwrap();
			let mut new_pb = pb + db;
			new_pb = ((new_pb - 1) % 10) + 1;
			let new_sb = sb + new_pb;
			let new_c = c * na * nb;
			*g.entry((new_pa, new_pb, new_sa, new_sb)).or_insert(0) += new_c;
		    }
		}
	    }
	}
	println!("g {:?}, {}", g.len(), done);
	if done == g.len() {
	    break;
	}
	games = g;
    }
    let wina : i64 = games.iter().filter(|((_, _, sa, _), _)| *sa >= 21).map(|(_, c)| c).sum();
    let winb : i64 = games.iter().filter(|((_, _, sa, sb), _)| *sa < 21 && *sb >= 21).map(|(_, c)| c).sum();
    println!("wina: {}, winb: {}", wina, winb);
    wina.max(winb)
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
