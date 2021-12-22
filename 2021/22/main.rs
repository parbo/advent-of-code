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

fn split(cbi: Cuboid, cbj: Cuboid) -> (Vec<Cuboid>, Vec<Cuboid>) {
    let minx = cbi.minx.min(cbj.minx);
    let midx1 = cbi.minx.max(cbj.minx).min(cbi.maxx.min(cbj.maxx));
    let midx2 = cbi.maxx.min(cbj.maxx).max(cbi.minx.max(cbj.minx));
    let maxx = cbi.maxx.max(cbj.maxx);
    let miny = cbi.miny.min(cbj.miny);
    let midy1 = cbi.miny.max(cbj.miny).min(cbi.maxy.min(cbj.maxy));
    let midy2 = cbi.maxy.min(cbj.maxy).max(cbi.miny.max(cbj.miny));
    let maxy = cbi.maxy.max(cbj.maxy);
    let minz = cbi.minz.min(cbj.minz);
    let midz1 = cbi.minz.max(cbj.minz).min(cbi.maxz.min(cbj.maxz));
    let midz2 = cbi.maxz.min(cbj.maxz).max(cbi.minz.max(cbj.minz));
    let maxz = cbi.maxz.max(cbj.maxz);

    let xx = [(minx, midx1 - 1), (midx1, midx2 - 1), (midx2, maxx)];
    let yy = [(miny, midy1 - 1), (midy1, midy2 - 1), (midy2, maxy)];
    let zz = [(minz, midz1 - 1), (midz1, midz2 - 1), (midz2, maxz)];

    println!("xx: {:?}", xx);
    println!("yy: {:?}", yy);
    println!("zz: {:?}", zz);

    let mut fromi = vec![];
    let mut fromj = vec![];
    for xi in 0..3 {
        for yi in 0..3 {
            for zi in 0..3 {
                let (minx, maxx) = xx[xi];
                let (miny, maxy) = yy[yi];
                let (minz, maxz) = zz[zi];
                let ini = minx >= cbi.minx
                    && minx <= cbi.maxx
                    && miny >= cbi.miny
                    && miny <= cbi.maxy
                    && minz >= cbi.minz
                    && minz <= cbi.maxz;
                let inj = minx >= cbj.minx
                    && minx <= cbj.maxx
                    && miny >= cbj.miny
                    && miny <= cbj.maxy
                    && minz >= cbj.minz
                    && minz <= cbj.maxz;
                if ini && !inj {
                    let c = Cuboid {
                        state: cbi.state,
                        minx,
                        maxx,
                        miny,
                        maxy,
                        minz,
                        maxz,
                    };
                    fromi.push(c);
                } else if inj {
                    let c = Cuboid {
                        state: cbj.state,
                        minx,
                        maxx,
                        miny,
                        maxy,
                        minz,
                        maxz,
                    };
                    fromj.push(c);
                }
            }
        }
    }
    (fromi, fromj)
}

fn part2(cuboids: &[ParsedItem]) -> Answer {
    // Split to non-overlapping cuboids
    let mut cb = cuboids.to_owned();
    let mut ctr = 0;
    let mut start = 0;
    loop {
        // Find overlapping pairs, split first cuboid
        let mut replace = None;
        'outer: for i in start..(cb.len() - 1) {
            let cbi = cb[i];
            for j in (i + 1)..cb.len() {
                let cbj = cb[j];
                if (cbj.minx >= cbi.minx && cbj.minx <= cbi.maxx
                    || cbj.maxx >= cbi.minx && cbj.maxx <= cbi.maxx)
                    && (cbj.miny >= cbi.miny && cbj.miny <= cbi.maxy
                        || cbj.maxy >= cbi.miny && cbj.maxy <= cbi.maxy)
                    && (cbj.minz >= cbi.minz && cbj.minz <= cbi.maxz
                        || cbj.maxz >= cbi.minz && cbj.maxz <= cbi.maxz)
                {
		    println!("i: {}, cbi: {}", i, cbi);
		    println!("j: {}, cbj: {}", j, cbj);
                    // We have overlap. Split into new cuboids.
                    let (first, last) = split(cbi, cbj);
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

        if let Some(((i, first), (j, last))) = replace {
            println!(
                "overlap: {},{}, num cuboids: {}, adding: {}, {}",
                i,
                j,
                cb.len(),
                first.len(),
                last.len()
            );
            let mut new_cb = if i > 0 { cb[0..i].to_owned() } else { vec![] };
            new_cb.extend_from_slice(&first);
            if i + 1 < j {
                new_cb.extend_from_slice(&cb[(i + 1)..j]);
            }
            new_cb.extend_from_slice(&last);
            new_cb.extend_from_slice(&cb[(j + 1)..]);
            start = i;
	    ctr += 1;
	    if ctr > 10 {
		return 0;
	    }
            cb = new_cb;
        } else {
            break;
        }
    }
    // Now we have only non-overlapping cubes
    let mut num = 0;
    for c in cb {
        if c.state == State::On {
            num += (c.maxx + 1 - c.minx) * (c.maxy + 1 - c.miny) * (c.maxz + 1 - c.minz);
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
