use std::iter::*;

type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;

fn secret_round(mut secret: i64) -> i64 {
    secret ^= secret * 64;
    secret = secret.rem_euclid(16777216);
    secret ^= secret / 32;
    secret = secret.rem_euclid(16777216);
    secret ^= secret * 2048;
    secret = secret.rem_euclid(16777216);
    secret
}

fn secret_number(mut secret: i64, num: usize) -> i64 {
    for _ in 0..num {
        secret = secret_round(secret);
    }
    secret
}

fn part1(data: &Parsed) -> i64 {
    data.iter().map(|num| secret_number(*num, 2000)).sum()
}

fn part2(data: &Parsed) -> i64 {
    let mut changes = vec![];
    for num in data {
        let mut deltas = vec![];
        let mut last = 0;
        let mut secret = *num;
        for _ in 0..2000 {
            secret = secret_round(secret);
            let price = secret.rem_euclid(10);
            deltas.push((price, price - last));
            last = price;
        }
        changes.push(deltas);
    }
    let mut allseqs = aoc::FxHashSet::default();
    let mut buyer_seqs: Vec<aoc::FxHashMap<Vec<i64>, i64>> = vec![];
    for c in &changes {
        let mut seqs = aoc::FxHashMap::default();
        for seq in c.windows(4) {
            let mut s = vec![];
            for (_p, c) in seq {
                s.push(*c);
            }
            let sv = s.to_vec();
            if !seqs.contains_key(&sv) {
                seqs.insert(sv.clone(), seq[3].0);
                allseqs.insert(sv);
            }
        }
        buyer_seqs.push(seqs);
    }
    let mut best = 0;
    for seq in &allseqs {
        let mut tot = 0;
        for seqs in &buyer_seqs {
            if let Some(p) = seqs.get(seq) {
                tot += p;
                if tot > best {
                    best = tot;
                }
                continue;
            }
        }
    }
    best
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
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_secret() {
        let mut secret = 123;
        let mut secrets = vec![];
        for _ in 0..10 {
            secret = secret_round(secret);
            secrets.push(secret);
        }
        assert_eq!(
            secrets,
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 37327623);
    }
}
