use aoc::{Point, DIRECTION_ROTATE_LEFT, DIRECTION_ROTATE_RIGHT, EAST, NORTH, SOUTH, WEST};
use std::{collections::HashMap, iter::*};

#[derive(Debug, Copy, Clone)]
enum Move {
    Step(i64),
    Left,
    Right,
}

type Parsed = (HashMap<Point, char>, Vec<Move>);

fn part1(data: &Parsed) -> i64 {
    let (grid, moves) = data;
    let mut pos = *grid.keys().filter(|p| p[1] == 0).min().unwrap();
    let mut dir = EAST;
    let mut path = vec![(pos, dir)];
    for m in moves {
        let min_x = grid
            .keys()
            .filter_map(|p| if p[1] == pos[1] { Some(p[0]) } else { None })
            .min()
            .unwrap();
        let max_x = grid
            .keys()
            .filter_map(|p| if p[1] == pos[1] { Some(p[0]) } else { None })
            .max()
            .unwrap();
        let min_y = grid
            .keys()
            .filter_map(|p| if p[0] == pos[0] { Some(p[1]) } else { None })
            .min()
            .unwrap();
        let max_y = grid
            .keys()
            .filter_map(|p| if p[0] == pos[0] { Some(p[1]) } else { None })
            .max()
            .unwrap();
        match m {
            Move::Step(x) => {
                for _ in 0..*x {
                    let mut p = pos;
                    match dir {
                        EAST => {
                            p[0] += 1;
                            if p[0] > max_x {
                                p[0] = min_x;
                            }
                        }
                        WEST => {
                            p[0] -= 1;
                            if p[0] < min_x {
                                p[0] = max_x;
                            }
                        }
                        NORTH => {
                            p[1] -= 1;
                            if p[1] < min_y {
                                p[1] = max_y;
                            }
                        }
                        SOUTH => {
                            p[1] += 1;
                            if p[1] > max_y {
                                p[1] = min_y;
                            }
                        }
                        _ => unreachable!(),
                    }
                    if *grid.get(&p).unwrap() == '#' {
                        break;
                    }
                    pos = p;
                    path.push((pos, dir));
                }
            }
            Move::Left => {
                dir = *DIRECTION_ROTATE_LEFT.get(&dir).unwrap();
                path.push((pos, dir));
            }
            Move::Right => {
                dir = *DIRECTION_ROTATE_RIGHT.get(&dir).unwrap();
                path.push((pos, dir));
            }
        }
    }
    let facing = match dir {
        EAST => 0,
        SOUTH => 1,
        WEST => 2,
        NORTH => 3,
        _ => unreachable!(),
    };
    1000 * (pos[1] + 1) + 4 * (pos[0] + 1) + facing
}

fn inside(p: Point, e: (i64, i64, i64, i64)) -> bool {
    p[0] >= e.0 && p[0] <= e.1 && p[1] >= e.2 && p[1] <= e.3
}

fn transition(
    face: i64,
    dir: Point,
    pos: Point,
    extents: &HashMap<i64, (i64, i64, i64, i64)>,
) -> (i64, Point, Point) {
    let [x, y] = pos;
    let (f, d, dp) = match (face, dir) {
        (0, EAST) => (1, EAST, [0, y]),
        (0, WEST) => (3, EAST, [0, 49 - y]),
        (0, NORTH) => (5, EAST, [0, x]),
        (0, SOUTH) => (2, SOUTH, [x, 0]),
        (1, EAST) => (4, WEST, [49, 49 - y]),
        (1, WEST) => (0, WEST, [49, y]),
        (1, NORTH) => (5, NORTH, [x, 49]),
        (1, SOUTH) => (2, WEST, [49, x]),
        (2, EAST) => (1, NORTH, [y, 49]),
        (2, WEST) => (3, SOUTH, [y, 0]),
        (2, NORTH) => (0, NORTH, [x, 49]),
        (2, SOUTH) => (4, SOUTH, [x, 0]),
        (3, EAST) => (4, EAST, [0, y]),
        (3, WEST) => (0, EAST, [0, 49 - y]),
        (3, NORTH) => (2, EAST, [0, x]),
        (3, SOUTH) => (5, SOUTH, [x, 0]),
        (4, EAST) => (1, WEST, [49, 49 - y]),
        (4, WEST) => (3, WEST, [49, y]),
        (4, NORTH) => (2, NORTH, [x, 49]),
        (4, SOUTH) => (5, WEST, [49, x]),
        (5, EAST) => (4, NORTH, [y, 49]),
        (5, WEST) => (0, SOUTH, [y, 0]),
        (5, NORTH) => (3, NORTH, [x, 49]),
        (5, SOUTH) => (1, SOUTH, [x, 0]),
        _ => unreachable!(),
    };
    let e = *extents.get(&f).unwrap();
    let p = [e.0 + dp[0], e.2 + dp[1]];
    assert!(inside(p, e));
    (f, d, p)
}

