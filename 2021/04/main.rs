use pancurses::*;
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
    fn has_bingo(&self) -> i64 {
        let mut i = 0;
        for row in &self.board {
            if row.iter().all(|(_num, marked)| *marked) {
                return i + 1;
            }
            i += 1;
        }
        let cols = self.board[0].len();
        'outer: for col in 0..cols {
            for row in &self.board {
                if !row[col].1 {
                    continue 'outer;
                }
            }
            return -1 * ((col + 1) as i64);
        }
        0
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

    fn draw(&mut self, window: &Window, offset: (i32, i32)) {
        let mut y = 0;
        let bingo = self.has_bingo();
        for row in &mut self.board {
            let mut x = 0;
            for col in row {
                let bb = (y + 1) as i64 == bingo || (x + 1) as i64 == -bingo;
                if bingo != 0 {
                    if bb {
                        window.color_set(if col.1 { 6 } else { 5 });
                    } else {
                        window.color_set(if col.1 { 4 } else { 3 });
                    }
                } else {
                    window.color_set(if col.1 { 2 } else { 1 });
                }
                window.mvaddstr(
                    offset.1 * 6 + y as i32,
                    3 * (offset.0 * 6 + x) as i32,
                    format!("{:2} ", col.0),
                );
                x += 1
            }
            y += 1;
        }
    }
}

fn part1(bingo: &mut Parsed) -> Answer {
    for num in &bingo.numbers {
        for board in &mut bingo.boards {
            if board.mark(*num) && board.has_bingo() != 0 {
                return board.score(*num);
            }
        }
    }
    -1
}

fn part2(bingo: &mut Parsed) -> Answer {
    let window = initscr();
    nl();
    noecho();
    start_color();
    use_default_colors();
    init_pair(1, COLOR_WHITE, COLOR_BLACK);
    init_pair(2, COLOR_BLACK, COLOR_WHITE);
    init_pair(3, COLOR_GREEN, COLOR_BLACK);
    init_pair(4, COLOR_BLACK, COLOR_GREEN);
    init_pair(5, COLOR_RED, COLOR_BLACK);
    init_pair(6, COLOR_BLACK, COLOR_RED);
    curs_set(0);
    window.keypad(true);
    window.scrollok(true);
    window.nodelay(true);
    window.timeout(200);
    let mut won = HashSet::new();
    let mut last = None;
    for num in &bingo.numbers {
        for i in 0..bingo.boards.len() {
            bingo.boards[i].draw(&window, (i as i32 % 10i32, i as i32 / 10i32));
            if won.contains(&i) {
                continue;
            }
            if bingo.boards[i].mark(*num) && bingo.boards[i].has_bingo() != 0 {
                last = Some((i, num));
                won.insert(i);
            }
        }
        window.refresh();
        let _ = window.getch();
    }
    // Draw one last time
    for i in 0..bingo.boards.len() {
        bingo.boards[i].draw(&window, (i as i32 % 10i32, i as i32 / 10i32));
    }
    window.refresh();
    let _ = window.getch();
    endwin();
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
    println!("boards: {}", boards.len());
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
