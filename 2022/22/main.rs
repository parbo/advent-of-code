use std::{collections::HashMap, iter::*};

use aoc::{
    Grid, GridDrawer, Point, PrintGridDrawer, DIRECTION_ROTATE_LEFT, DIRECTION_ROTATE_RIGHT, EAST,
    NORTH, SOUTH, WEST,
};

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
        match m {
            Move::Step(x) => match dir {
                EAST => {
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
                    for _ in 0..*x {
                        let mut p = pos;
                        p[0] += 1;
                        if p[0] > max_x {
                            p[0] = min_x;
                        }
                        if *grid.get(&p).unwrap() == '#' {
                            break;
                        }
                        pos = p;
                        path.push((pos, dir));
                    }
                }
                WEST => {
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
                    for _ in 0..*x {
                        let mut p = pos;
                        p[0] -= 1;
                        if p[0] < min_x {
                            p[0] = max_x;
                        }
                        if *grid.get(&p).unwrap() == '#' {
                            break;
                        }
                        pos = p;
                        path.push((pos, dir));
                    }
                }
                NORTH => {
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
                    for _ in 0..*x {
                        let mut p = pos;
                        p[1] -= 1;
                        if p[1] < min_y {
                            p[1] = max_y;
                        }
                        if *grid.get(&p).unwrap() == '#' {
                            break;
                        }
                        pos = p;
                        path.push((pos, dir));
                    }
                }
                SOUTH => {
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
                    for _ in 0..*x {
                        let mut p = pos;
                        p[1] += 1;
                        if p[1] > max_y {
                            p[1] = min_y;
                        }
                        if *grid.get(&p).unwrap() == '#' {
                            break;
                        }
                        pos = p;
                        path.push((pos, dir));
                    }
                }
                _ => unreachable!(),
            },
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

fn dir_c(d: Point) -> char {
    match d {
        EAST => '>',
        SOUTH => 'v',
        WEST => '<',
        NORTH => '^',
        _ => unreachable!(),
    }
}

fn draw(grid: &HashMap<Point, char>, path: &[(Point, Point)]) {
    let mut gd = PrintGridDrawer::new(|c| c);
    let mut g = grid.clone();
    for (p, d) in path {
        let c = dir_c(*d);
        g.insert(*p, c);
    }
    gd.draw(&g);
}

fn inside(p: Point, e: (i64, i64, i64, i64)) -> bool {
    p[0] >= e.0 && p[0] <= e.1 && p[1] >= e.2 && p[1] <= e.3
}

fn part2(data: &Parsed) -> i64 {
    let (grid, moves) = data;

    let mut pos = *grid.keys().filter(|p| p[1] == 0).min().unwrap();
    let mut dir = EAST;
    let mut path = vec![(pos, dir)];
    let sz = grid
        .keys()
        .filter_map(|p| if p[1] == 0 { Some(p[0]) } else { None })
        .max()
        .unwrap()
        - grid
            .keys()
            .filter_map(|p| if p[1] == 0 { Some(p[0]) } else { None })
            .min()
            .unwrap()
        + 1;
    let sz = sz / 2;
    let extents: HashMap<i64, (i64, i64, i64, i64)> = [
        (1, (sz, 2 * sz - 1, 0, sz - 1)),
        (2, (2 * sz, 3 * sz - 1, 0, sz - 1)),
        (3, (sz, 2 * sz - 1, sz, 2 * sz - 1)),
        (4, (0, sz - 1, 2 * sz, 3 * sz - 1)),
        (5, (sz, 2 * sz - 1, 2 * sz, 3 * sz - 1)),
        (6, (0, sz - 1, 3 * sz, 4 * sz - 1)),
    ]
    .into_iter()
    .collect();
    println!("{:?}", extents);

    let mut face = 1;
    for m in moves {
        dbg!(m);
        match m {
            Move::Step(x) => {
                for s in 0..*x {
                    let (min_x, max_x, min_y, max_y) = *extents.get(&face).unwrap();
                    let mut p = pos;
                    dbg!(face, pos, dir, s);
                    let mut d = dir;
                    let mut f = face;
                    match (face, dir) {
                        (1, EAST) => {
                            p[0] += 1;
                            if p[0] > max_x {
                                f = 2;
                                d = EAST;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2, min_y2 + (p[1] - min_y)];
                                assert!(inside(p, e));
                            }
                        }
                        (1, WEST) => {
                            p[0] -= 1;
                            if p[0] < min_x {
                                f = 4;
                                d = EAST;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2, max_y2 - (p[1] - min_y)];
                                assert!(inside(p, e));
                            }
                        }
                        (1, NORTH) => {
                            p[1] -= 1;
                            if p[1] < min_y {
                                f = 6;
                                d = EAST;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2, min_y2 + (p[0] - min_x)];
                                assert!(inside(p, e));
                            }
                        }
                        (1, SOUTH) => {
                            p[1] += 1;
                            if p[1] > max_y {
                                f = 3;
                                d = SOUTH;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2 + (p[0] - min_x), min_y2];
                                assert!(inside(p, e));
                            }
                        }
                        (2, EAST) => {
                            p[0] += 1;
                            if p[0] > max_x {
                                f = 5;
                                d = WEST;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [max_x2, max_y2 - (p[1] - min_y)];
                                assert!(inside(p, e));
                            }
                        }
                        (2, WEST) => {
                            p[0] -= 1;
                            if p[0] < min_x {
                                f = 1;
                                d = WEST;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [max_x2, min_y2 + (p[1] - min_y)];
                                assert!(inside(p, e));
                            }
                        }
                        (2, NORTH) => {
                            p[1] -= 1;
                            if p[1] < min_y {
                                f = 6;
                                d = NORTH;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2 + (p[0] - min_x), max_y2];
                                assert!(inside(p, e));
                            }
                        }
                        (2, SOUTH) => {
                            p[1] += 1;
                            if p[1] > max_y {
                                f = 3;
                                d = WEST;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [max_x2, min_y2 + (p[0] - min_x)];
                                assert!(inside(p, e));
                            }
                        }
                        (3, EAST) => {
                            p[0] += 1;
                            if p[0] > max_x {
                                f = 2;
                                d = NORTH;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2 + (p[1] - min_y), max_y2];
                                assert!(inside(p, e));
                            }
                        }
                        (3, WEST) => {
                            p[0] -= 1;
                            if p[0] < min_x {
                                f = 4;
                                d = SOUTH;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2 + (p[1] - min_y), min_y2];
                                assert!(inside(p, e));
                            }
                        }
                        (3, NORTH) => {
                            p[1] -= 1;
                            if p[1] < min_y {
                                f = 1;
                                d = NORTH;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2 + (p[0] - min_x), max_y2];
                                assert!(inside(p, e));
                            }
                        }
                        (3, SOUTH) => {
                            p[1] += 1;
                            if p[1] > max_y {
                                f = 5;
                                d = SOUTH;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2 + (p[0] - min_x), min_y2];
                                assert!(inside(p, e));
                            }
                        }
                        (4, EAST) => {
                            p[0] += 1;
                            if p[0] > max_x {
                                f = 5;
                                d = EAST;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2, min_y2 + (p[1] - min_y)];
                                assert!(inside(p, e));
                            }
                        }
                        (4, WEST) => {
                            p[0] -= 1;
                            if p[0] < min_x {
                                f = 1;
                                d = EAST;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2, max_y2 - (p[1] - min_y)];
                                assert!(inside(p, e));
                            }
                        }
                        (4, NORTH) => {
                            p[1] -= 1;
                            if p[1] < min_y {
                                f = 3;
                                d = EAST;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2, min_y2 + (p[0] - min_x)];
                                assert!(inside(p, e));
                            }
                        }
                        (4, SOUTH) => {
                            p[1] += 1;
                            if p[1] > max_y {
                                f = 6;
                                d = SOUTH;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2 + (p[0] - min_x), min_y2];
                                assert!(inside(p, e));
                            }
                        }
                        (5, EAST) => {
                            p[0] += 1;
                            if p[0] > max_x {
                                f = 2;
                                d = WEST;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [max_x2, max_y2 - (p[1] - min_y)];
                                assert!(inside(p, e));
                            }
                        }
                        (5, WEST) => {
                            p[0] -= 1;
                            if p[0] < min_x {
                                f = 4;
                                d = WEST;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [max_x2, min_y2 + (p[1] - min_y)];
                                assert!(inside(p, e));
                            }
                        }
                        (5, NORTH) => {
                            let mut p = pos;
                            p[1] -= 1;
                            if p[1] < min_y {
                                f = 3;
                                d = NORTH;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2 + (p[0] - min_x), max_y2];
                                assert!(inside(p, e));
                            }
                        }
                        (5, SOUTH) => {
                            p[1] += 1;
                            if p[1] > max_y {
                                f = 6;
                                d = WEST;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [max_x2, min_y2 + (p[0] - min_x)];
                                assert!(inside(p, e));
                            }
                        }
                        (6, EAST) => {
                            p[0] += 1;
                            if p[0] > max_x {
                                f = 5;
                                d = NORTH;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2 + (p[1] - min_y), max_y2];
                                assert!(inside(p, e));
                            }
                        }
                        (6, WEST) => {
                            p[0] -= 1;
                            if p[0] < min_x {
                                f = 1;
                                d = SOUTH;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2 + (p[1] - min_y), min_y2];
                                assert!(inside(p, e));
                            }
                        }
                        (6, NORTH) => {
                            p[1] -= 1;
                            if p[1] < min_y {
                                f = 4;
                                d = NORTH;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2 + (p[0] - min_x), max_y2];
                                assert!(inside(p, e));
                            }
                        }
                        (6, SOUTH) => {
                            p[1] += 1;
                            if p[1] > max_y {
                                f = 2;
                                d = SOUTH;
                                let e = *extents.get(&f).unwrap();
                                let (min_x2, max_x2, min_y2, max_y2) = e;
                                p = [min_x2 + (p[0] - min_x), min_y2];
                                assert!(inside(p, e));
                            }
                        }
                        _ => unreachable!(),
                    }
                    dbg!(p, f, d);
                    if *grid.get(&p).unwrap() == '#' {
                        println!("wall!");
                        break;
                    }
                    pos = p;
                    dir = d;
                    face = f;
                    path.push((pos, dir));
                    // draw(grid, &path);
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
    dbg!(pos, facing);
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
