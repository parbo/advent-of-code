use std::iter::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Mapping {
    from: String,
    to: String,
    ranges: Vec<[i64; 3]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Almanac {
    seeds: Vec<i64>,
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn location(&self, seed: i64) -> i64 {
        let mut curr = seed;
        for mapping in &self.mappings {
            for &[dest, source, num] in &mapping.ranges {
                if source <= curr && curr < source + num {
                    let diff = curr - source;
                    curr = dest + diff;
                    break;
                }
            }
        }
        curr
    }

    fn location_ranges(&self, seeds: Vec<[i64; 2]>) -> Vec<[i64; 2]> {
        let mut curr = seeds;
        for mapping in &self.mappings {
            let mut next = vec![];
            for &[dest, source, num] in &mapping.ranges {
                for cc in &curr {
                    let overlap = [source.max(cc[0]), (source + num).min(cc[0] + cc[1])];
                    if overlap[0] < overlap[1] {
                        let diff = overlap[0] - source;
                        let nn = [dest + diff, overlap[1] - overlap[0]];
                        next.push(nn);
                    }
                }
            }
            curr = next;
        }
        curr
    }
}

type Parsed = Almanac;

fn part1(data: &Parsed) -> i64 {
    data.seeds.iter().map(|x| data.location(*x)).min().unwrap()
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
        let map_type = &group[0];
        let map_type = aoc::split_ch(&map_type[0..map_type.len() - 5], '-');
        let map_type = (map_type[0], map_type[2]);
        let ranges: Vec<Vec<i64>> = group[1..].iter().map(|x| aoc::things(x)).collect();
        let ranges = ranges.iter().map(|x| [x[0], x[1], x[2]]).collect();
        let m = Mapping {
            from: map_type.0.to_string(),
            to: map_type.1.to_string(),
            ranges,
        };
        mappings.push(m)
    }
    Almanac { seeds, mappings }
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