fn part2(data: &Parsed) -> i64 {
    let (grid, moves) = data;

    let mut pos = *grid.keys().filter(|p| p[1] == 0).min().unwrap();
    let mut dir = EAST;
    let mut path = vec![(pos, dir)];
    let sz = 50;
    let extents: HashMap<i64, (i64, i64, i64, i64)> = [
        (0, (sz, 2 * sz - 1, 0, sz - 1)),
        (1, (2 * sz, 3 * sz - 1, 0, sz - 1)),
        (2, (sz, 2 * sz - 1, sz, 2 * sz - 1)),
        (3, (0, sz - 1, 2 * sz, 3 * sz - 1)),
        (4, (sz, 2 * sz - 1, 2 * sz, 3 * sz - 1)),
        (5, (0, sz - 1, 3 * sz, 4 * sz - 1)),
    ]
    .into_iter()
    .collect();

    let mut face = 0;
    for m in moves {
        match m {
            Move::Step(x) => {
                for _ in 0..*x {
                    let e = *extents.get(&face).unwrap();
                    let (min_x, max_x, min_y, max_y) = e;
                    let mut p = pos;
                    let x = p[0] - min_x;
                    let y = p[1] - min_y;
                    assert!(inside(p, e));
                    let mut d = dir;
                    let mut f = face;
                    match dir {
                        EAST => {
                            p[0] += 1;
                            if p[0] > max_x {
                                (f, d, p) = transition(face, dir, [x, y], &extents);
                            }
                        }
                        WEST => {
                            p[0] -= 1;
                            if p[0] < min_x {
                                (f, d, p) = transition(face, dir, [x, y], &extents);
                            }
                        }
                        NORTH => {
                            p[1] -= 1;
                            if p[1] < min_y {
                                (f, d, p) = transition(face, dir, [x, y], &extents);
                            }
                        }
                        SOUTH => {
                            p[1] += 1;
                            if p[1] > max_y {
                                (f, d, p) = transition(face, dir, [x, y], &extents);
                            }
                        }
                        _ => unreachable!(),
                    }
                    if *grid.get(&p).unwrap() == '#' {
                        break;
                    }
                    pos = p;
                    dir = d;
                    face = f;
                    path.push((pos, dir));
                }
            }
            Move::Left => {
                dir = *DIRECTION_ROTATE_LEFT.get(&dir).unwrap();
                path.push((pos, dir));
            }
            Move::Right => {
                dir = *DIRECTION_ROTATE_RIGHT.get(&dir).unwrap();
                path.push((pos, dir));
            }
        }
    }

    let facing = match dir {
        EAST => 0,
        SOUTH => 1,
        WEST => 2,
        NORTH => 3,
        _ => unreachable!(),
    };
    1000 * (pos[1] + 1) + 4 * (pos[0] + 1) + facing
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    let grid = aoc::parse_grid_to_sparse(
        &parts[0]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        |c| {
            if c != ' ' {
                Some(c)
            } else {
                None
            }
        },
    );
    let mut moves = vec![];
    let mut d = 0;
    let mut s = 1;
    for c in parts[1][0].chars() {
        match c {
            'L' => {
                if d != 0 {
                    moves.push(Move::Step(d));
                    s = 1;
                    d = 0;
                }
                moves.push(Move::Left);
            }
            'R' => {
                if d != 0 {
                    moves.push(Move::Step(d));
                    s = 1;
                    d = 0;
                }
                moves.push(Move::Right);
            }
            x => {
                d = (d * s) + x.to_digit(10).unwrap() as i64;
                s *= 10;
            }
        }
    }
    if d != 0 {
        moves.push(Move::Step(d));
    }
    (grid, moves)
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
        assert_eq!(part1(&parse(&example())), 6032);
    }
}
