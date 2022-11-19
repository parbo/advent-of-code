type Parsed = usize;
type Answer = usize;

fn part1(data: &Parsed) -> Answer {
    let mut buf = vec![0];
    let mut pos = 0;
    for v in 1..=2017 {
        pos = (pos + data) % buf.len() + 1;
        buf.insert(pos, v);
    }
    buf[(pos + 1) % buf.len()]
}

fn part2(data: &Parsed) -> Answer {
    let mut vv = 0;
    let mut pos = 0;
    for v in 1..=50000000 {
        pos = (pos + data) % v + 1;
        if pos == 1 {
            vv = v;
        }
    }
    vv
}

fn parse(lines: &[String]) -> Parsed {
    lines[0].parse().unwrap()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&3), 638);
    }
}
