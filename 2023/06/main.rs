use std::iter::*;

type Parsed = Vec<String>;

fn ways_to_win(time: i64, distance: i64) -> i64 {
    (0..=time).filter(|t| (time - t) * t > distance).count() as i64
}

fn part1(data: &Parsed) -> i64 {
    let times = aoc::things::<i64>(&data[0]);
    let distances = aoc::things::<i64>(&data[1]);
    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| ways_to_win(time, distance))
        .product()
}

fn part2(data: &Parsed) -> i64 {
    let time = data[0][10..].replace(' ', "").parse::<i64>().unwrap();
    let distance = data[1][10..].replace(' ', "").parse::<i64>().unwrap();
    ways_to_win(time, distance)
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 71503);
    }
}
