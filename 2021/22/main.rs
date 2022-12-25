use aoc::FxHashMap;
use std::iter::*;

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

fn part1(cuboids: &Parsed) -> Answer {
    let mut cubes = FxHashMap::default();
    for c in cuboids {
        if c.minx < -50 || c.maxx > 50 || c.miny < -50 || c.maxy > 50 || c.minz < -50 || c.maxz > 50
        {
            continue;
        }
        let v = matches!(c.state, State::On);
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
                // Merge if exactly the same or only different in one dimension and not disjoint
                let samex = ncb[i].minx == ncb[j].minx && ncb[i].maxx == ncb[j].maxx;
                let samey = ncb[i].miny == ncb[j].miny && ncb[i].maxy == ncb[j].maxy;
                let samez = ncb[i].minz == ncb[j].minz && ncb[i].maxz == ncb[j].maxz;
                let overlapx =
                    ncb[i].minx.max(ncb[j].minx) <= (ncb[i].maxx + 1).min(ncb[j].maxx + 1);
                let overlapy =
                    ncb[i].miny.max(ncb[j].miny) <= (ncb[i].maxy + 1).min(ncb[j].maxy + 1);
                let overlapz =
                    ncb[i].minz.max(ncb[j].minz) <= (ncb[i].maxz + 1).min(ncb[j].maxz + 1);
                if samex && samey && samez {
                    new_cb.push(merged);
                    any_merged = true;
                } else if !samex && samey && samez && overlapx {
                    merged.minx = ncb[i].minx.min(ncb[j].minx);
                    merged.maxx = ncb[i].maxx.max(ncb[j].maxx);
                    // println!("merged: {}, {} -> {}", ncb[i], ncb[j], merged);
                    new_cb.push(merged);
                    any_merged = true;
                } else if samex && !samey && samez && overlapy {
                    merged.miny = ncb[i].miny.min(ncb[j].miny);
                    merged.maxy = ncb[i].maxy.max(ncb[j].maxy);
                    // println!("merged: {}, {} -> {}", ncb[i], ncb[j], merged);
                    new_cb.push(merged);
                    any_merged = true;
                } else if samex && samey && !samez && overlapz {
                    merged.minz = ncb[i].minz.min(ncb[j].minz);
                    merged.maxz = ncb[i].maxz.max(ncb[j].maxz);
                    // println!("merged: {}, {} -> {}", ncb[i], ncb[j], merged);
                    new_cb.push(merged);
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
    // if merged_count > 0 {
    //     println!("merged: {:?}, {:?}", cbs.len(), ncb.len());
    // }
    ncb
}

fn split(cbi: &Cuboid, cbj: &Cuboid) -> Vec<Cuboid> {
    // If fully contained, return later
    if cbj.contains(cbi) {
        return vec![];
    }
    // Use exclusive coords here
    let nminx = cbi.minx.min(cbj.minx);
    let nmidx1 = cbi.minx.max(cbj.minx);
    let nmidx2 = (cbi.maxx + 1).min(cbj.maxx + 1);
    let nmaxx = (cbi.maxx + 1).max(cbj.maxx + 1);

    let nminy = cbi.miny.min(cbj.miny);
    let nmidy1 = cbi.miny.max(cbj.miny);
    let nmidy2 = (cbi.maxy + 1).min(cbj.maxy + 1);
    let nmaxy = (cbi.maxy + 1).max(cbj.maxy + 1);

    let nminz = cbi.minz.min(cbj.minz);
    let nmidz1 = cbi.minz.max(cbj.minz);
    let nmidz2 = (cbi.maxz + 1).min(cbj.maxz + 1);
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

    let mut fromi = vec![];
    for xxx in xx {
        for yyy in yy {
            for zzz in zz {
                let (minx, maxx) = xxx;
                let (miny, maxy) = yyy;
                let (minz, maxz) = zzz;
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
                }
            }
        }
    }
    merge(&fromi)
}

#[cfg(test)]
fn draw(_: &[Cuboid], _: &[Cuboid]) {}

#[cfg(not(test))]
fn draw(cuboids_a: &[Cuboid], cuboids_b: &[Cuboid]) {
    let mut window = kiss3d::window::Window::new_with_size("Day 22", 800, 1000);
    window.set_light(kiss3d::light::Light::StickToCamera);
    let eye = kiss3d::nalgebra::Point3::new(40.0f32, 0.0, 80.0);
    let at = kiss3d::nalgebra::Point3::origin();
    let mut camera = kiss3d::camera::ArcBall::new(eye, at);
    println!("add cubes");
    let mut ix = 0;
    for cb in cuboids_a {
        println!("a: {}", cb);
        let cminx = cb.minx as f32 / 10000.0;
        let cmaxx = (cb.maxx + 1) as f32 / 10000.0;
        let cminy = cb.miny as f32 / 10000.0;
        let cmaxy = (cb.maxy + 1) as f32 / 10000.0;
        let cminz = cb.minz as f32 / 10000.0;
        let cmaxz = (cb.maxz + 1) as f32 / 10000.0;
        let mut c = window.add_cube(cmaxx - cminx, cmaxy - cminy, cmaxz - cminz);
        c.append_translation(&kiss3d::nalgebra::Translation3::new(
            cminx + (cmaxx - cminx) / 2.0,
            cminy + (cmaxy - cminy) / 2.0 - 15.0,
            cminz + (cmaxz - cminz) / 2.0,
        ));
        let colors1 = [
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
        ];
        let colors2 = [
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 0.0],
        ];
        if cb.state == State::On {
            c.set_color(
                colors1[ix % colors1.len()][0],
                colors1[ix % colors1.len()][1],
                colors1[ix % colors1.len()][2],
            );
        } else {
            c.set_color(
                colors2[ix % colors2.len()][0],
                colors2[ix % colors2.len()][1],
                colors2[ix % colors2.len()][2],
            );
        }
    }
    ix = 0;
    for cb in cuboids_b {
        println!("b: {}", cb);
        let cminx = cb.minx as f32 / 10000.0;
        let cmaxx = (cb.maxx + 1) as f32 / 10000.0;
        let cminy = cb.miny as f32 / 10000.0;
        let cmaxy = (cb.maxy + 1) as f32 / 10000.0;
        let cminz = cb.minz as f32 / 10000.0;
        let cmaxz = (cb.maxz + 1) as f32 / 10000.0;
        let mut c = window.add_cube(cmaxx - cminx, cmaxy - cminy, cmaxz - cminz);
        c.append_translation(&kiss3d::nalgebra::Translation3::new(
            cminx + (cmaxx - cminx) / 2.0,
            cminy + (cmaxy - cminy) / 2.0 + 15.0,
            cminz + (cmaxz - cminz) / 2.0,
        ));
        let colors1 = [
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
        ];
        let colors2 = [
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 0.0],
        ];
        if cb.state == State::On {
            c.set_color(
                colors1[ix % colors1.len()][0],
                colors1[ix % colors1.len()][1],
                colors1[ix % colors1.len()][2],
            );
        } else {
            c.set_color(
                colors2[ix % colors2.len()][0],
                colors2[ix % colors2.len()][1],
                colors2[ix % colors2.len()][2],
            );
        }
        ix += 1;
    }
    println!("start rendering");
    while window.render_with_camera(&mut camera) {
        // rotate the arc-ball camera.
        let curr_yaw = camera.yaw();
        camera.set_yaw(curr_yaw + 0.05);
    }
}

