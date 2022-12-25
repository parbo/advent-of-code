use std::{collections::HashMap, time::Instant};

type Parsed = i64;
type Answer = i64;

fn part1(num: &Parsed) -> Answer {
    if *num == 1 {
        return 0;
    }
    let mut val = 2;
    let mut ring_size = 8;
    let mut pos: (i64, i64) = (1, 0);
    loop {
        if val + ring_size > *num {
            break;
        }
        val += ring_size;
        ring_size += 8;
        pos.0 += 1;
        pos.1 += 1;
    }
    let steps = ring_size / 4;
    'outer: for dir in [
        (0, -1, steps - 1),
        (-1, 0, steps),
        (0, 1, steps),
        (1, 0, steps),
    ] {
        for _ in 0..dir.2 {
            if val == *num {
                break 'outer;
            }
            val += 1;
            pos.0 += dir.0;
            pos.1 += dir.1;
        }
    }
    pos.0.abs() + pos.1.abs()
}

fn part2(num: &Parsed) -> Answer {
    let mut ring_size = 8;
    let mut p: (i64, i64) = (1, 0);
    let mut vals: HashMap<(i64, i64), i64> = HashMap::new();
    vals.insert((0, 0), 1);
    let res;
    'outer: loop {
        let steps = ring_size / 4;
        for dir in [
            (0, -1, steps - 1),
            (-1, 0, steps),
            (0, 1, steps),
            (1, 0, steps + 1),
        ] {
            for _ in 0..dir.2 {
                let adj_sum: i64 = [
                    (1, 0),
                    (1, -1),
                    (0, -1),
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ]
                .iter()
                .filter_map(|x| vals.get(&(p.0 + x.0, p.1 + x.1)))
                .sum();
                if adj_sum > *num {
                    res = adj_sum;
                    break 'outer;
                }
                vals.insert(p, adj_sum);
                p.0 += dir.0;
                p.1 += dir.1;
            }
        }
        ring_size += 8;
    }
    res
}

fn parse(lines: &[String]) -> Parsed {
    lines[0].parse().unwrap()
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
        assert_eq!(part1(&1), 0);
        assert_eq!(part1(&12), 3);
        assert_eq!(part1(&23), 2);
        assert_eq!(part1(&1024), 31);
    }
}
