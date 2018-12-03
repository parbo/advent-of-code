use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

pub type Color = [f64; 3];
pub type Pixel = [f64; 3];
fn write_ppm_file(pixels: &[Pixel], w: i64, h: i64, filename: &str) {
    let path = Path::new(filename);
    let mut file = File::create(&path).unwrap();
    let header = format!("P6 {} {} 255\n", w, h);
    let mut data = vec![];
    data.reserve(header.len() + pixels.len() * 3);
    data.extend(header.as_bytes());
    for p in pixels {
        data.push((255.0 * p[0].max(0.0).min(1.0)) as u8);
        data.push((255.0 * p[1].max(0.0).min(1.0)) as u8);
        data.push((255.0 * p[2].max(0.0).min(1.0)) as u8);
    }
    file.write(&data).unwrap();
}

#[derive(Debug)]
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

fn solve(path: &Path) -> i64 {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let claims : Vec<Claim> = buffered.lines().filter_map(Result::ok).map(|s| Claim::new(&s)).collect();
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{}", result);
}
