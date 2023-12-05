use std::iter::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Mapping(Vec<[i64; 3]>);

impl Mapping {
    fn map(&self, cc: [i64; 2]) -> Vec<[i64; 2]> {
        let mut nxt: Vec<_> = self
            .0
            .iter()
            .filter_map(|&[dest, source, num]| {
                let overlap = [source.max(cc[0]), (source + num).min(cc[0] + cc[1])];
                if overlap[0] < overlap[1] {
                    let diff = overlap[0] - source;
                    Some([dest + diff, overlap[1] - overlap[0]])
                } else {
                    None
                }
            })
            .collect();
        if nxt.is_empty() {
            nxt.push(cc);
        }
        nxt
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Almanac {
    seeds: Vec<i64>,
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn location_ranges(&self, seeds: Vec<[i64; 2]>) -> Vec<[i64; 2]> {
        let mut curr = seeds;
        for mapping in &self.mappings {
            curr = curr.iter().flat_map(|cc| mapping.map(*cc)).collect();
        }
        curr
    }
}

type Parsed = Almanac;

fn part1(data: &Parsed) -> i64 {
    let seeds = data.seeds.iter().map(|s| [*s, 1]).collect();
    let ranges = data.location_ranges(seeds);
    ranges.iter().map(|x| x[0]).min().unwrap()
}

fn part2(data: &Parsed) -> i64 {
    let seeds: Vec<[i64; 2]> = data.seeds.chunks(2).map(|x| [x[0], x[1]]).collect();
    let ranges = data.location_ranges(seeds);
    ranges.iter().map(|x| x[0]).min().unwrap()
}

fn parse(lines: &[String]) -> Parsed {
    let groups = aoc::split_by_empty_line(lines);
    let seeds: Vec<i64> = aoc::things(&groups[0][0][7..]);
    let mut mappings = vec![];
    for group in &groups[1..] {
        let ranges: Vec<Vec<i64>> = group[1..].iter().map(|x| aoc::things(x)).collect();
        let ranges = ranges.iter().map(|x| [x[0], x[1], x[2]]).collect();
        mappings.push(Mapping(ranges))
    }
    Almanac { seeds, mappings }
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
