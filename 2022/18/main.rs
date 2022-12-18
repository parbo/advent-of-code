use std::{collections::HashSet, iter::*};

use aoc::{vec_add, Vec3};

type ParsedItem = Vec3;
type Parsed = HashSet<ParsedItem>;
type Answer = i64;

fn area(data: &Parsed) -> Answer {
    let mut exposed = 0;
    for cube in data {
        for nb in [
            [-1, 0, 0],
            [0, -1, 0],
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, -1],
            [0, 0, 1],
        ] {
            let p = vec_add(*cube, nb);
            if !data.contains(&p) {
                exposed += 1;
            }
        }
    }
    exposed
}

fn fill(
    droplet: &HashSet<Vec3>,
    pos: Vec3,
    extents: ([i64; 3], [i64; 3]),
) -> Option<HashSet<Vec3>> {
    let ([min_x, min_y, min_z], [max_x, max_y, max_z]) = extents;
    if !droplet.contains(&pos) {
        let mut ret = droplet.clone();
        let mut todo = vec![];
        todo.push(pos);
        while let Some(p) = todo.pop() {
            if p[0] < min_x
                || p[0] > max_x
                || p[1] < min_y
                || p[1] > max_y
                || p[2] < min_z
                || p[2] > max_z
            {
                return None;
            }
            if !ret.contains(&p) {
                ret.insert(p);
                for nb in [
                    [-1, 0, 0],
                    [0, -1, 0],
                    [1, 0, 0],
                    [0, 1, 0],
                    [0, 0, -1],
                    [0, 0, 1],
                ] {
                    todo.push(vec_add(p, nb));
                }
            }
        }
        Some(ret)
    } else {
        None
    }
}

fn part1(data: &Parsed) -> Answer {
    area(data)
}

fn part2(data: &Parsed) -> Answer {
    let minx = data.iter().map(|p| p[0]).min().unwrap();
    let maxx = data.iter().map(|p| p[0]).max().unwrap();
    let miny = data.iter().map(|p| p[1]).min().unwrap();
    let maxy = data.iter().map(|p| p[1]).max().unwrap();
    let minz = data.iter().map(|p| p[2]).min().unwrap();
    let maxz = data.iter().map(|p| p[2]).max().unwrap();
    let mut d = data.clone();
    for x in minx..=maxx {
        for y in miny..=maxy {
            for z in minz..=maxz {
                if let Some(dd) = fill(&d, [x, y, z], ([minx, miny, minz], [maxx, maxy, maxz])) {
                    d = dd;
                }
            }
        }
    }
    area(&d)
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let v = aoc::things::<i64>(x);
            [v[0], v[1], v[2]]
        })
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
            "2,2,2".into(),
            "1,2,2".into(),
            "3,2,2".into(),
            "2,1,2".into(),
            "2,3,2".into(),
            "2,2,1".into(),
            "2,2,3".into(),
            "2,2,4".into(),
            "2,2,6".into(),
            "1,2,5".into(),
            "3,2,5".into(),
            "2,1,5".into(),
            "2,3,5".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 58);
    }
}
