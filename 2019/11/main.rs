use aoc;
use intcode;
use std::iter::*;
use std::collections::HashMap;

enum Dir {
    Up,
    Right,
    Down,
    Left
}

fn paint(numbers: &Vec<i128>, color: i128) -> HashMap<(i128, i128), i128> {
    let mut m = intcode::Machine::new(&numbers, &vec![]);
    let mut current_color;
    let mut init_color = color;
    let mut current_dir = Dir::Up;
    let mut hull = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    loop {
        current_color = match hull.get(&(x, y)) {
            Some(c) => *c,
            None => init_color,
        };
        init_color = 0;
        m.add_inputs(&vec![current_color]);
        let color = match m.run_to_next_output() {
            Some(c) => c,
            None => {
                println!("could not get color");
                break hull;
            }
        };
        let turn = match m.run_to_next_output() {
            Some(t) => t,
            None => {
                println!("could not get turn");
                break hull;
            }
        };
        hull.insert((x, y), color);
        current_dir = match current_dir {
            Dir::Up => if turn == 0 { Dir::Left } else { Dir::Right },
            Dir::Right => if turn == 0 { Dir::Up } else { Dir::Down },
            Dir::Down => if turn == 0 { Dir::Right } else { Dir::Left },
            Dir::Left => if turn == 0 { Dir::Down } else { Dir::Up },
        };
        let pos = match current_dir {
            Dir::Up => (x, y - 1),
            Dir::Right => (x + 1, y),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
        };
        x = pos.0;
        y = pos.1;
    }
}

fn part1(numbers: &Vec<i128>) -> i128 {
    let hull = paint(numbers, 0);
    hull.iter().count() as i128
}

fn part2(numbers: &Vec<i128>) -> i128 {
    let hull = paint(numbers, 1);
    let min_x = hull.iter().min_by_key(|p| (p.0).0).map(|p| (p.0).0).unwrap();
    let min_y = hull.iter().min_by_key(|p| (p.0).1).map(|p| (p.0).1).unwrap();
    let max_x = hull.iter().max_by_key(|p| (p.0).0).map(|p| (p.0).0).unwrap();
    let max_y = hull.iter().max_by_key(|p| (p.0).1).map(|p| (p.0).1).unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = match hull.get(&(x, y)) {
                Some(c) => *c,
                None => 0,
            };
            if c == 0 {
                print!(" ");
            } else {
                print!("█");
            }
        }
        println!();
    }
    0
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = aoc::parse_intcode(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(&vec![0]), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![0]), 0);
    }
}
