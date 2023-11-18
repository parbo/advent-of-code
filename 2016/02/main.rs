use std::iter::*;

type ParsedItem = Vec<char>;
type Parsed = Vec<ParsedItem>;

fn pos_to_key(p: aoc::Point) -> char {
    match p {
        [0, 0] => '1',
        [1, 0] => '2',
        [2, 0] => '3',
        [0, 1] => '4',
        [1, 1] => '5',
        [2, 1] => '6',
        [0, 2] => '7',
        [1, 2] => '8',
        [2, 2] => '9',
        _ => panic!(),
    }
}

fn pos_to_key_2(p: aoc::Point) -> char {
    match p {
        [2, 0] => '1',
        [1, 1] => '2',
        [2, 1] => '3',
        [3, 1] => '4',
        [0, 2] => '5',
        [1, 2] => '6',
        [2, 2] => '7',
        [3, 2] => '8',
        [4, 2] => '9',
        [1, 3] => 'A',
        [2, 3] => 'B',
        [3, 3] => 'C',
        [2, 4] => 'D',
        _ => panic!(),
    }
}

fn part1(data: &Parsed) -> String {
    let mut pos = [1, 1];
    let mut code = vec![];
    for key in data {
        for m in key {
            let cs = m.to_string();
            let npos = aoc::point_add(pos, *aoc::DIRECTION_MAP.get(&cs as &str).unwrap());
            if npos[0] < 0 || npos[0] > 2 || npos[1] < 0 || npos[1] > 2 {
                continue;
            }
            pos = npos;
        }
        code.push(pos_to_key(pos));
    }
    code.iter().collect()
}

fn part2(data: &Parsed) -> String {
    let mut pos = [0, 2];
    let mut code = vec![];
    let banned = [
        [0, 0],
        [1, 0],
        [3, 0],
        [4, 0],
        [0, 1],
        [4, 1],
        [0, 3],
        [4, 3],
        [0, 4],
        [1, 4],
        [3, 4],
        [4, 4],
    ];
    for key in data {
        for m in key {
            let cs = m.to_string();
            let npos = aoc::point_add(pos, *aoc::DIRECTION_MAP.get(&cs as &str).unwrap());
            if npos[0] < 0 || npos[0] > 4 || npos[1] < 0 || npos[1] > 4 || banned.contains(&npos) {
                continue;
            }
            pos = npos;
        }
        code.push(pos_to_key_2(pos));
    }
    code.iter().collect()
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
