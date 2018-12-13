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
struct Star {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64
}

impl Star {
    fn new(s: &str) -> Star {
        // position=<-53868, -10684> velocity=< 5,  1>
        let x = s[10..16].trim().parse::<i64>().unwrap();
        let y = s[18..24].trim().parse::<i64>().unwrap();
        let dx = s[36..38].trim().parse::<i64>().unwrap();
        let dy = s[40..42].trim().parse::<i64>().unwrap();
        Star {
            x: x,
            y: y,
            dx: dx,
            dy: dy
        }
    }
}

fn draw(stars: &[Star], second: i64) -> bool {
    let max_x = stars.iter().map(|c| c.x).max().unwrap();
    let min_x = stars.iter().map(|c| c.x).min().unwrap();
    let max_y = stars.iter().map(|c| c.y).max().unwrap();
    let min_y = stars.iter().map(|c| c.y).min().unwrap();
    let w = max_x - min_x + 1;
    let h = max_y - min_y + 1;
    if w > 100 || h > 100 {
        return false;
    }
    println!("w: {}, h: {}, second: {}", w, h, second);
    let mut pixels = vec![];
    pixels.resize((w * h) as usize, [0.0, 0.0, 0.0]);
    for star in stars {
        let offset = (star.y - min_y) * w + (star.x - min_x);
        pixels[offset as usize] = [1.0, 1.0, 1.0];
    }
    let filename = format!("{}.ppm", second);
    write_ppm_file(&pixels, w, h, &filename);
    return true;
}

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let mut stars : Vec<Star> = buffered.lines().filter_map(Result::ok).map(|s| Star::new(&s)).collect();
    let mut second = 0;
    let mut any_drawn = false;
    let mut stopped_drawn = false;
    while !stopped_drawn {
        let drawn = draw(&stars, second);
        for star in &mut stars {
            star.x += star.dx;
            star.y += star.dy;
        }
        any_drawn = any_drawn || drawn;
        if any_drawn && !drawn {
            stopped_drawn = true;
        }
        second += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
