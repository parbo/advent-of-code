use aoc::Grid;
use std::iter::*;

type ParsedItem = Vec<char>;
type Parsed = Vec<ParsedItem>;

fn part1(data: &Parsed) -> i64 {
    let mut g = data.clone();
    let w = ['X', 'M', 'A', 'S'];
    let mut num = 0;
    for p in g.points() {
        // check in all dirs from here
        let last_num = num;
        'outer: for d in aoc::DIRECTIONS_INCL_DIAGONALS {
            let mut pp = p;
            for x in w {
                if let Some(a) = g.get_value(pp) {
                    if a != x {
                        continue 'outer;
                    }
                } else {
                    continue 'outer;
                }
                pp = aoc::point_add(pp, d);
            }
            num += 1;
        }
        if last_num != num {
            // Don't include this point in any other XMAS instances
            g.set_value(p, '.');
        }
    }
    num
}

fn part2(data: &Parsed) -> i64 {
    let wds = [
        ['M', 'S', 'A', 'M', 'S'],
        ['S', 'S', 'A', 'M', 'M'],
        ['M', 'M', 'A', 'S', 'S'],
        ['S', 'M', 'A', 'S', 'M'],
    ];
    let pat = [[0, 0], [2, 0], [1, 1], [0, 2], [2, 2]];
    let mut num = 0;
    for p in data.points() {
        'outer: for w in wds {
            for (x, d) in zip(w, pat) {
                let pp = aoc::point_add(p, d);
                if let Some(a) = data.get_value(pp) {
                    if a != x {
                        continue 'outer;
                    }
                } else {
                    continue 'outer;
                }
            }
            num += 1;
        }
    }
    num
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
        vec![
            "MMMSXXMASM".into(),
            "MSAMXMSMSA".into(),
            "AMXSXMAAMM".into(),
            "MSAMASMSMX".into(),
            "XMASAMXAMM".into(),
            "XXAMMXXAMA".into(),
            "SMSMSASXSS".into(),
            "SAXAMASAAA".into(),
            "MAMMMXMMMM".into(),
            "MXMXAXMASX".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 9);
    }
}
