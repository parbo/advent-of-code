use aoc::Grid;
use aoc::*;

static DG: (u8, u8, u8) = (0, 100, 0);
static LG: (u8, u8, u8) = (0, 255, 0);
static OW: (u8, u8, u8) = (200, 200, 200);

fn part1(grid: &Vec<Vec<char>>) -> i64 {
    let mut g = grid.clone();
    let ([minx, miny], [maxx, maxy]) = grid.extents();
    let mut gd = aoc::BitmapGridDrawer::new(
        (2, 2),
        |x| match x {
            '#' => vec![LG, LG, LG, LG],
            'L' => vec![DG, DG, DG, DG],
            _ => vec![OW, OW, OW, OW],
        },
        "ppm/day11/part1",
    );
    loop {
        let mut newg = g.clone();
        gd.draw(&g);
        for y in miny..=maxy {
            for x in minx..=maxx {
                let p: Point = [x as i64, y as i64];
                if let Some(c) = g.get_value(p) {
                    let mut empty = 0;
                    let mut seats = 0;
                    let mut occupied = 0;
                    for d in &DIRECTIONS_INCL_DIAGONALS.clone() {
                        let np = point_add(p, *d);
                        if np[0] >= minx && np[0] <= maxx && np[1] >= miny && np[1] <= maxy {
                            match g.get_value(np) {
                                Some('L') => {
                                    empty += 1;
                                    seats += 1;
                                }
                                Some('#') => {
                                    occupied += 1;
                                    seats += 1;
                                }
                                _ => {}
                            }
                        }
                    }
                    if c == 'L' {
                        if empty == seats {
                            newg.set_value(p, '#');
                        }
                    } else if c == '#' {
                        if occupied >= 4 {
                            newg.set_value(p, 'L');
                        }
                    }
                }
            }
        }
        if g == newg {
            break;
        }
        g = newg.clone();
    }
    let mut occupied = 0;
    for y in miny..=maxy {
        for x in minx..=maxx {
            let p: Point = [x as i64, y as i64];
            if let Some('#') = g.get_value(p) {
                occupied += 1;
            }
        }
    }
    occupied
}

fn part2(grid: &Vec<Vec<char>>) -> i64 {
    let mut g = grid.clone();
    let ([minx, miny], [maxx, maxy]) = grid.extents();
    let mut gd = aoc::BitmapGridDrawer::new(
        (2, 2),
        |x| match x {
            '#' => vec![LG, LG, LG, LG],
            'L' => vec![DG, DG, DG, DG],
            _ => vec![OW, OW, OW, OW],
        },
        "ppm/day11/part2",
    );
    loop {
        let mut newg = g.clone();
        gd.draw(&g);
        for y in miny..=maxy {
            for x in minx..=maxx {
                let p: Point = [x as i64, y as i64];
                if let Some(c) = g.get_value(p) {
                    let mut empty = 0;
                    let mut seats = 0;
                    let mut occupied = 0;
                    for d in &DIRECTIONS_INCL_DIAGONALS.clone() {
                        let mut np = p;
                        loop {
                            np = point_add(np, *d);
                            if np[0] >= minx && np[0] <= maxx && np[1] >= miny && np[1] <= maxy {
                                match g.get_value(np) {
                                    Some('L') => {
                                        empty += 1;
                                        seats += 1;
                                        break;
                                    }
                                    Some('#') => {
                                        occupied += 1;
                                        seats += 1;
                                        break;
                                    }
                                    Some('.') => {}
                                    _ => panic!(),
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    if c == 'L' {
                        if empty == seats {
                            newg.set_value(p, '#');
                        }
                    } else if c == '#' {
                        if occupied >= 5 {
                            newg.set_value(p, 'L');
                        }
                    }
                }
            }
        }
        if g == newg {
            break;
        }
        g = newg.clone();
    }
    let mut occupied = 0;
    for y in miny..=maxy {
        for x in minx..=maxx {
            let p: Point = [x as i64, y as i64];
            if let Some('#') = g.get_value(p) {
                occupied += 1;
            }
        }
    }
    occupied
}

fn parse(lines: &[String]) -> Vec<Vec<char>> {
    aoc::parse_grid(lines)
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut g: Vec<String> = vec![];
        g.push("L.LL.LL.LL".into());
        g.push("LLLLLLL.LL".into());
        g.push("L.L.L..L..".into());
        g.push("LLLL.LL.LL".into());
        g.push("L.LL.LL.LL".into());
        g.push("L.LLLLL.LL".into());
        g.push("..L.L.....".into());
        g.push("LLLLLLLLLL".into());
        g.push("L.LLLLLL.L".into());
        g.push("L.LLLLL.LL".into());
        let grid = parse_grid(&g);
        assert_eq!(part1(&grid), 37);
    }
}
