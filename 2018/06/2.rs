use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64
}

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

fn solve(path: &Path) -> i64 {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut coords = vec![];
    for line in lines {
        let mut t = line.split(|c| [',', ' '].contains(&c)).filter(|s| s.len() > 0).map(|v| v.parse::<i64>().unwrap());
        let c = Coord { x: t.next().unwrap(), y: t.next().unwrap() };
        coords.push(c);
    }
    let min_x = coords.iter().map(|c| c.x).min().unwrap();
    let min_y = coords.iter().map(|c| c.y).min().unwrap();
    let max_x = coords.iter().map(|c| c.x).max().unwrap();
    let max_y = coords.iter().map(|c| c.y).max().unwrap();
    let w = max_x - min_x + 1;
    let h = max_y - min_y + 1;
    let mut v = vec![];
    v.resize((w * h) as usize, 0);
    for y in min_y..(max_y+1) {
        for x in min_x..(max_x+1) {
            let sum_mh_dist = coords.iter().map(|c| (c.x - x).abs() + (c.y - y).abs()).sum();
            v[((y - min_y) * w + (x - min_x)) as usize] = sum_mh_dist;
        }
    }
    let d = 10000;
    let a = v.iter().filter(|&c| *c < d).count();
    let mut pixels : Vec<Pixel>= vec![];
    let mut c = 0;
    for value in v {
        if value < d {
            pixels.push([0.0, (value as f64) / 10000.0, 0.0]);
            c += 1;
        } else {
            pixels.push([1.0, 0.0, 0.0]);
        }
    }
    println!("{} {} {}", c, a, w * h);
    write_ppm_file(&pixels, w, h, "area.ppm");
    return a as i64;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    println!("{}", result);
}
