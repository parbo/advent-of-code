use std::{iter::*, collections::VecDeque};

type ParsedItem = i64;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(f: &[ParsedItem]) -> Answer {
    let mut fish = f.to_owned();
    for _i in 0..80 {
	let mut n = vec![];
	for f in &mut fish {
	    match f {
		0 => {
		    *f = 6;
		    n.push(8);
		},
		_ => *f -= 1,
	    }
	}
	fish.append(&mut n);
    }
    fish.len() as Answer
}

fn part2(f: &[ParsedItem]) -> Answer {
    let mut fish: VecDeque<i64> = VecDeque::from([0,0,0,0,0,0,0,0,0]);
    for ff in f {
	fish[*ff as usize] += 1;
    }
    for i in 0..256 {
	let born = fish.pop_front().unwrap();
	fish[6] += born;
	fish.push_back(born);
    }
    fish.iter().sum::<i64>() as Answer
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_ch(&lines[0], ',').iter().map(|x| x.parse().unwrap()).collect()
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
    fn test_part1() {
        assert_eq!(part1(&vec![3,4,3,1,2]), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![3,4,3,1,2]), 26984457539);
    }
}
