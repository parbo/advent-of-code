use aoc::HexGrid;
use aoc::HexGridDrawer;
use aoc::Vec3;
use std::collections::HashMap;
use std::iter::*;

const HEX_E: Vec3 = [1, -1, 0];
const HEX_W: Vec3 = [-1, 1, 0];
const HEX_SE: Vec3 = [0, -1, 1];
const HEX_SW: Vec3 = [-1, 0, 1];
const HEX_NW: Vec3 = [0, 1, -1];
const HEX_NE: Vec3 = [1, 0, -1];

type Parsed = Vec<Vec<Vec3>>;
type Answer = usize;

fn init(paths: &Parsed) -> HashMap<Vec3, char> {
    let mut g = HashMap::new();
    for path in paths {
        let mut coord = [0, 0, 0];
        for d in path {
            coord = aoc::vec_add(coord, *d);
        }
        let col = g.entry(coord).or_insert('.');
        if *col == '.' {
            *col = 'B';
        } else {
            *col = '.';
        }
    }
    g
}

fn part1(paths: &Parsed) -> Answer {
    let tiles = init(paths);
    tiles.iter().filter(|(_coord, col)| **col == 'B').count()
}

fn part2(paths: &Parsed) -> Answer {
    let mut g = HashMap::new();
    let mut all_grid = vec![g.clone()];
    let draw = cfg!(feature = "vis");
    // init tiles to white
    for path in paths {
        let mut coord = [0, 0, 0];
        let mut gg = g.clone();
        for d in path {
            coord = aoc::vec_add(coord, *d);
            gg.insert(coord, '+');
        }
        let col = g.entry(coord).or_insert('.');
        if *col != 'B' {
            *col = 'B';
        } else {
            *col = '.';
        }
        gg.insert(coord, *col);
        if draw {
            all_grid.push(gg.clone());
        }
    }
    let d = [HEX_E, HEX_W, HEX_SW, HEX_SE, HEX_NW, HEX_NE];
    for _ in 0..100 {
        let mut newg = g.clone();
        // Note: extents is in axial coords
        let ([min_q, min_r], [max_q, max_r]) = newg.axial_extents();
        for q in (min_q - 1)..=(max_q + 1) {
            for r in (min_r - 1)..=(max_r + 1) {
                let p = aoc::axial_to_cube([q, r]);
                let mut black = 0;
                let c = g.get(&p).unwrap_or(&'.');
                for dir in &d {
                    let np = aoc::vec_add(p, *dir);
                    if let Some('B') = g.get(&np) {
                        black += 1;
                    }
                }
                if *c == 'B' && (black == 0 || black > 2) {
                    newg.insert(p, '.');
                } else if *c != 'B' && black == 2 {
                    newg.insert(p, 'B');
                }
            }
        }
        g = newg.clone();
        if draw {
            all_grid.push(g.clone());
        }
    }
    if draw {
        // Draw all the grids, using the same coord system
        let mut min_xx = 0;
        let mut min_yy = 0;
        let mut max_xx = 0;
        let mut max_yy = 0;
        for gg in &all_grid {
            // To figure the row/col span, use oddr coords
            let ([min_x, min_y], [max_x, max_y]) = gg.oddr_extents();
            min_xx = min_xx.min(min_x);
            min_yy = min_yy.min(min_y);
            max_xx = max_xx.max(max_x);
            max_yy = max_yy.max(max_y);
        }
        for gg in &mut all_grid {
            // Insert the min/max corners in all grids
            gg.insert(aoc::oddr_to_cube([min_xx, min_yy]), '.');
            gg.insert(aoc::oddr_to_cube([max_xx, max_yy]), '.');
        }
        let mut gd = aoc::BitmapHexGridDrawer::new(
            |x| {
                if x == 'B' {
                    [0, 0, 0]
                } else if x == '+' {
                    [200, 20, 20]
                } else if x == '*' {
                    [70, 70, 70]
                } else {
                    [255, 255, 255]
                }
            },
            "ppm/day24/part2",
        );
        let mut gdc = aoc::CursesHexGridDrawer::new(|c| c);
        for gg in &all_grid {
            gd.draw(gg);
            gdc.draw(gg);
        }
    }
    g.iter().filter(|(_coord, c)| **c == 'B').count()
}

