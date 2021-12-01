use std::iter::*;

type Parsed = Vec<i64>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    let mut num = 0;
    let mut last_v = None;
    for v in data {
        if let Some(lv) = last_v {
            if v > lv {
                num += 1;
            }
        }
        last_v = Some(v);
    }
    num
}

fn part2(data: &Parsed) -> Answer {
    let mut sliding = vec![];
    for i in 0..(data.len() - 2) {
        sliding.push(data[i] + data[i + 1] + data[i + 2])
    }
    part1(&sliding)
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            5
        );
    }
}
