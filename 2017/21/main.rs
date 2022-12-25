use std::{collections::HashMap, iter::*, str::FromStr};

use aoc::{Grid, GridDrawer, GridTranspose, Point};

#[derive(Debug, Clone)]
struct Rule {
    sz: usize,
    pattern: HashMap<Point, char>,
    output: HashMap<Point, char>,
}

fn parse_to_grid(s: &str) -> (HashMap<Point, char>, usize) {
    let mut pat = HashMap::new();
    let mut y = 0;
    let mut x = 0;
    for c in s.chars() {
        if c == '/' {
            y += 1;
            x = 0;
            continue;
        } else {
            pat.insert([x, y], c);
        }
        x += 1;
    }
    (pat, (y + 1) as usize)
}

impl FromStr for Rule {
    type Err = aoc::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = aoc::split_w(s);
        if parts.len() != 3 {
            return Err(aoc::ParseError::Generic);
        }
        let (pattern, sz) = parse_to_grid(parts[0]);
        let (output, _) = parse_to_grid(parts[2]);
        Ok(Rule {
            sz,
            pattern,
            output,
        })
    }
}

type ParsedItem = Rule;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn solve(data: &Parsed, iterations: usize, draw: bool) -> Answer {
    let mut drawer = aoc::PrintGridDrawer::new(|c| c);
    let mut grid = aoc::parse_grid_to_sparse(&vec![".#.", "..#", "###"], Some);
    if draw {
        drawer.draw(&grid);
        println!();
    }
    let mut transpositions = vec![];
    for rule in data {
        let t = rule.pattern.transpositions().collect::<Vec<_>>();
        transpositions.push(t);
    }
    let mut g_sz = 3;
    for _ in 0..iterations {
        let sz = if g_sz % 2 == 0 { 2 } else { 3 };
        // construct new grid
        let new_sz = if sz == 2 { 3 } else { 4 };
        let mut new_grid = HashMap::new();
        let mut y = 0;
        let mut out_y = 0;
        while y < g_sz {
            let mut x = 0;
            let mut out_x = 0;
            while x < g_sz {
                // Copy the subgrid so we can compare
                let mut subgrid = HashMap::new();
                for yy in y..(y + sz) {
                    for xx in x..(x + sz) {
                        let p = [xx as i64, yy as i64];
                        if let Some(v) = grid.get_value(p) {
                            subgrid.insert([(xx - x) as i64, (yy - y) as i64], v);
                        }
                    }
                }
                let mut found = false;
                'outer: for (ix, rule) in data.iter().enumerate() {
                    if rule.sz == sz {
                        'pat: for pat in &transpositions[ix] {
                            if *pat != subgrid {
                                continue 'pat;
                            }
                            // println!("match");
                            found = true;
                            new_grid.blit([out_x as i64, out_y as i64], &rule.output);
                            break 'outer;
                        }
                    }
                }
                assert!(found);
                x += sz;
                out_x += new_sz;
            }
            y += sz;
            out_y += new_sz;
        }
        grid = new_grid;
        g_sz = g_sz * new_sz / sz;
        if draw {
            drawer.draw(&grid);
            println!();
        }
    }

    grid.points()
        .filter(|p| grid.get_value(*p).unwrap_or('.') == '#')
        .count() as i64
}

fn part1(data: &Parsed) -> Answer {
    solve(data, 5, true)
}

fn part2(data: &Parsed) -> Answer {
    solve(data, 18, false)
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
