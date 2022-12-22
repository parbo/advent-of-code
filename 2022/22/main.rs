use std::{collections::HashMap, iter::*};

use aoc::{
    CursesGridDrawer, GridDrawer, Point, PrintGridDrawer, DIRECTION_ROTATE_LEFT,
    DIRECTION_ROTATE_RIGHT, EAST, NORTH, SOUTH, WEST,
};

#[derive(Debug, Copy, Clone)]
enum Move {
    Step(i64),
    Left,
    Right,
}

type Parsed = (HashMap<Point, char>, Vec<Move>);

fn dir_c(d: Point) -> char {
    match d {
        EAST => '>',
        SOUTH => 'v',
        WEST => '<',
        NORTH => '^',
        _ => unreachable!(),
    }
}

fn draw<F>(gd: &mut CursesGridDrawer<F, char>, grid: &HashMap<Point, char>, path: &[(Point, Point)])
where
    F: Fn(char) -> char,
{
    let mut g = grid.clone();
    for (p, d) in path {
        let c = dir_c(*d);
        g.insert(*p, c);
    }
    gd.draw(&g);
}

fn part1(data: &Parsed) -> i64 {
    // let mut gd = CursesGridDrawer::new(|c| c);
    let (grid, moves) = data;
    let mut pos = *grid.keys().filter(|p| p[1] == 0).min().unwrap();
    let mut dir = EAST;
    let mut path = vec![(pos, dir)];
    println!("{:?}", moves);
    for m in moves {
        println!("pos: {:?}, dir: {:?}, move: {:?}", pos, dir_c(dir), m);
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
                        // draw(&mut gd, grid, &path);
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
                        // draw(&mut gd, grid, &path);
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
                        // draw(&mut gd, grid, &path);
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
                        // draw(&mut gd, grid, &path);
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

fn part2(_: &Parsed) -> i64 {
    0
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
                println!("{}, {}, {}", d, s, x);
                d = (d * s) + x.to_digit(10).unwrap() as i64;
                s *= 10;
            }
        }
    }
    if d != 0 {
        moves.push(Move::Step(d));
        s = 1;
        d = 0;
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