#[allow(clippy::needless_range_loop)]
fn solve(cuboids: &[ParsedItem], d: bool) -> Answer {
    // Split to non-overlapping cuboids
    let mut cb = cuboids.to_owned();
    let mut start = 0;
    loop {
        // Find overlapping pairs, split first cuboid
        let mut replace = None;
        'outer: for i in start..(cb.len() - 1) {
            let cbi = cb[i];
            for j in (i + 1)..cb.len() {
                let cbj = cb[j];
                if cbi.overlaps(&cbj) {
                    let first = split(&cbi, &cbj);
                    replace = Some((i, first));
                    break 'outer;
                }
            }
        }

        if let Some((i, first)) = replace {
            start = i;
            cb.splice(i..(i + 1), first);
        } else {
            break;
        }
    }
    if d {
        draw(cuboids, &cb);
    }
    // Now we have only non-overlapping cubes
    cb.iter()
        .filter(|x| x.state == State::On)
        .map(|c| (c.maxx + 1 - c.minx) * (c.maxy + 1 - c.miny) * (c.maxz + 1 - c.minz))
        .sum::<i64>()
}

fn part2(cuboids: &Parsed) -> Answer {
    solve(cuboids, false)
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
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

    #[test]
    fn test_merge_1() {
        let c = [
            Cuboid {
                state: State::On,
                minx: -10,
                maxx: 8,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
            Cuboid {
                state: State::On,
                minx: 9,
                maxx: 12,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
        ];
        let expected = Cuboid {
            state: State::On,
            minx: -10,
            maxx: 12,
            miny: -5,
            maxy: 5,
            minz: 14,
            maxz: 20,
        };
        assert_eq!(merge(&c), vec![expected]);
    }

    #[test]
    fn test_merge_2() {
        let c = [
            Cuboid {
                state: State::On,
                minx: -10,
                maxx: 7,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
            Cuboid {
                state: State::On,
                minx: 9,
                maxx: 12,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
        ];
        assert_eq!(merge(&c), c.to_owned());
    }

    #[test]
    fn test_merge_3() {
        let c = [
            Cuboid {
                state: State::On,
                minx: -10,
                maxx: 14,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
            Cuboid {
                state: State::On,
                minx: 9,
                maxx: 12,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
        ];
        let expected = Cuboid {
            state: State::On,
            minx: -10,
            maxx: 14,
            miny: -5,
            maxy: 5,
            minz: 14,
            maxz: 20,
        };
        assert_eq!(merge(&c), vec![expected]);
    }

    #[test]
    fn test_merge_4() {
        let c = [
            Cuboid {
                state: State::On,
                minx: -10,
                maxx: 14,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
            Cuboid {
                state: State::On,
                minx: -10,
                maxx: 14,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
        ];
        let expected = Cuboid {
            state: State::On,
            minx: -10,
            maxx: 14,
            miny: -5,
            maxy: 5,
            minz: 14,
            maxz: 20,
        };
        assert_eq!(merge(&c), vec![expected]);
    }

    #[test]
    fn test_split_disjoint() {
        let c = [
            Cuboid {
                state: State::On,
                minx: -10,
                maxx: 7,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
            Cuboid {
                state: State::Off,
                minx: 8,
                maxx: 14,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
        ];
        assert_eq!(split(&c[0], &c[1]), vec![c[0]]);
    }

    #[test]
    fn test_split_same_but_different() {
        let c = [
            Cuboid {
                state: State::On,
                minx: -10,
                maxx: 7,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
            Cuboid {
                state: State::Off,
                minx: -10,
                maxx: 7,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
        ];
        assert_eq!(split(&c[0], &c[1]), vec![]);
    }

    #[test]
    fn test_split_same() {
        let c = [
            Cuboid {
                state: State::On,
                minx: -10,
                maxx: 7,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
            Cuboid {
                state: State::On,
                minx: -10,
                maxx: 7,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
        ];
        assert_eq!(split(&c[0], &c[1]), vec![]);
    }

    #[test]
    fn test_split_overlap_1() {
        let c = [
            Cuboid {
                state: State::On,
                minx: -10,
                maxx: 9,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
            Cuboid {
                state: State::Off,
                minx: 5,
                maxx: 12,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
        ];
        assert_eq!(
            split(&c[0], &c[1]),
            vec![Cuboid {
                state: State::On,
                minx: -10,
                maxx: 4,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },],
        );
    }

    #[test]
    fn test_split_overlap_1_2() {
        let c = [
            Cuboid {
                state: State::On,
                minx: -5,
                maxx: 5,
                miny: -10,
                maxy: 9,
                minz: 14,
                maxz: 20,
            },
            Cuboid {
                state: State::Off,
                minx: -5,
                maxx: 5,
                miny: 5,
                maxy: 12,
                minz: 14,
                maxz: 20,
            },
        ];
        assert_eq!(
            split(&c[0], &c[1]),
            vec![Cuboid {
                state: State::On,
                minx: -5,
                maxx: 5,
                miny: -10,
                maxy: 4,
                minz: 14,
                maxz: 20,
            },],
        );
    }

    #[test]
    fn test_split_overlap_1_3() {
        let c = [
            Cuboid {
                state: State::On,
                minx: -5,
                maxx: 5,
                miny: 14,
                maxy: 20,
                minz: -10,
                maxz: 9,
            },
            Cuboid {
                state: State::Off,
                minx: -5,
                maxx: 5,
                miny: 14,
                maxy: 20,
                minz: 5,
                maxz: 12,
            },
        ];
        assert_eq!(
            split(&c[0], &c[1]),
            vec![Cuboid {
                state: State::On,
                minx: -5,
                maxx: 5,
                miny: 14,
                maxy: 20,
                minz: -10,
                maxz: 4,
            },],
        );
    }

    #[test]
    fn test_split_overlap_2() {
        let c = [
            Cuboid {
                state: State::Off,
                minx: 5,
                maxx: 12,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
            Cuboid {
                state: State::On,
                minx: -10,
                maxx: 9,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
        ];
        assert_eq!(
            split(&c[0], &c[1]),
            vec![Cuboid {
                state: State::Off,
                minx: 10,
                maxx: 12,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },],
        );
    }

    #[test]
    fn test_split_overlap_3() {
        let c = [
            Cuboid {
                state: State::Off,
                minx: 5,
                maxx: 12,
                miny: -7,
                maxy: 3,
                minz: 16,
                maxz: 22,
            },
            Cuboid {
                state: State::On,
                minx: -10,
                maxx: 9,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
        ];
        assert_eq!(
            split(&c[0], &c[1]),
            vec![
                Cuboid {
                    state: State::Off,
                    minx: 5,
                    maxx: 9,
                    miny: -7,
                    maxy: -6,
                    minz: 16,
                    maxz: 22
                },
                Cuboid {
                    state: State::Off,
                    minx: 5,
                    maxx: 9,
                    miny: -5,
                    maxy: 3,
                    minz: 21,
                    maxz: 22
                },
                Cuboid {
                    state: State::Off,
                    minx: 10,
                    maxx: 12,
                    miny: -7,
                    maxy: 3,
                    minz: 16,
                    maxz: 22
                }
            ],
        );
    }

    #[test]
    fn test_split_overlap_4() {
        let c = [
            Cuboid {
                state: State::On,
                minx: -10,
                maxx: 7,
                miny: -5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },
            Cuboid {
                state: State::Off,
                minx: -10,
                maxx: 7,
                miny: -6,
                maxy: 4,
                minz: 14,
                maxz: 20,
            },
        ];
        assert_eq!(
            split(&c[0], &c[1]),
            vec![Cuboid {
                state: State::On,
                minx: -10,
                maxx: 7,
                miny: 5,
                maxy: 5,
                minz: 14,
                maxz: 20,
            },],
        );
    }
}
