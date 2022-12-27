use std::collections::HashMap;
use std::iter::*;

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

fn part1(players: &Parsed) -> Answer {
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
        }
    }
    rolls * s.iter().min().unwrap()
}

fn part2(players: &Parsed) -> Answer {
    let mut games = HashMap::new();
    games.insert((players[0], players[1], 0, 0), 1i64);

    loop {
        let mut updates = vec![];
        let mut done = 0;
        for ((pa, pb, sa, sb), c) in &games {
            // Already done with this?
            if *sa >= 21 || *sb >= 21 {
                done += 1;
                updates.push(((*pa, *pb, *sa, *sb), c));
                continue;
            }
            // Roll a
            for da1 in 1..=3 {
                for da2 in 1..=3 {
                    for da3 in 1..=3 {
                        let mut new_pa = pa + da1 + da2 + da3;
                        new_pa = ((new_pa - 1) % 10) + 1;
                        let new_sa = sa + new_pa;
                        if new_sa >= 21 {
                            // If a goes over 21, we don't roll any b
                            updates.push(((new_pa, *pb, new_sa, *sb), c));
                        } else {
                            // Roll b
                            for db1 in 1..=3 {
                                for db2 in 1..=3 {
                                    for db3 in 1..=3 {
                                        let mut new_pb = pb + db1 + db2 + db3;
                                        new_pb = ((new_pb - 1) % 10) + 1;
                                        let new_sb = sb + new_pb;
                                        updates.push(((new_pa, new_pb, new_sa, new_sb), c));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut g = HashMap::new();
        for (k, c) in updates {
            *g.entry(k).or_insert(0) += c;
        }
        games = g;
        if done == games.len() {
            break;
        }
    }
    let wina: i64 = games
        .iter()
        .filter(|((_, _, sa, _), _)| *sa >= 21)
        .map(|(_, c)| c)
        .sum();
    let winb: i64 = games
        .iter()
        .filter(|((_, _, sa, sb), _)| *sa < 21 && *sb >= 21)
        .map(|(_, c)| c)
        .sum();
    wina.max(winb)
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| aoc::split_ch(x, ':')[1].parse().unwrap())
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
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
