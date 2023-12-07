use std::{cmp::Ordering, iter::*};

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
    hand_type: HandType,
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
        let st = self.hand_type;
        let ot = other.hand_type;
        st.cmp(&ot).then_with(|| {
            let (mut so, mut oo) = (0, 0);
            for (ca, cb) in self.cards.chars().zip(other.cards.chars()) {
                if ca != cb {
                    so = get_card_order(self.rules, ca);
                    oo = get_card_order(other.rules, cb);
                    break;
                }
            }
            so.cmp(&oo)
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_type(cards: &str, rules: Rules) -> HandType {
    match rules {
        Rules::Part1 => {
            let mut counts: aoc::FxHashMap<char, i64> = aoc::FxHashMap::default();
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
        Rules::Part2 => {
            let cards = cards.chars().collect::<Vec<_>>();
            let mut h = possible_hands(&cards)
                .into_iter()
                .map(|x| x.iter().collect::<String>())
                .map(|x| (get_type(&x, Rules::Part1), x))
                .collect::<Vec<(HandType, String)>>();
            h.sort_by(|a, b| {
                a.0.cmp(&b.0).then_with(|| {
                    let (mut so, mut oo) = (0, 0);
                    for (ca, cb) in a.1.chars().zip(b.1.chars()) {
                        if ca != cb {
                            so = get_card_order(Rules::Part2, ca);
                            oo = get_card_order(Rules::Part2, cb);
                            break;
                        }
                    }
                    so.cmp(&oo)
                })
            });
            h[0].0
        }
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

fn possible_hands(cards: &[char]) -> aoc::FxHashSet<Vec<char>> {
    let mut possible = aoc::FxHashSet::default();
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

type ParsedItem = String;
type Parsed = Vec<ParsedItem>;

fn parse_rules(lines: &[String], rules: Rules) -> Vec<Hand> {
    lines
        .iter()
        .map(|s| {
            let (cards, bid) = s.split_once(' ').unwrap();
            let cards = cards.to_string();
            let bid = bid.parse::<i64>().unwrap();
            let hand_type = get_type(&cards, rules);
            Hand {
                cards,
                bid,
                rules,
                hand_type,
            }
        })
        .collect()
}

fn part1(data: &Parsed) -> i64 {
    let mut data = parse_rules(data, Rules::Part1);
    data.sort();
    data.iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) as i64 * v.bid)
        .sum()
}

fn part2(data: &Parsed) -> i64 {
    let mut data = parse_rules(data, Rules::Part2);
    data.sort();
    data.iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) as i64 * v.bid)
        .sum()
}

fn parse(lines: &[String]) -> Parsed {
    lines.to_vec()
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
