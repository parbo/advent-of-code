use aoc::{Grid, GridDrawer, EAST, NORTH, SOUTH, WEST};
use std::{
    collections::{HashMap, HashSet},
    iter::*,
};

type Parsed = Vec<Vec<char>>;

fn get_loop(data: &Parsed) -> HashMap<aoc::Point, i64> {
    let s = data
        .points()
        .find(|p| data.get_value(*p).unwrap() == 'S')
        .unwrap();
    dbg!(s);
    let mut distances = HashMap::from([(s, 0)]);
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
        (EAST, vec!['-', 'J', '7']),
        (WEST, vec!['-', 'L', 'F']),
        (NORTH, vec!['|', 'F', '7']),
        (SOUTH, vec!['|', 'J', 'L']),
    ]);
    let mut frontier = vec![s];
    while let Some(pos) = frontier.pop() {
        let c = data.get_value(pos).unwrap();
        if let Some(nbs) = poss.get(&c) {
            let d = *distances.get(&pos).unwrap_or(&0);
            for nb in nbs {
                let new_pos = aoc::point_add(pos, *nb);
                if let Some(new_c) = data.get_value(new_pos) {
                    if poss2.get(nb).unwrap().contains(&new_c) {
                        let dd = d + 1;
                        let e = distances.entry(new_pos).or_insert(dd);
                        if dd <= *e {
                            *e = dd;
                            frontier.push(new_pos);
                        }
                    }
                }
            }
        }
    }
    distances
}

fn part1(data: &Parsed) -> i64 {
    let distances = get_loop(data);
    let mut gd = aoc::PrintGridDrawer::new(|_x: i64| '*');
    gd.draw(&distances);
    *distances.values().max().unwrap()
}

fn part2(_: &Parsed) -> i64 {
    0
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 8);
    }
}
