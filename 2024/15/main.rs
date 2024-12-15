use std::iter::*;

use aoc::Grid;

#[cfg(feature = "vis")]
use aoc::GridDrawer;

type Parsed = (Vec<Vec<char>>, Vec<char>);

fn part1(data: &Parsed) -> i64 {
    let mut g = data.0.clone();
    #[cfg(feature = "vis")]
    let mut gd = aoc::make_bitmap_text_grid_drawer(
        |x| {
            let c = if x == '@' {
                [0xff, 0xff, 0xff]
            } else if x == '#' {
                [0xff, 0, 0]
            } else if x == 'O' || x == '[' || x == ']' {
                [0, 0xff, 0]
            } else {
                [0, 0, 0]
            };
            (x, c)
        },
        "vis/15/day15a",
    );
    #[cfg(feature = "vis")]
    gd.draw(&g);
    let mut pos = g.points().find(|p| g.get_value(*p) == Some('@')).unwrap();
    for m in &data.1 {
        let dir = match m {
            '>' => aoc::EAST,
            '^' => aoc::NORTH,
            '<' => aoc::WEST,
            'v' => aoc::SOUTH,
            _ => panic!(),
        };
        let mut newp = aoc::point_add(pos, dir);
        match g.get_value(newp) {
            Some('.') => {
                g.set_value(newp, '@');
                g.set_value(pos, '.');
                pos = newp;
            }
            Some('#') => {
                // Nothing
            }
            Some('O') => {
                let pp = newp;
                loop {
                    newp = aoc::point_add(newp, dir);
                    match g.get_value(newp) {
                        Some('.') => {
                            g.set_value(pp, '@');
                            g.set_value(pos, '.');
                            g.set_value(newp, 'O');
                            pos = pp;
                            break;
                        }
                        Some('#') => {
                            // Nothing
                            break;
                        }
                        Some('O') => {
                            // Keep going
                        }
                        _ => panic!(),
                    }
                }
            }
            _ => panic!(),
        }
        #[cfg(feature = "vis")]
        gd.draw(&g);
    }
    g.points()
        .filter_map(|p| {
            let v = g.get_value(p);
            if v == Some('O') {
                Some(p[0] + 100 * p[1])
            } else {
                None
            }
        })
        .sum()
}

fn part2(data: &Parsed) -> i64 {
    let mut g = vec![vec!['X'; 2 * data.0[0].len()]; data.0.len()];
    #[cfg(feature = "vis")]
    let mut gd = aoc::make_bitmap_text_grid_drawer(
        |x| {
            let c = if x == '@' {
                [0xff, 0xff, 0xff]
            } else if x == '#' {
                [0xff, 0, 0]
            } else if x == 'O' || x == '[' || x == ']' {
                [0, 0xff, 0]
            } else {
                [0, 0, 0]
            };
            (x, c)
        },
        "vis/15/day15a",
    );
    #[cfg(feature = "vis")]
    gd.draw(&g);
    for p in data.0.points() {
        let p1 = [2 * p[0], p[1]];
        let p2 = [2 * p[0] + 1, p[1]];
        match data.0.get_value(p) {
            Some('#') => {
                g.set_value(p1, '#');
                g.set_value(p2, '#');
            }
            Some('O') => {
                g.set_value(p1, '[');
                g.set_value(p2, ']');
            }
            Some('@') => {
                g.set_value(p1, '@');
                g.set_value(p2, '.');
            }
            Some('.') => {
                g.set_value(p1, '.');
                g.set_value(p2, '.');
            }
            _ => {}
        }
    }
    let mut pos = g.points().find(|p| g.get_value(*p) == Some('@')).unwrap();
    for m in &data.1 {
        let dir = match m {
            '>' => aoc::EAST,
            '^' => aoc::NORTH,
            '<' => aoc::WEST,
            'v' => aoc::SOUTH,
            _ => panic!(),
        };
        let mut newp = aoc::point_add(pos, dir);
        match g.get_value(newp) {
            Some('.') => {
                g.set_value(newp, '@');
                g.set_value(pos, '.');
                pos = newp;
            }
            Some('#') => {
                // Nothing
            }
            Some('[') | Some(']') => {
                if *m == '<' || *m == '>' {
                    let pp = newp;
                    loop {
                        newp = aoc::point_add(newp, dir);
                        match g.get_value(newp) {
                            Some('.') => {
                                g.set_value(pp, '@');
                                g.set_value(pos, '.');
                                pos = pp;
                                // Fill in with boxes
                                let mut ppp = aoc::point_add(pp, dir);
                                let mut c = if *m == '>' { '[' } else { ']' };
                                loop {
                                    g.set_value(ppp, c);
                                    if ppp == newp {
                                        break;
                                    }
                                    c = if c == '[' { ']' } else { '[' };
                                    ppp = aoc::point_add(ppp, dir);
                                }
                                break;
                            }
                            Some('#') => {
                                // Nothing
                                break;
                            }
                            Some('[') => {
                                // Keep going
                            }
                            Some(']') => {
                                // Keep going
                            }
                            _ => panic!(),
                        }
                    }
                } else if *m == '^' || *m == 'v' {
                    let mut boxpos = aoc::FxHashSet::default();
                    let mut todo = vec![pos];
                    while let Some(pp) = todo.pop() {
                        if g.get_value(pp) != Some('@') && !boxpos.insert(pp) {
                            continue;
                        }
                        let ppp = aoc::point_add(pp, dir);
                        match g.get_value(ppp) {
                            Some('[') => {
                                todo.push(ppp);
                                todo.push([ppp[0] + 1, ppp[1]]);
                            }
                            Some(']') => {
                                todo.push(ppp);
                                todo.push([ppp[0] - 1, ppp[1]]);
                            }
                            Some('.') => {}
                            Some('#') => {
                                boxpos.clear();
                                break;
                            }
                            _ => panic!(),
                        }
                    }
                    let mut newvals = vec![];
                    for bp in &boxpos {
                        let bpp = aoc::point_add(*bp, dir);
                        newvals.push((bpp, g.get_value(*bp).unwrap()));
                        g.set_value(*bp, '.');
                    }
                    for (bp, val) in newvals {
                        g.set_value(bp, val);
                    }
                    if !boxpos.is_empty() {
                        g.set_value(newp, '@');
                        g.set_value(pos, '.');
                        pos = newp;
                    }
                }
            }
            _ => panic!(),
        }
        #[cfg(feature = "vis")]
        gd.draw(&g);
    }
    g.points()
        .filter_map(|p| {
            let v = g.get_value(p);
            if v == Some('[') {
                Some(p[0] + 100 * p[1])
            } else {
                None
            }
        })
        .sum()
}

fn parse(lines: &[String]) -> Parsed {
    let parts = aoc::split_by_empty_line(lines);
    let s = parts[1].join("");
    (aoc::parse_grid(&parts[0]), s.chars().collect())
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

    fn example2() -> Vec<String> {
        let s = include_str!("smallexample.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 9021);
    }

    #[test]
    fn test_part2_small() {
        assert_eq!(part2(&parse(&example2())), 618);
    }
}
