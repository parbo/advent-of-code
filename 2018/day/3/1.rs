use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

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
}

fn solve(path: &Path) -> Result<i64, Box<Error>> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let claims : Vec<Claim> = buffered.lines().filter_map(Result::ok).map(|s| Claim::new(&s)).collect();
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve(Path::new(&filename));
    match result {
        Ok(str) => println!("{}", str),
        Err(err) => println!("{}", err)
    }
}
