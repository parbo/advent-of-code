use aoc::Grid;
use aoc::*;

type Parsed = Vec<Vec<char>>;

static DG: [u8; 3] = [0, 100, 0];
static LG: [u8; 3] = [0, 255, 0];
static OW: [u8; 3] = [200, 200, 200];

fn part1(grid: &Parsed) -> usize {
    let mut g = grid.to_owned();
    let mut gd = aoc::BitmapSpriteGridDrawer::new(
        (2, 2),
        |x| match x {
            '#' => vec![LG, LG, LG, LG],
            'L' => vec![DG, DG, DG, DG],
            _ => vec![OW, OW, OW, OW],
        },
        "ppm/day11/part1",
    );
    let d = cfg!(feature = "vis");
    loop {
        let mut newg = g.clone();
        if d {
            gd.draw(&g);
        }
        for p in g.points() {
            if let Some(c) = g.get_value(p) {
                let mut empty = 0;
                let mut seats = 0;
                let mut occupied = 0;
                for d in &DIRECTIONS_INCL_DIAGONALS.clone() {
                    let np = point_add(p, *d);
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
                if c == 'L' && empty == seats {
                    newg.set_value(p, '#');
                } else if c == '#' && occupied >= 4 {
                    newg.set_value(p, 'L');
                }
            }
        }
        if g == newg {
            break;
        }
        g = newg.clone();
    }
    g.points().filter(|p| g.get_value(*p) == Some('#')).count()
}

fn part2(grid: &Parsed) -> usize {
    let mut g = grid.to_owned();
    let mut gd = aoc::BitmapSpriteGridDrawer::new(
        (2, 2),
        |x| match x {
            '#' => vec![LG, LG, LG, LG],
            'L' => vec![DG, DG, DG, DG],
            _ => vec![OW, OW, OW, OW],
        },
        "ppm/day11/part2",
    );
    let d = cfg!(feature = "vis");
    loop {
        let mut newg = g.clone();
        if d {
            gd.draw(&g);
        }
        for p in g.points() {
            if let Some(c) = g.get_value(p) {
                let mut empty = 0;
                let mut seats = 0;
                let mut occupied = 0;
                for d in &DIRECTIONS_INCL_DIAGONALS.clone() {
                    let mut np = p;
                    loop {
                        np = point_add(np, *d);
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
                            None => break,
                            _ => panic!(),
                        }
                    }
                }
                if c == 'L' && empty == seats {
                    newg.set_value(p, '#');
                } else if c == '#' && occupied >= 5 {
                    newg.set_value(p, 'L');
                }
            }
        }
        if g == newg {
            break;
        }
        g = newg.clone();
    }
    g.points().filter(|p| g.get_value(*p) == Some('#')).count()
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
