use aoc::{Grid, GridDrawer};
use std::{collections::VecDeque, iter::*};

type ParsedItem = Vec<char>;
type Parsed = Vec<ParsedItem>;

fn search(g: &Parsed, start: aoc::Point) -> i64 {
    let mut todo = VecDeque::from([(start, '0')]);
    let mut found = aoc::FxHashSet::default();
    let mut seen = aoc::FxHashSet::default();
    while let Some((p, v)) = todo.pop_back() {
        if v == '9' {
            found.insert(p);
        }
        if seen.contains(&p) {
            continue;
        }
        seen.insert(p);
        for nb in aoc::neighbors(p) {
            if let Some(value) = g.get_value(nb) {
                if value as i8 - v as i8 == 1 {
                    todo.push_front((nb, value));
                }
            }
        }
    }
    found.len() as i64
}

fn search_paths<F: Fn(char) -> Vec<[u8; 3]>>(
    g: &Parsed,
    start: aoc::Point,
    draw: &mut Option<aoc::BitmapSpriteGridDrawer<F, Parsed, char>>,
) -> i64 {
    let mut todo = VecDeque::from([(vec![start], '0')]);
    let mut found = aoc::FxHashSet::default();
    let mut seen = aoc::FxHashSet::default();
    let mut all_p = vec![];
    while let Some((p, v)) = todo.pop_back() {
        if v == '9' {
            found.insert(p.clone());
        }
        if seen.contains(&p) {
            continue;
        }
        seen.insert(p.clone());
        for nb in aoc::neighbors(*p.last().unwrap()) {
            if let Some(value) = g.get_value(nb) {
                if value as i8 - v as i8 == 1 {
                    let mut newp = p.clone();
                    if draw.is_some() {
                        all_p.push(nb);
                    }
                    newp.push(nb);
                    todo.push_front((newp, value));
                }
            }
        }
    }
    if let Some(gd) = draw {
        let ext = g.extents();
        gd.set_rect(ext);
        let mut gg = g.clone();
        for p in all_p {
            gg.set_value(p, '#');
            gd.draw(&gg);
            gd.save_image();
        }
    }
    found.len() as i64
}

fn part1(data: &Parsed) -> i64 {
    data.points()
        .filter(|p| data.get_value(*p) == Some('0'))
        .map(|p| search(data, p))
        .sum()
}

fn part2(data: &Parsed) -> i64 {
    let mut draw = if cfg!(feature = "vis") {
        let gd = aoc::BitmapSpriteGridDrawer::new(
            (2, 2),
            |c| {
                if c == '#' {
                    let v = [0xff, 0, 0];
                    vec![v, v, v, v]
                } else {
                    let v = c as i32 - '0' as i32;
                    let v = [0, (v * 255 / 9) as u8, 0];
                    vec![v, v, v, v]
                }
            },
            "vis/day10",
        );
        Some(gd)
    } else {
        None
    };
    data.points()
        .filter(|p| data.get_value(*p) == Some('0'))
        .map(|p| search_paths(data, p, &mut draw))
        .sum()
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

    fn example1() -> Vec<String> {
        vec!["0123".into(), "1234".into(), "8765".into(), "9876".into()]
    }

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&parse(&example1())), 1);
    }
}
