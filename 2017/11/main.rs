use std::iter::*;
use std::time::Instant;

type ParsedItem = aoc::Vec3;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(dirs: &[ParsedItem]) -> Answer {
    let mut pos = [0, 0, 0];
    for d in dirs {
        pos = aoc::vec_add(pos, *d);
    }
    aoc::manhattan_hex_cube(pos, [0, 0, 0])
}

fn part2(dirs: &[ParsedItem]) -> Answer {
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
    let start_time = Instant::now();
    let (part, lines) = aoc::read_lines();
    let io_time = Instant::now();
    let parsed = parse(&lines);
    let parse_time = Instant::now();
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    let done_time = Instant::now();
    println!(
        "read: {:?}, parse: {:?}, solve: {:?}\n",
        io_time.duration_since(start_time),
        parse_time.duration_since(io_time),
        done_time.duration_since(parse_time)
    );
    println!("{}", result);
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
