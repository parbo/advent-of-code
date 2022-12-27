use std::iter::*;

type Parsed = Vec<Vec<u32>>;

fn part1(layers: &Parsed) -> usize {
    let min_zeroes = layers
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            a.iter()
                .filter(|&x| *x == 0)
                .count()
                .cmp(&b.iter().filter(|&x| *x == 0).count())
        })
        .map(|(index, _)| index)
        .unwrap();
    let ans = layers[min_zeroes].iter().filter(|&x| *x == 1).count()
        * layers[min_zeroes].iter().filter(|&x| *x == 2).count();
    ans
}

fn part2(layers: &Parsed) -> usize {
    for y in 0..6 {
        for x in 0..25 {
            let mut pixel = 2u32;
            for layer in layers.iter().rev() {
                let v = layer[y * 25 + x];
                match v {
                    0 => pixel = 0,
                    1 => pixel = 1,
                    2 => {}
                    _ => panic!("OH NOES"),
                }
            }
            match pixel {
                0 => print!(" "),
                1 => print!("█"),
                _ => panic!("OH NOES"),
            }
        }
        println!();
    }
    0
}

fn parse(lines: &[String]) -> Parsed {
    let mut layers = vec![];
    let mut iter = lines[0].chars().map(|x| x.to_digit(10).unwrap());
    loop {
        let mut layer = vec![];
        for _ in 0..25 {
            for _ in 0..6 {
                if let Some(x) = iter.next() {
                    layer.push(x)
                } else {
                    return layers;
                }
            }
        }
        layers.push(layer);
    }
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::{part1, part2};

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&vec![0]), 0);
    // }
}
