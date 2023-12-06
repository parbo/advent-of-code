use std::iter::*;

// #[derive(parse_display::Display, parse_display::FromStr, Debug, Clone, PartialEq, Eq, Hash)]
// #[display("{thing}: {al}-{ah} or {bl}-{bh}")]
// struct Rule {
//     thing: String,
//     al: i64,
//     ah: i64,
//     bl: i64,
//     bh: i64,
// }

type Parsed = Vec<String>;

fn part1(data: &Parsed) -> i64 {
    let times = aoc::things::<i64>(&data[0]);
    let distances = aoc::things::<i64>(&data[1]);
    let mut margin = vec![];
    for (time, distance) in times.into_iter().zip(distances) {
        let mut n = 0;
        for t in 0..=time {
            let time_left = time - t;
            let d = time_left * t;
            if d > distance {
                n += 1;
            }
        }
        margin.push(n);
    }
    margin.iter().product()
}

fn part2(data: &Parsed) -> i64 {
    let time = data[0]
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<i64>()
        .unwrap();
    let distance = data[1]
        .split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse::<i64>()
        .unwrap();
    let mut n = 0;
    for t in 0..=time {
        let time_left = time - t;
        let d = time_left * t;
        if d > distance {
            n += 1;
        }
    }
    n
}

fn parse(lines: &[String]) -> Parsed {
    lines.to_vec()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "Time:      7  15   30".into(),
            "Distance:  9  40  200".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 288);
    }
}
