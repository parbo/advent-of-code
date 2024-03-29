use aoc::{FxHashSet, Grid, Point, EAST, NORTH, SOUTH, WEST};
use std::iter::*;

type Parsed = Vec<Vec<char>>;

pub fn find_all<T>(
    grid: &dyn Grid<T>,
    is_node: fn(&Point, &Point, &T) -> bool,
    start: Point,
    goal: Point,
) -> Vec<Vec<Point>>
where
    T: PartialEq + Copy,
{
    let mut paths = vec![];
    let mut frontier = vec![(start, vec![])];
    while let Some((current, path)) = frontier.pop() {
        if current == goal {
            paths.push(path);
            continue;
        }
        for nb in aoc::neighbors(current) {
            if path.contains(&nb) {
                continue;
            }
            if let Some(value) = grid.get_value(nb) {
                if is_node(&current, &nb, &value) {
                    let mut pp = path.clone();
                    pp.push(nb);
                    frontier.push((nb, pp));
                }
            }
        }
    }
    paths
}

fn part1(data: &Parsed) -> i64 {
    let g = data[data.len() - 1]
        .iter()
        .enumerate()
        .find(|(_i, x)| **x == '.')
        .unwrap()
        .0;
    let r = find_all(
        data,
        |old, p, c| {
            let dir = aoc::point_sub(*p, *old);
            match dir {
                NORTH => ['.', '^'].contains(c),
                EAST => ['.', '>'].contains(c),
                SOUTH => ['.', 'v'].contains(c),
                WEST => ['.', '<'].contains(c),
                _ => panic!(),
            }
        },
        [1, 0],
        [g as i64, data.len() as i64 - 1],
    );
    let s = r.iter().map(|x| x.len()).max().unwrap() as i64;
    s
}

pub fn find_all_g(edges: &[(usize, usize, i64)], start: usize, goal: usize) -> Vec<i64> {
    let mut paths = vec![];
    let mut frontier = vec![(start, aoc::BitSet::new(), 0)];
    while let Some((current, path, lp)) = frontier.pop() {
        if current == goal {
            paths.push(lp);
            continue;
        }
        for (a, b, c) in edges {
            if *a == current && !path.test(*b) {
                let mut pp = path.clone();
                pp.set(*b, true);
                frontier.push((*b, pp, lp + c));
            }
        }
    }
    paths
}

pub fn find_junctions<T>(
    grid: &dyn Grid<T>,
    is_node: fn(T) -> bool,
    start: Point,
    goal: Point,
) -> Vec<(Point, Point, i64)>
where
    T: PartialEq + Copy,
{
    let mut junctions = vec![];
    let mut frontier = vec![(start, vec![start], 0)];
    let mut seen = FxHashSet::default();
    while let Some((current, jp, d)) = frontier.pop() {
        let mut poss = vec![];
        for nb in aoc::neighbors(current) {
            if let Some(value) = grid.get_value(nb) {
                if is_node(value) {
                    poss.push(nb);
                }
            }
        }
        let is_junction = poss.len() > 2;
        if is_junction || current == goal {
            if let Some(p) = jp.last() {
                if current != *p {
                    junctions.push((current, *p, d));
                    junctions.push((*p, current, d));
                }
            }
        }
        for nb in &poss {
            let mut new_d = d + 1;
            let mut p = jp.clone();
            if is_junction || current == goal {
                p.push(current);
                new_d = 1;
            }
            if seen.insert((current, *nb)) {
                frontier.push((*nb, p, new_d));
            }
        }
    }
    junctions
}

fn part2(data: &Parsed) -> i64 {
    let g = data[data.len() - 1]
        .iter()
        .enumerate()
        .find(|(_i, x)| **x == '.')
        .unwrap()
        .0;
    let start = [1, 0];
    let goal = [g as i64, data.len() as i64 - 1];
    let j = find_junctions(data, |c| c != '#', start, goal);
    let nodes: FxHashSet<Point> = j.iter().map(|x| x.0).collect();
    let nodes: Vec<Point> = nodes.into_iter().collect();
    let get_ix = |p: Point| {
        nodes
            .iter()
            .enumerate()
            .find_map(|(j, x)| if *x == p { Some(j) } else { None })
            .unwrap()
    };
    let edges = j
        .iter()
        .map(|(a, b, c)| (get_ix(*a), get_ix(*b), *c))
        .collect::<Vec<_>>();
    let start = get_ix(start);
    let goal = get_ix(goal);
    let r = find_all_g(&edges, start, goal);
    r.into_iter().max().unwrap() as i64
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid(lines)
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
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 94);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 154);
    }
}
