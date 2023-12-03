use std::{collections::HashSet, iter::*, num::ParseIntError, str::FromStr};

use aoc::{manhattan_vec3, vec_add, Vec3};

#[derive(Debug, Clone)]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

fn parse_vec3(s: &str) -> Result<Vec3, aoc::ParseError> {
    let v: Result<Vec<i64>, ParseIntError> = aoc::split_ch(s, ',')
        .iter()
        .map(|x| x.parse::<i64>())
        .collect();
    let v = v?;
    if v.len() != 3 {
        return Err(aoc::ParseError::Generic);
    }
    Ok([v[0], v[1], v[2]])
}

impl FromStr for Particle {
    type Err = aoc::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = aoc::split_w(s);
        if parts.len() != 3 {
            return Err(aoc::ParseError::Generic);
        }
        let pos = parse_vec3(&parts[0][3..(parts[0].len() - 2)])?;
        let vel = parse_vec3(&parts[1][3..(parts[1].len() - 2)])?;
        let acc = parse_vec3(&parts[2][3..(parts[2].len() - 1)])?;
        Ok(Particle { pos, vel, acc })
    }
}

type ParsedItem = Particle;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    let mut particles = data.to_vec();
    let mut last_min_ix = None;
    let mut cnt = 0;
    loop {
        for p in &mut particles {
            p.vel = vec_add(p.vel, p.acc);
            p.pos = vec_add(p.pos, p.vel);
        }
        let mut min_dist = None;
        let mut min_ix = None;
        for (ix, p) in particles.iter().enumerate() {
            let dist = manhattan_vec3(p.pos, [0, 0, 0]);
            if let Some(d) = min_dist {
                if dist < d {
                    min_dist = Some(dist);
                    min_ix = Some(ix);
                }
            } else {
                min_dist = Some(dist);
                min_ix = Some(ix);
            }
        }
        if last_min_ix == min_ix {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt > 1000 {
            break;
        }
        last_min_ix = min_ix;
    }
    last_min_ix.unwrap() as i64
}

fn part2(data: &Parsed) -> Answer {
    let mut particles = data.to_vec();
    let mut cnt = 0;
    loop {
        for p in &mut particles {
            p.vel = vec_add(p.vel, p.acc);
            p.pos = vec_add(p.pos, p.vel);
        }
        let mut colliding = HashSet::new();
        for i in 0..particles.len() {
            for j in (i + 1)..particles.len() {
                if particles[i].pos == particles[j].pos {
                    colliding.insert(i);
                    colliding.insert(j);
                }
            }
        }
        // Have to remove from the back to not invalidate indices
        let mut colliding = colliding.into_iter().collect::<Vec<_>>();
        colliding.sort_by(|a, b| b.cmp(a));
        for i in &colliding {
            particles.remove(*i);
        }
        if colliding.is_empty() {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt > 1000 {
            break;
        }
    }
    particles.len() as i64
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
