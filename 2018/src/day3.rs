use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;
use std::iter::*;
use crate::ppm::*;

struct Claim {
    id: i64,
    left: i64,
    top: i64,
    right: i64,
    bottom: i64
}

impl Claim {
    fn new(s: &str) -> Claim {
        //     #1 @ 669,271: 17x11
        let at = s.find('@').unwrap();
        let comma = s.find(',').unwrap();
        let colon = s.find(':').unwrap();
        let x = s.find('x').unwrap();
        let id = s[1..(at - 1)].parse::<i64>().unwrap();
        let left = s[(at + 2)..comma].parse::<i64>().unwrap();
        let top = s[(comma + 1)..colon].parse::<i64>().unwrap();
        let w = s[(colon + 2)..x].parse::<i64>().unwrap();
        let h =  s[(x + 1)..].parse::<i64>().unwrap();
        let right = left + w;
        let bottom = top + h;
        Claim {
            id: id,
            left: left,
            top: top,
            right: right,
            bottom: bottom
        }
    }
    fn overlap(&self, other: &Claim) -> bool {
        self.left < other.right && self.right > other.left && self.top < other.bottom && self.bottom > other.top
    }
    fn draw(&self, pixels: &mut [Pixel], color: &Color, stride: i64) {
        for y in self.top..self.bottom {
            for x in self.left..self.right {
                pixels[(y * stride + x) as usize] = *color;
            }
        }
    }
}

fn draw(claims: &[Claim], id: i64) {
    let max_x = claims.iter().map(|c| c.right).max().unwrap();
    let max_y = claims.iter().map(|c| c.bottom).max().unwrap();
    let mut pixels = vec![];
    pixels.resize((max_x * max_y) as usize, [0.0, 0.0, 0.0]);
    for claim in claims {
        if claim.id == id {
            claim.draw(&mut pixels, &[0.0, 1.0, 0.0], max_x);
        } else {
            claim.draw(&mut pixels, &[1.0, 0.0, 0.0], max_x);
        }
    }
    let filename = format!("{}.ppm", id);
    write_ppm_file(&pixels, max_x, max_y, &filename);
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Claim> {
    input.lines().map(|s| Claim::new(&s)).collect()
}

#[aoc(day3, part1)]
fn solve_pt1(claims: &[Claim]) -> Result<i64, Box<Error>> {
    let max_x = claims.iter().map(|c| c.right).max().unwrap();
    let max_y = claims.iter().map(|c| c.bottom).max().unwrap();
    let mut pixels = vec![];
    pixels.resize((max_y * max_x) as usize, 0);
    for claim in claims.iter() {
        for y in claim.top..claim.bottom {
            for x in claim.left..claim.right {
                pixels[(y * max_x + x) as usize] += 1;
            }
        }
    }
    let area = pixels.iter().filter(|&p| *p >= 2).count();
    Ok(area as i64)
}

#[aoc(day3, part2)]
fn solve_pt2(claims: &[Claim]) -> i64 {
    for a in 0..claims.len() {
        let claim_a = &claims[a];
        let mut overlap = false;
        for b in 0..claims.len() {
            if a != b {
                let claim_b = &claims[b];
                if claim_a.overlap(&claim_b) {
                    overlap = true;
                    break;
                }
            }
        }
        if !overlap {
            draw(&claims, claim_a.id);
            return claim_a.id;
        }
    }
    return -1;
}
