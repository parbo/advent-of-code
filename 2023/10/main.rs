use aoc::{Grid, EAST, NORTH, SOUTH, WEST};
use std::{
    collections::{HashMap, HashSet},
    iter::*,
};

type Parsed = Vec<Vec<char>>;

fn get_loop(data: &Parsed) -> Vec<aoc::Point> {
    let s = data
        .points()
        .find(|p| data.get_value(*p).unwrap() == 'S')
        .unwrap();
    let poss = HashMap::from([
        ('-', vec![EAST, WEST]),
        ('|', vec![NORTH, SOUTH]),
        ('J', vec![NORTH, WEST]),
        ('L', vec![NORTH, EAST]),
        ('7', vec![SOUTH, WEST]),
        ('F', vec![SOUTH, EAST]),
        ('S', vec![SOUTH, EAST, WEST, NORTH]),
    ]);
    let poss2 = HashMap::from([
        (EAST, vec!['-', 'J', '7', 'S']),
        (WEST, vec!['-', 'L', 'F', 'S']),
        (NORTH, vec!['|', 'F', '7', 'S']),
        (SOUTH, vec!['|', 'J', 'L', 'S']),
    ]);
    let mut frontier = vec![vec![s]];
    let mut paths = vec![];
    while let Some(path) = frontier.pop() {
        let pos = *path.last().unwrap();
        let c = data.get_value(pos).unwrap();
        if let Some(nbs) = poss.get(&c) {
            for nb in nbs {
                let new_pos = aoc::point_add(pos, *nb);
                if let Some(new_c) = data.get_value(new_pos) {
                    if path.len() > 2 && new_c == 'S' {
                        paths.push(path.clone());
                        continue;
                    }
                    if !path.contains(&new_pos) && poss2.get(nb).unwrap().contains(&new_c) {
                        let mut new_path = path.clone();
                        new_path.push(new_pos);
                        frontier.push(new_path);
                    }
                }
            }
        }
    }
    paths[0].clone()
}

fn part1(data: &Parsed) -> i64 {
    let lp = get_loop(data);
    (lp.len() / 2) as i64
}

fn part2(data: &Parsed) -> i64 {
    let lp = get_loop(data);
    let mut data = data.clone();
    let ([min_x, min_y], [max_x, max_y]) = data.extents();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !lp.contains(&[x, y]) {
                data[y as usize][x as usize] = '.';
            }
        }
    }
    let mut right = HashSet::new();
    let mut left = HashSet::new();
    for mv in lp.windows(2) {
        let dir = aoc::point_sub(mv[1], mv[0]);
        let r = *aoc::DIRECTION_ROTATE_RIGHT.get(&dir).unwrap();
        let nbr = aoc::point_add(mv[0], r);
        let nbr2 = aoc::point_add(mv[1], r);
        if let Some(c) = data.get_value(nbr) {
            if c == '.' {
                right.insert(nbr);
            }
        }
        if let Some(c) = data.get_value(nbr2) {
            if c == '.' {
                right.insert(nbr2);
            }
        }
        let l = *aoc::DIRECTION_ROTATE_LEFT.get(&dir).unwrap();
        let nbr = aoc::point_add(mv[0], l);
        let nbr2 = aoc::point_add(mv[1], l);
        if let Some(c) = data.get_value(nbr) {
            if c == '.' {
                left.insert(nbr);
            }
        }
        if let Some(c) = data.get_value(nbr2) {
            if c == '.' {
                left.insert(nbr2);
            }
        }
    }
    for p in right {
        data.fill(p, 'r');
    }
    for p in left {
        data.fill(p, 'l');
    }
    let ([min_x, min_y], [max_x, max_y]) = data.extents();
    let mut inside = '.';
    let edge = (min_x..=max_x)
        .map(|x| [x, 0])
        .chain((min_x..=max_x).map(|x| [x, max_y]))
        .chain((min_y..=max_y).map(|y| [0, y]))
        .chain((min_y..=max_y).map(|y| [max_x, y]));
    for p in edge {
        match data.get_value(p) {
            Some('l') => {
                inside = 'r';
                break;
            }
            Some('r') => {
                inside = 'l';
                break;
            }
            _ => (),
        }
    }
    let unknown = data
        .points()
        .filter_map(|x| data.get_value(x))
        .filter(|x| *x == '.')
        .count() as i64;
    assert_eq!(unknown, 0);
    assert_ne!(inside, '.');
    data.points()
        .filter_map(|x| data.get_value(x))
        .filter(|x| *x == inside)
        .count() as i64
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "..F7.".into(),
            ".FJ|.".into(),
            "SJ.L7".into(),
            "|F--J".into(),
            "LJ...".into(),
        ]
    }

    fn example2() -> Vec<String> {
        vec![
            ".F----7F7F7F7F-7....".into(),
            ".|F--7||||||||FJ....".into(),
            ".||.FJ||||||||L7....".into(),
            "FJL7L7LJLJ||LJ.L-7..".into(),
            "L--J.L7...LJS7F-7L7.".into(),
            "....F-J..F7FJ|L7L7L7".into(),
            "....L7.F7||L7|.L7L7|".into(),
            ".....|FJLJ|FJ|F7|.LJ".into(),
            "....FJL-7.||.||||...".into(),
            "....L---J.LJ.LJLJ...".into(),
        ]
    }

    fn example3() -> Vec<String> {
        vec![
            "FF7FSF7F7F7F7F7F---7".into(),
            "L|LJ||||||||||||F--J".into(),
            "FL-7LJLJ||||||LJL-77".into(),
            "F--JF--7||LJLJ7F7FJ-".into(),
            "L---JF-JLJ.||-FJLJJ7".into(),
            "|F|F-JF---7F7-L7L|7|".into(),
            "|FFJF7L7F-JF7|JL---7".into(),
            "7-L-JL7||F7|L7F-7F7|".into(),
            "L.L7LFJ|||||FJL7||LJ".into(),
            "L7JLJL-JLJLJL--JLJ.L".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example2())), 8);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&parse(&example3())), 10);
    }
}
