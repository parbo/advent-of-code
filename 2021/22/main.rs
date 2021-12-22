use aoc::FxHashMap;
use std::iter::*;
use std::time::Instant;

#[derive(
    parse_display::Display, parse_display::FromStr, Debug, Copy, Clone, PartialEq, Eq, Hash,
)]
#[display(style = "lowercase")]
enum State {
    On,
    Off,
}

#[derive(
    parse_display::Display, parse_display::FromStr, Debug, Copy, Clone, PartialEq, Eq, Hash,
)]
#[display("{state} x={minx}..{maxx},y={miny}..{maxy},z={minz}..{maxz}")]
struct Cuboid {
    state: State,
    minx: i64,
    maxx: i64,
    miny: i64,
    maxy: i64,
    minz: i64,
    maxz: i64,
}

impl Cuboid {
    fn contains(&self, other: &Cuboid) -> bool {
        other.minx >= self.minx
            && other.maxx <= self.maxx
            && other.miny >= self.miny
            && other.maxy <= self.maxy
            && other.minz >= self.minz
            && other.maxz <= self.maxz
    }

    fn overlaps(&self, other: &Cuboid) -> bool {
        other.minx.max(self.minx) < (other.maxx + 1).min(self.maxx + 1)
            && other.miny.max(self.miny) < (other.maxy + 1).min(self.maxy + 1)
            && other.minz.max(self.minz) < (other.maxz + 1).min(self.maxz + 1)
    }

    fn is_empty(&self) -> bool {
        self.maxx < self.minx || self.maxy < self.miny || self.maxz < self.minz
    }
}

type ParsedItem = Cuboid;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(cuboids: &[ParsedItem]) -> Answer {
    let mut cubes = FxHashMap::default();
    for c in cuboids {
        if c.minx < -50 || c.maxx > 50 || c.miny < -50 || c.maxy > 50 || c.minz < -50 || c.maxz > 50
        {
            continue;
        }
        let v = if let State::On = c.state { true } else { false };
        for x in c.minx..=c.maxx {
            for y in c.miny..=c.maxy {
                for z in c.minz..=c.maxz {
                    *cubes.entry([x, y, z]).or_insert(false) = v;
                }
            }
        }
    }
    cubes.values().filter(|x| **x).count() as Answer
}

fn merge(cbs: &[Cuboid]) -> Vec<Cuboid> {
    let mut ncb = cbs.to_owned();
    if ncb.is_empty() {
        return ncb;
    }
    let mut merged_count = 0;
    loop {
        let mut new_cb = vec![];
        let mut any_merged = false;
        let mut i = 0;
        loop {
            let j = i + 1;
            if j == ncb.len() {
                new_cb.push(ncb[i]);
                i += 1;
            } else if ncb[i].state == ncb[j].state {
                let mut merged = ncb[i];
                // Merge if only different in one dimension and not disjoint
                let samex = ncb[i].minx == ncb[j].minx && ncb[i].maxx == ncb[j].maxx;
                let samey = ncb[i].miny == ncb[j].miny && ncb[i].maxy == ncb[j].maxy;
                let samez = ncb[i].minz == ncb[j].minz && ncb[i].maxz == ncb[j].maxz;
                let overlapx =
                    ncb[i].minx.max(ncb[j].minx) <= (ncb[i].maxx + 1).min(ncb[j].maxx + 1);
                let overlapy =
                    ncb[i].miny.max(ncb[j].miny) <= (ncb[i].maxy + 1).min(ncb[j].maxy + 1);
                let overlapz =
                    ncb[i].minz.max(ncb[j].minz) <= (ncb[i].maxz + 1).min(ncb[j].maxz + 1);
                if !samex && samey && samez && overlapx {
                    merged.minx = ncb[i].minx.min(ncb[j].minx);
                    merged.maxx = ncb[i].maxx.max(ncb[j].maxx);
                    // println!("merged: {}, {} -> {}", ncb[i], ncb[j], merged);
                    new_cb.push(merged);
                    any_merged = true;
                    merged_count += 1;
                } else if samex && !samey && samez && overlapy {
                    merged.miny = ncb[i].miny.min(ncb[j].miny);
                    merged.maxy = ncb[i].maxy.max(ncb[j].maxy);
                    // println!("merged: {}, {} -> {}", ncb[i], ncb[j], merged);
                    new_cb.push(merged);
                    merged_count += 1;
                    any_merged = true;
                } else if samex && samey && !samez && overlapz {
                    merged.minz = ncb[i].minz.min(ncb[j].minz);
                    merged.maxz = ncb[i].maxz.max(ncb[j].maxz);
                    // println!("merged: {}, {} -> {}", ncb[i], ncb[j], merged);
                    new_cb.push(merged);
                    merged_count += 1;
                    any_merged = true;
                } else {
                    new_cb.push(ncb[i]);
                    new_cb.push(ncb[j]);
                }
                i += 2
            } else {
                i += 1
            }
            if i >= ncb.len() {
                break;
            }
        }
        ncb = new_cb;
        if !any_merged {
            break;
        }
    }
    if merged_count > 0 {
        println!("merged: {:?}, {:?}", cbs.len(), ncb.len());
    }
    ncb
}

