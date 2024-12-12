use std::{collections::BTreeSet, iter::*};

use aoc::Grid;
use aoc::GridDrawer;

type Parsed = Vec<Vec<char>>;

fn part1(data: &Parsed) -> i64 {
    let crops: BTreeSet<_> = data.points().filter_map(|p| data.get_value(p)).collect();
    let mut cost = 0;
    let mut g = data.clone();
    for c in crops {
        let ext = data.extents();
        while let Some(p) = g.points().find(|p| g.get_value(*p) == Some(c)) {
            g.fill(p, '.');
            let mut area = 0;
            let mut perimeter = 0;
            for y in (ext.0[1] - 1)..=(ext.1[1] + 1) {
                for x in (ext.0[0] - 1)..=(ext.1[0] + 1) {
                    if g.get_value([x, y]) != Some('.') {
                        for nb in aoc::neighbors([x, y]) {
                            if g.get_value(nb) == Some('.') {
                                // println!("{}: per {:?} {:?}", c, nb, [x, y]);
                                perimeter += 1;
                            }
                        }
                    } else {
                        area += 1;
                    }
                }
            }
            g.fill(p, ' ');
            // dbg!(c, area, perimeter);
            cost += area * perimeter;
        }
    }
    cost
}

fn part2(data: &Parsed) -> i64 {
    let crops: BTreeSet<_> = data.points().filter_map(|p| data.get_value(p)).collect();
    let mut cost = 0;
    let mut g = data.clone();
    #[cfg(feature = "vis")]
    let mut gd = aoc::BitmapSpriteGridDrawer::new(
        (
            aoc::SMALLFONT.glyph_size().0 as i64,
            aoc::SMALLFONT.glyph_size().1 as i64,
        ),
        |x| {
            let mut v = vec![];
            let g = aoc::SMALLFONT.glyph(x as u32).unwrap();
            for y in 0..aoc::SMALLFONT.glyph_size().1 {
                for x in 0..aoc::SMALLFONT.glyph_size().0 {
                    let byte = y + (x / 8);
                    let bit = 7 - (x % 8);
                    if (g[byte as usize] & (1 << bit)) == 0 {
                        v.push([0xff, 0xff, 0xff]);
                    } else {
                        v.push([0x0, 0x0, 0x0]);
                    }
                }
            }
            v
        },
        "vis/day12",
    );
    for c in crops {
        let ext = data.extents();
        while let Some(p) = g.points().find(|p| g.get_value(*p) == Some(c)) {
            g.fill(p, '.');
            #[cfg(feature = "vis")]
            gd.draw(&g);
            let mut area = 0;
            let mut perimeter = BTreeSet::new();
            for y in (ext.0[1] - 1)..=(ext.1[1] + 1) {
                for x in (ext.0[0] - 1)..=(ext.1[0] + 1) {
                    if g.get_value([x, y]) != Some('.') {
                        for nb in aoc::neighbors([x, y]) {
                            if g.get_value(nb) == Some('.') {
                                perimeter.insert((nb, aoc::point_sub([x, y], nb)));
                            }
                        }
                    } else {
                        area += 1;
                    }
                }
            }
            let mut sides = 0;
            let mut counted = aoc::FxHashSet::default();
            for (p, dir) in &perimeter {
                if counted.insert((*p, *dir)) {
                    sides += 1;
                    // Find all neighbors with the same fence direction and mark them as counted too
                    let mut todo = vec![*p];
                    while let Some(pp) = todo.pop() {
                        for nb in aoc::neighbors(pp) {
                            if perimeter
                                .iter()
                                .any(|(ppp, ddd)| nb == *ppp && *ddd == *dir)
                                && counted.insert((nb, *dir))
                            {
                                todo.push(nb);
                            }
                        }
                    }
                }
            }
            g.fill(p, ' ');
            for (p, _dir) in &perimeter {
                g.set_value(*p, '*');
            }
            #[cfg(feature = "vis")]
            gd.draw(&g);
            cost += area * sides;
        }
    }
    cost
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
        assert_eq!(part1(&parse(&example())), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 1206);
    }
}
