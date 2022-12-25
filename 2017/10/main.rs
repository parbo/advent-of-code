use std::iter::*;

type ParsedItem = u8;
type Parsed = Vec<ParsedItem>;
type Answer = usize;

fn reverse(data: &mut [u8], pos: usize, length: usize) {
    let max = data.len();
    for i in 0..(length / 2) {
        let a = (pos + i) % max;
        let b = (pos + length - 1 - i) % max;
        data.swap(a, b);
    }
}

fn solve(lengths: &[u8], max: u8, rounds: usize) -> Vec<u8> {
    let mut data: Vec<u8> = (0..=max).collect();
    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..rounds {
        for l in lengths {
            // Reverse the length
            let lu = *l as usize;
            reverse(&mut data, pos, lu);
            pos = (pos + lu + skip) % data.len();
            skip += 1;
        }
    }
    data
}

fn part1(data: &[ParsedItem]) -> Answer {
    let res = solve(data, 255, 1);
    (res[0] as usize) * (res[1] as usize)
}

fn part2(data: &[ParsedItem]) -> String {
    let mut data = data.to_vec();
    data.extend(vec![17, 31, 73, 47, 23]);
    let h = solve(&data, 255, 64);
    let mut dense = vec![];
    let mut ix = 0;
    while ix < h.len() {
        let v = h[ix]
            ^ h[ix + 1]
            ^ h[ix + 2]
            ^ h[ix + 3]
            ^ h[ix + 4]
            ^ h[ix + 5]
            ^ h[ix + 6]
            ^ h[ix + 7]
            ^ h[ix + 8]
            ^ h[ix + 9]
            ^ h[ix + 10]
            ^ h[ix + 11]
            ^ h[ix + 12]
            ^ h[ix + 13]
            ^ h[ix + 14]
            ^ h[ix + 15];
        ix += 16;
        dense.push(v);
    }

    aoc::to_hex(&dense)
}

fn parse(lines: &[String]) -> Parsed {
    aoc::split_ch(&lines[0], ',')
        .iter()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse2(lines: &[String]) -> Parsed {
    lines[0].as_bytes().to_vec()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    if part == 1 {
        let parsed = parse(&lines);
        let result = part1(&parsed);
        println!("{}", result);
    } else {
        let parsed = parse2(&lines);
        let result = part2(&parsed);
        println!("{}", result);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let mut data = vec![0, 1, 2, 3, 4, 5, 6];
        reverse(&mut data, 0, 3);
        assert_eq!(data, vec![2, 1, 0, 3, 4, 5, 6]);

        let mut data = vec![0, 1, 2, 3, 4, 5, 6];
        reverse(&mut data, 1, 3);
        assert_eq!(data, vec![0, 3, 2, 1, 4, 5, 6]);

        let mut data = vec![0, 1, 2, 3, 4, 5, 6];
        reverse(&mut data, 4, 3);
        assert_eq!(data, vec![0, 1, 2, 3, 6, 5, 4]);

        let mut data = vec![0, 1, 2, 3, 4, 5, 6];
        reverse(&mut data, 5, 3);
        assert_eq!(data, vec![5, 1, 2, 3, 4, 0, 6]);

        let mut data = vec![0, 1, 2, 3, 4, 5, 6];
        reverse(&mut data, 6, 3);
        assert_eq!(data, vec![0, 6, 2, 3, 4, 5, 1]);

        let mut data = vec![0, 1, 2, 3, 4, 5, 6];
        reverse(&mut data, 8, 3);
        assert_eq!(data, vec![0, 3, 2, 1, 4, 5, 6]);

        let mut data = vec![0, 1, 2, 3, 4, 5, 6];
        reverse(&mut data, 6, 5);
        assert_eq!(data, vec![2, 1, 0, 6, 4, 5, 3]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&[3, 4, 1, 5], 4, 1), vec![3, 4, 2, 1, 0]);
    }
}