fn parse(lines: &[String]) -> Parsed {
    let mut paths = vec![];
    for line in lines {
        let mut path = vec![];
        let mut cit = line.chars();
        while let Some(c) = cit.next() {
            match c {
                's' => {
                    if let Some(cc) = cit.next() {
                        match cc {
                            'e' => path.push(HEX_SE),
                            'w' => path.push(HEX_SW),
                            _ => panic!(),
                        }
                    }
                }
                'n' => {
                    if let Some(cc) = cit.next() {
                        match cc {
                            'e' => path.push(HEX_NE),
                            'w' => path.push(HEX_NW),
                            _ => panic!(),
                        }
                    }
                }
                'e' => path.push(HEX_E),
                'w' => path.push(HEX_W),
                _ => panic!(),
            }
        }
        paths.push(path);
    }
    paths
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            "sesenwnenenewseeswwswswwnenewsewsw".to_string(),
            "neeenesenwnwwswnenewnwwsewnenwseswesw".to_string(),
            "seswneswswsenwwnwse".to_string(),
            "nwnwneseeswswnenewneswwnewseswneseene".to_string(),
            "swweswneswnenwsewnwneneseenw".to_string(),
            "eesenwseswswnenwswnwnwsewwnwsene".to_string(),
            "sewnenenenesenwsewnenwwwse".to_string(),
            "wenwwweseeeweswwwnwwe".to_string(),
            "wsweesenenewnwwnwsenewsenwwsesesenwne".to_string(),
            "neeswseenwwswnwswswnw".to_string(),
            "nenwswwsewswnenenewsenwsenwnesesenew".to_string(),
            "enewnwewneswsewnwswenweswnenwsenwsw".to_string(),
            "sweneswneswneneenwnewenewwneswswnese".to_string(),
            "swwesenesewenwneswnwwneseswwne".to_string(),
            "enesenwswwswneneswsenwnewswseenwsese".to_string(),
            "wnwnesenesenenwwnenwsewesewsesesew".to_string(),
            "nenewswnwewswnenesenwnesewesw".to_string(),
            "eneswnwswnwsenenwnwnwwseeswneewsenese".to_string(),
            "neswnwewnwnwseenwseesewsenwsweewe".to_string(),
            "wseweeenwnesenwwwswnew".to_string(),
        ];
        let parsed = parse(&input);
        assert_eq!(part1(&parsed), 10);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "sesenwnenenewseeswwswswwnenewsewsw".to_string(),
            "neeenesenwnwwswnenewnwwsewnenwseswesw".to_string(),
            "seswneswswsenwwnwse".to_string(),
            "nwnwneseeswswnenewneswwnewseswneseene".to_string(),
            "swweswneswnenwsewnwneneseenw".to_string(),
            "eesenwseswswnenwswnwnwsewwnwsene".to_string(),
            "sewnenenenesenwsewnenwwwse".to_string(),
            "wenwwweseeeweswwwnwwe".to_string(),
            "wsweesenenewnwwnwsenewsenwwsesesenwne".to_string(),
            "neeswseenwwswnwswswnw".to_string(),
            "nenwswwsewswnenenewsenwsenwnesesenew".to_string(),
            "enewnwewneswsewnwswenweswnenwsenwsw".to_string(),
            "sweneswneswneneenwnewenewwneswswnese".to_string(),
            "swwesenesewenwneswnwwneseswwne".to_string(),
            "enesenwswwswneneswsenwnewswseenwsese".to_string(),
            "wnwnesenesenenwwnenwsewesewsesesew".to_string(),
            "nenewswnwewswnenesenwnesewesw".to_string(),
            "eneswnwswnwsenenwnwnwwseeswneewsenese".to_string(),
            "neswnwewnwnwseenwseesewsenwsweewe".to_string(),
            "wseweeenwnesenwwwswnew".to_string(),
        ];
        let parsed = parse(&input);
        assert_eq!(part2(&parsed), 2208);
    }
}
