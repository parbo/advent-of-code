use std::{collections::HashSet, iter::*};

#[derive(Debug)]
struct Board {
    board: Vec<Vec<(i64, bool)>>,
}

#[derive(Debug)]
struct Bingo {
    numbers: Vec<i64>,
    boards: Vec<Board>,
}

type Parsed = Bingo;
type Answer = i64;

impl Board {
    fn has_bingo(&self) -> bool {
        for row in &self.board {
            if row.iter().all(|(_num, marked)| *marked) {
                return true;
            }
        }
        let cols = self.board[0].len();
        'outer: for col in 0..cols {
            for row in &self.board {
                if !row[col].1 {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }

    fn score(&self, num: i64) -> i64 {
        num * self
            .board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|(num, marked)| if *marked { 0 } else { *num })
                    .sum::<i64>()
            })
            .sum::<i64>()
    }

    fn mark(&mut self, num: i64) -> bool {
        for row in &mut self.board {
            for col in row {
                if col.0 == num {
                    col.1 = true;
                    return true;
                }
            }
        }
        false
    }
}

fn part1(bingo: &mut Parsed) -> Answer {
    for num in &bingo.numbers {
        for board in &mut bingo.boards {
            if board.mark(*num) && board.has_bingo() {
                return board.score(*num);
            }
        }
    }
    -1
}

fn part2(bingo: &mut Parsed) -> Answer {
    let mut won = HashSet::new();
    let mut last = None;
    for num in &bingo.numbers {
        for i in 0..bingo.boards.len() {
            if won.contains(&i) {
                continue;
            }
            if bingo.boards[i].mark(*num) && bingo.boards[i].has_bingo() {
                last = Some((i, num));
                won.insert(i);
            }
        }
    }
    let (ix, num) = last.unwrap();
    bingo.boards[ix].score(*num)
}

fn parse(lines: &[String]) -> Parsed {
    let sections = aoc::split_by_empty_line(lines);
    let numbers: Vec<i64> = aoc::split_ch(sections[0][0], ',')
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut boards = vec![];
    for section in sections.iter().skip(1) {
        boards.push(Board {
            board: section
                .iter()
                .map(|x| {
                    aoc::split_w(x)
                        .iter()
                        .map(|x| (x.parse().unwrap(), false))
                        .collect::<Vec<_>>()
                })
                .collect(),
        });
    }
    Bingo { numbers, boards }
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let mut parsed = parse(&lines);
    let result = if part == 1 {
        part1(&mut parsed)
    } else {
        part2(&mut parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut bingo = parse(&vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".into(),
            "".into(),
            "22 13 17 11  0".into(),
            " 8  2 23  4 24".into(),
            "21  9 14 16  7".into(),
            " 6 10  3 18  5".into(),
            " 1 12 20 15 19".into(),
            "".into(),
            " 3 15  0  2 22".into(),
            " 9 18 13 17  5".into(),
            "19  8  7 25 23".into(),
            "20 11 10 24  4".into(),
            "14 21 16 12  6".into(),
            "".into(),
            "14 21 17 24  4".into(),
            "10 16 15  9 19".into(),
            "18  8 23 26 20".into(),
            "22 11 13  6  5".into(),
            " 2  0 12  3  7".into(),
        ]);
        assert_eq!(part1(&mut bingo), 4512);
    }

    #[test]
    fn test_part2() {
        let mut bingo = parse(&vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".into(),
            "".into(),
            "22 13 17 11  0".into(),
            " 8  2 23  4 24".into(),
            "21  9 14 16  7".into(),
            " 6 10  3 18  5".into(),
            " 1 12 20 15 19".into(),
            "".into(),
            " 3 15  0  2 22".into(),
            " 9 18 13 17  5".into(),
            "19  8  7 25 23".into(),
            "20 11 10 24  4".into(),
            "14 21 16 12  6".into(),
            "".into(),
            "14 21 17 24  4".into(),
            "10 16 15  9 19".into(),
            "18  8 23 26 20".into(),
            "22 11 13  6  5".into(),
            " 2  0 12  3  7".into(),
        ]);
        assert_eq!(part2(&mut bingo), 1924);
    }
}
