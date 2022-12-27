use std::iter::*;

type ParsedItem = aoc::Vec3;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(dirs: &Parsed) -> Answer {
    let mut pos = [0, 0, 0];
    for d in dirs {
        pos = aoc::vec_add(pos, *d);
    }
    aoc::manhattan_hex_cube(pos, [0, 0, 0])
}

fn part2(dirs: &Parsed) -> Answer {
    let mut max = 0;
    let mut pos = [0, 0, 0];
    for d in dirs {
        pos = aoc::vec_add(pos, *d);
        let dist = aoc::manhattan_hex_cube(pos, [0, 0, 0]);
        max = max.max(dist);
    }
    max
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_ch(&lines[0], ',')
        .iter()
        .map(|x| *aoc::HEX_ALT_DIRECTION_MAP.get(*x).unwrap())
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&["ne,ne,ne".into()])), 3);
        assert_eq!(part1(&parse(&["ne,ne,sw,sw".into()])), 0);
        assert_eq!(part1(&parse(&["ne,ne,s,s".into()])), 2);
        assert_eq!(part1(&parse(&["se,sw,se,sw,sw".into()])), 3);
    }
}