fn split(cbi: &Cuboid, cbj: &Cuboid) -> (Vec<Cuboid>, Vec<Cuboid>) {
    // If fully contained, return later
    if cbj.contains(&cbi) {
        return (vec![], vec![*cbj]);
    }
    // Use exclusive coords here
    let nminx = cbi.minx.min(cbj.minx);
    let mut nmidx1 = cbi.minx.max(cbj.minx);
    let mut nmidx2 = (cbi.maxx + 1).min(cbj.maxx + 1);
    let nmaxx = (cbi.maxx + 1).max(cbj.maxx + 1);

    let nminy = cbi.miny.min(cbj.miny);
    let mut nmidy1 = cbi.miny.max(cbj.miny);
    let mut nmidy2 = (cbi.maxy + 1).min(cbj.maxy + 1);
    let nmaxy = (cbi.maxy - 1).max(cbj.maxy - 1);

    let nminz = cbi.minz.min(cbj.minz);
    let mut nmidz1 = cbi.minz.max(cbj.minz);
    let mut nmidz2 = (cbi.maxz + 1).min(cbj.maxz + 1);
    let nmaxz = (cbi.maxz + 1).max(cbj.maxz + 1);

    // And back to inclusive
    let xx = [
        (nminx, nmidx1 - 1),
        (nmidx1, nmidx2 - 1),
        (nmidx2, nmaxx - 1),
    ];
    let yy = [
        (nminy, nmidy1 - 1),
        (nmidy1, nmidy2 - 1),
        (nmidy2, nmaxy - 1),
    ];
    let zz = [
        (nminz, nmidz1 - 1),
        (nmidz1, nmidz2 - 1),
        (nmidz2, nmaxz - 1),
    ];

    // Order needs to be maintained
    let mut fromi = vec![];
    let mut fromj = vec![];
    for xi in 0..3 {
        for yi in 0..3 {
            for zi in 0..3 {
                let (minx, maxx) = xx[xi];
                let (miny, maxy) = yy[yi];
                let (minz, maxz) = zz[zi];
                let mut c = Cuboid {
                    state: State::Off,
                    minx,
                    maxx,
                    miny,
                    maxy,
                    minz,
                    maxz,
                };
                if c.is_empty() {
                    continue;
                }
                let ini = cbi.contains(&c);
                let inj = cbj.contains(&c);
                if ini && !inj {
                    c.state = cbi.state;
                    fromi.push(c);
                } else if inj {
                    c.state = cbj.state;
                    fromj.push(c);
                }
            }
        }
    }
    (merge(&fromi), merge(&fromj))
}

fn part2(cuboids: &[ParsedItem]) -> Answer {
    // Split to non-overlapping cuboids
    let mut cb = cuboids.to_owned();
    let mut ctr = 0;
    let mut start = 0;
    loop {
        println!("1");
        // Find overlapping pairs, split first cuboid
        let mut replace = None;
        'outer: for i in 0..(cb.len() - 1) {
            let cbi = cb[i];
            for j in (i + 1)..cb.len() {
                let cbj = cb[j];
                if cbi.overlaps(&cbj) {
                    println!("i: {}, cbi: {}", i, cbi);
                    println!("j: {}, cbj: {}", j, cbj);
                    // We have overlap. Split into new cuboids.
                    println!("pslit");
                    let (first, last) = split(&cbi, &cbj);
                    for a in &first {
                        println!("1: {}", a);
                    }
                    for a in &last {
                        println!("2: {}", a);
                    }
                    replace = Some(((i, first), (j, last)));
                    break 'outer;
                }
            }
        }

        println!("2");
        if let Some(((i, first), (j, last))) = replace {
            ctr += 1;
            //	    if ctr % 1000 == 0 {
            println!(
                "overlap: {},{}, num cuboids: {}, adding: {}, {}, {}",
                i,
                j,
                cb.len(),
                first.len(),
                last.len(),
                ctr
            );
            //	    }
            let new_j = j - 1 + first.len();
            cb.splice(i..(i + 1), first);
            cb.splice(new_j..(new_j + 1), last);
        } else {
            println!("3");
            break;
        }
        println!("4");
    }
    // Merge
    println!("merging cb: {}", cb.len());
    loop {
        let lb = cb.len();
        cb = merge(&cb);
        if lb == cb.len() {
            break;
        }
    }
    // Now we have only non-overlapping cubes
    let mut num: i64 = 0;
    for c in cb {
        let area = (c.maxx + 1 - c.minx) * (c.maxy + 1 - c.miny) * (c.maxz + 1 - c.minz);
        println!("{}, area: {}", c, area);
        if c.state == State::On {
            num += area;
        } else {
            // Can't go to negative reactors on
            num -= area.min(num);
        }
    }
    num
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
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

    fn example() -> Vec<String> {
        include_str!("sample.txt")
            .lines()
            .map(|x| x.into())
            .collect()
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 2758514936282235);
    }
}
