use aoc::Grid;
use aoc::GridDrawer;
use aoc::GridTranspose;
use std::collections::{HashMap, HashSet};
use std::iter::*;

type Parsed = Vec<(i64, Vec<Vec<char>>)>;
type Answer = i64;

fn get_edge(grid: &Vec<Vec<char>>, dir: aoc::Point) -> Vec<char> {
    let ([_min_x, min_y], [_max_x, max_y]) = grid.extents();
    match dir {
        aoc::NORTH => grid[0].clone(),
        aoc::SOUTH => grid.last().unwrap().clone(),
        aoc::EAST => {
            let mut v = vec![];
            for y in min_y..=max_y {
                v.push(*grid[y as usize].last().unwrap());
            }
            v
        }
        aoc::WEST => {
            let mut v = vec![];
            for y in min_y..=max_y {
                v.push(grid[y as usize][0]);
            }
            v
        }
        _ => panic!(),
    }
}

fn get_matches(input: &Parsed) -> HashMap<i64, Vec<(i64, i64, i64, bool)>> {
    let mut matches: HashMap<i64, Vec<(i64, i64, i64, bool)>> = HashMap::new();
    for i in 0..input.len() {
        let edges = vec![
            get_edge(&input[i].1, aoc::NORTH),
            get_edge(&input[i].1, aoc::EAST),
            get_edge(&input[i].1, aoc::SOUTH),
            get_edge(&input[i].1, aoc::WEST),
        ];
        for j in 0..input.len() {
            if i == j {
                continue;
            }
            let other_edges = vec![
                get_edge(&input[j].1, aoc::NORTH),
                get_edge(&input[j].1, aoc::EAST),
                get_edge(&input[j].1, aoc::SOUTH),
                get_edge(&input[j].1, aoc::WEST),
            ];
            for (di, edge) in edges.iter().enumerate() {
                for (dj, other_edge) in other_edges.iter().enumerate() {
                    let mut other_edge_reversed = other_edge.to_owned();
                    other_edge_reversed.reverse();
                    if *edge == *other_edge {
                        matches
                            .entry(input[i].0)
                            .or_default()
                            .push((di as i64, input[j].0, dj as i64, false));
                    } else if *edge == other_edge_reversed {
                        matches
                            .entry(input[i].0)
                            .or_default()
                            .push((di as i64, input[j].0, dj as i64, true));
                    }
                }
            }
        }
    }
    matches
}

fn part1(input: &Parsed) -> Answer {
    let matches = get_matches(input);
    let mut ans = 1;
    for (id, b) in matches {
        // Corners have only two neighbours
        if b.len() == 2 {
            ans *= id;
        }
    }
    ans
}

fn find_monsters(grid: &HashMap<aoc::Point, char>) -> Vec<aoc::Point> {
    let monster = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    for g in grid.transpositions() {
        let ([min_x, min_y], [max_x, max_y]) = g.extents();
        let mut coords = vec![];
        for y in min_y..=max_y {
            'outer: for x in min_x..=max_x {
                let mut matches = 0;
                let mut monster_coords = vec![];
                for (yy, myy) in monster.iter().enumerate() {
                    for (xx, mc) in myy.chars().enumerate() {
                        if mc == '#' {
                            let xxx = x + xx as i64;
                            let yyy = y + yy as i64;
                            monster_coords.push([xxx, yyy]);
                        }
                    }
                }
                for gc in &monster_coords {
                    if let Some(c) = g.get_value(*gc) {
                        if c == '#' {
                            matches += 1;
                        }
                    } else {
                        // Monster is outside the picture, skip this coord
                        continue 'outer;
                    }
                }
                if matches == 15 {
                    coords.append(&mut monster_coords);
                }
            }
        }
        if !coords.is_empty() {
            return coords;
        }
    }
    vec![]
}

