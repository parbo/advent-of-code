use std::{collections::BTreeSet, iter::*, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Card {
    num: usize,
    winning: BTreeSet<i64>,
    numbers: BTreeSet<i64>,
}

impl FromStr for Card {
    type Err = aoc::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (gg, rest) = s.split_once(':').unwrap();
        let (w, n) = rest.split_once('|').unwrap();
        let winning = aoc::split_w(w).iter().map(|x| x.parse().unwrap()).collect();
        let numbers = aoc::split_w(n).iter().map(|x| x.parse().unwrap()).collect();
        let num = aoc::split_w(gg)[1].parse()?;

        Ok(Card {
            num,
            winning,
            numbers,
        })
    }
}

type ParsedItem = Card;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let mut s = 0;
    for card in data {
        let matching = card.winning.intersection(&card.numbers);
        let c = matching.count();
        if c > 0 {
            s += 1 << (c - 1);
        }
    }
    s
}

fn part2(data: &Parsed) -> i64 {
    let mut cards: Vec<usize> = (0..data.len()).collect();
    cards.reserve(10000000);
    let mut pos = 0;
    while pos < cards.len() {
        let card = &data[cards[pos]];
        let matching = card.winning.intersection(&card.numbers);
        let c = matching.count();
        if c > 0 {
            let pos = card.num - 1;
            cards.extend(pos + 1..=pos + c);
        }
        pos += 1;
    }
    cards.len() as i64
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
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".into(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".into(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".into(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".into(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".into(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 30);
    }
}
