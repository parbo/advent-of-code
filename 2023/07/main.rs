use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
    iter::*,
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Rules {
    Part1,
    Part2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: String,
    bid: i64,
    rules: Rules,
    cached_type: RefCell<Option<HandType>>,
}

impl FromStr for Hand {
    type Err = aoc::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or(aoc::ParseError::Generic)?;
        let cards = cards.to_string();
        let bid = bid.parse::<i64>()?;
        Ok(Hand {
            cards,
            bid,
            rules: Rules::Part1,
            cached_type: RefCell::new(None),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let st = self.get_type();
        let ot = other.get_type();
        let (mut so, mut oo) = (0, 0);
        for (ca, cb) in self.cards.chars().zip(other.cards.chars()) {
            if ca != cb {
                so = get_card_order(self.rules, ca);
                oo = get_card_order(other.rules, cb);
                break;
            }
        }
        (st, so).cmp(&(ot, oo))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_type(cards: &str) -> HandType {
    let mut counts: BTreeMap<char, i64> = BTreeMap::new();
    for c in cards.chars() {
        *counts.entry(c).or_default() += 1;
    }
    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    if counts[0].1 == 5 {
        HandType::FiveOfAKind
    } else if counts[0].1 == 4 {
        HandType::FourOfAKind
    } else if counts[0].1 == 3 && counts[1].1 == 2 {
        HandType::FullHouse
    } else if counts[0].1 == 3 {
        HandType::ThreeOfAKind
    } else if counts[0].1 == 2 && counts[1].1 == 2 {
        HandType::TwoPair
    } else if counts[0].1 == 2 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn get_card_order(rules: Rules, k: char) -> i64 {
    match rules {
        Rules::Part1 => match k {
            'A' => 1,
            'K' => 2,
            'Q' => 3,
            'J' => 4,
            'T' => 5,
            '9' => 6,
            '8' => 7,
            '7' => 8,
            '6' => 9,
            '5' => 10,
            '4' => 11,
            '3' => 12,
            '2' => 13,
            _ => panic!(),
        },
        Rules::Part2 => match k {
            'A' => 1,
            'K' => 2,
            'Q' => 3,
            'T' => 4,
            '9' => 5,
            '8' => 6,
            '7' => 7,
            '6' => 8,
            '5' => 9,
            '4' => 10,
            '3' => 11,
            '2' => 12,
            'J' => 13,
            _ => panic!(),
        },
    }
}

fn possible_hands(cards: &[char]) -> HashSet<Vec<char>> {
    let mut possible = HashSet::new();
    for i in 0..5 {
        if cards[i] == 'J' {
            for cc in ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'] {
                if cards.contains(&cc) || possible.is_empty() {
                    let mut hh = cards.to_vec();
                    hh[i] = cc;
                    let pp = possible_hands(&hh);
                    for p in pp {
                        possible.insert(p);
                    }
                }
            }
            return possible;
        }
    }
    possible.insert(cards.to_vec());
    possible
}

impl Hand {
    fn get_type(&self) -> HandType {
        if let Some(cached_type) = *self.cached_type.borrow() {
            return cached_type;
        }
        match self.rules {
            Rules::Part1 => {
                let t = get_type(&self.cards);
                *self.cached_type.borrow_mut() = Some(t);
                t
            }
            Rules::Part2 => {
                let cards = self.cards.chars().collect::<Vec<_>>();
                let mut h = possible_hands(&cards)
                    .into_iter()
                    .map(|x| x.iter().collect())
                    .collect::<Vec<String>>();
                h.sort_by(|a, b| {
                    get_type(a).cmp(&get_type(b)).then_with(|| {
                        let (mut so, mut oo) = (0, 0);
                        for (ca, cb) in a.chars().zip(b.chars()) {
                            if ca != cb {
                                so = get_card_order(Rules::Part2, ca);
                                oo = get_card_order(Rules::Part2, cb);
                                break;
                            }
                        }
                        so.cmp(&oo)
                    })
                });
                let t = get_type(&h[0]);
                *self.cached_type.borrow_mut() = Some(t);
                t
            }
        }
    }
}

type ParsedItem = Hand;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let mut data = data.to_vec();
    data.sort();
    data.iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) as i64 * v.bid)
        .sum()
}

fn part2(data: &Parsed) -> i64 {
    let mut data = data
        .iter()
        .map(|x| Hand {
            cards: x.cards.clone(),
            bid: x.bid,
            rules: Rules::Part2,
            cached_type: RefCell::new(None),
        })
        .collect::<Vec<_>>();
    data.sort();
    data.iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) as i64 * v.bid)
        .sum()
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
            "32T3K 765".into(),
            "T55J5 684".into(),
            "KK677 28".into(),
            "KTJJT 220".into(),
            "QQQJA 483".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 5905);
    }
}
