use aoc::Grid;
use std::collections::HashMap;
use std::iter::*;
use std::time::Instant;

type Parsed = String;

// From day 10
fn knot_hash(data: &[u8]) -> Vec<u8> {
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
    dense
}

type Answer = i64;

fn part1(key: &String) -> Answer {
    let mut used = 0;
    for i in 0..128 {
        let hash = knot_hash(format!("{}-{}", key, i).as_bytes());
        for h in hash {
            used += h.count_ones();
        }
    }
    used as i64
}

fn part2(key: &String) -> Answer {
    let mut grid = HashMap::new();
    for row in 0..128 {
        let hash = knot_hash(format!("{}-{}", key, row).as_bytes());
        for (grp, h) in hash.iter().enumerate() {
            for i in 0..8 {
                if (h & (1 << (7 - i))) != 0 {
                    grid.insert([(grp * 8 + i) as i64, row as i64], '#');
                }
            }
        }
    }
    let mut graph = aoc::UnGraphMap::new();
    for p in grid.points() {
        if grid.get_value(p).is_some() {
            graph.add_node(p);
            for np in aoc::neighbors(p) {
                if grid.get_value(np).is_some() {
                    graph.add_node(np);
                    graph.add_edge(p, np, 1);
                }
            }
        }
    }
    aoc::algo::kosaraju_scc(&graph).len() as i64
}

fn parse(lines: &[String]) -> String {
    lines[0].clone()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&"flqrgnkx".to_string()), 8108);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&"flqrgnkx".to_string()), 1242);
    }
}