fn place(
    grids: &HashMap<i64, Vec<Vec<char>>>,
    corners: &[i64],
    coord: aoc::Point,
    placed: HashSet<i64>,
    grid_of_grids: HashMap<aoc::Point, (i64, Vec<Vec<char>>)>,
) -> HashMap<aoc::Point, (i64, Vec<Vec<char>>)> {
    let expected = (grids.len() as f64).sqrt() as i64;
    if grid_of_grids.len() == grids.len() {
        return grid_of_grids;
    }
    let mut candidates = vec![];
    for (id, g) in grids {
        if placed.contains(id) {
            continue;
        }
        if coord == [0, 0]
            || coord == [0, expected - 1]
            || coord == [expected - 1, 0]
            || coord == [expected - 1, expected - 1]
        {
            // Only allow corner tiles
            let mut found = false;
            for c in corners {
                if id == c {
                    found = true;
                    break;
                }
            }
            if !found {
                continue;
            }
        }
        // Try to make it fit with the neighbors
        'outer: for ggg in g.transpositions() {
            for d in &aoc::DIRECTIONS {
                let c = aoc::point_add(coord, *d);
                if let Some((_id, g)) = grid_of_grids.get(&c) {
                    let e = get_edge(&ggg, *d);
                    let other_e = match *d {
                        aoc::NORTH => get_edge(g, aoc::SOUTH),
                        aoc::EAST => get_edge(g, aoc::WEST),
                        aoc::SOUTH => get_edge(g, aoc::NORTH),
                        aoc::WEST => get_edge(g, aoc::EAST),
                        _ => panic!(),
                    };
                    if e != other_e {
                        continue 'outer;
                    }
                }
            }
            // All existing dirs matched, we can place this here
            candidates.push((id, ggg.clone()));
        }
    }
    if !candidates.is_empty() {
        for (id, g) in candidates {
            let mut gog = grid_of_grids.clone();
            gog.insert(coord, (*id, g.clone()));
            let mut p = placed.clone();
            p.insert(*id);
            // go to the next coord
            let new_coord = if coord[0] + 1 < expected {
                [coord[0] + 1, coord[1]]
            } else {
                [0, coord[1] + 1]
            };
            let next_gog = place(grids, corners, new_coord, p, gog);
            if !next_gog.is_empty() {
                return next_gog;
            }
        }
    }
    HashMap::new()
}

fn stitch_grids(
    grid_of_grids: &HashMap<aoc::Point, (i64, Vec<Vec<char>>)>,
) -> HashMap<aoc::Point, char> {
    let mut big_grid = HashMap::new();
    // Minus the borders
    if let Some((_, (_, g))) = grid_of_grids.iter().next() {
        let mut xx = 0;
        let mut yy = 0;
        let min_x = grid_of_grids.iter().map(|(p, _v)| p[0]).min().unwrap();
        let min_y = grid_of_grids.iter().map(|(p, _v)| p[1]).min().unwrap();
        let max_x = grid_of_grids.iter().map(|(p, _v)| p[0]).max().unwrap();
        let max_y = grid_of_grids.iter().map(|(p, _v)| p[1]).max().unwrap();
        let gh = (g.len() - 2) as i64;
        let gw = (g[0].len() - 2) as i64;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some((_id, g)) = grid_of_grids.get(&[x, y]) {
                    // Cut out the borders
                    big_grid.blit_rect([xx, yy], g, [1, 1], [gw, gh]);
                }
                xx += gw;
            }
            xx = 0;
            yy += gh;
        }
    }
    big_grid
}

fn part2(input: &Parsed) -> Answer {
    let mut grids = HashMap::new();
    for (id, g) in input {
        grids.insert(*id, g.clone());
    }
    let matches = get_matches(input);
    // Corners have only two neighbours
    let corners: Vec<_> = matches
        .iter()
        .filter_map(|(id, b)| if b.len() == 2 { Some(*id) } else { None })
        .collect();
    let grid_of_grids = place(&grids, &corners, [0, 0], HashSet::new(), HashMap::new());
    // Stitch the grids together
    let mut big_grid = stitch_grids(&grid_of_grids);
    // Find the sea monsters
    let hashes = big_grid.iter().filter(|(_p, v)| **v == '#').count();
    let m = find_monsters(&big_grid);
    let monsters = m.len();
    // Fill in the monsters
    for mc in m {
        big_grid.set_value(mc, 'O');
    }
    let mut gd = aoc::PrintGridDrawer::new(|c| c);
    gd.draw(&big_grid);
    (hashes - monsters) as i64
}

fn parse(lines: &[String]) -> Parsed {
    let grids = aoc::split_by_empty_line(lines);
    let mut r = vec![];
    for g in grids {
        let id = g[0][5..(g[0].len() - 1)].parse().unwrap();
        let grid = aoc::parse_grid(&g[1..]);
        r.push((id, grid));
    }
    r
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
