use std::fs::File;
use std::io::Write;
use std::path::Path;

pub type Color = [f64; 3];
pub type Pixel = [f64; 3];

pub fn write_ppm_file(pixels: &[Pixel], w: i64, h: i64, filename: &str) {
    let path = Path::new(filename);
    let mut file = File::create(path).unwrap();
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
