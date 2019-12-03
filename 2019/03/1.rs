use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
struct Segment {
    dir: char,
    length: i64
}

impl FromStr for Segment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = s.chars().next().unwrap();
        let length = s[1..].parse::<i64>()?;

        Ok(Segment { dir, length })
    }
}

fn extents(segment: &[Segment]) -> ((i64, i64), (i64, i64)) {
    let mut x = 0;
    let mut y = 0;
    let mut max_x = 0;
    let mut min_x = 0;
    let mut max_y = 0;
    let mut min_y = 0;
    for Segment{dir, length} in segment {
        match dir {
            'U' => y += length,
            'R' => x += length,
            'L' => x -= length,
            'D' => y -= length,
            _ => panic!("OH NOES")
        }
        max_x = std::cmp::max(x, max_x);
        max_y = std::cmp::max(y, max_y);
        min_x = std::cmp::min(x, min_x);
        min_y = std::cmp::min(y, min_y);
    }
    ((min_x, max_x), (min_y, max_y))
}

fn field(exts: &[((i64, i64), (i64, i64))]) -> ((i64, i64), (i64, i64)) {
    let mut max_x = 0;
    let mut min_x = 0;
    let mut max_y = 0;
    let mut min_y = 0;
    for ((x1, x2), (y1, y2)) in exts {
        max_x = std::cmp::max(*x1, max_x);
        max_x = std::cmp::max(*x2, max_x);
        max_y = std::cmp::max(*y1, max_y);
        max_y = std::cmp::max(*y2, max_y);
        min_x = std::cmp::min(*x1, min_x);
        min_x = std::cmp::min(*x2, min_x);
        min_y = std::cmp::min(*y1, min_y);
        min_y = std::cmp::min(*y1, min_y);
    }
    ((min_x, max_x), (min_y, max_y))
}

fn solve_input(path: &Path) -> i64 {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    solve(&lines)
}

fn solve(lines: &Vec<String>) -> i64 {
    let mut segments : Vec<Vec<Segment>> = Vec::new();
    for line in lines {
        let seg = line.split(|c| c == ',').map(|s| s.trim()).map(|v| v.parse::<Segment>().unwrap()).collect();
        segments.push(seg);
    }
    let exts : Vec<((i64, i64), (i64, i64))> = segments.iter().map(|s| extents(s)).collect();
    let f = field(&exts);
    let xoffs = (f.0).0;
    let yoffs = (f.1).0;
    let xsize = (f.0).1 - (f.0).0 + 1;
    let ysize = (f.1).1 - (f.1).0 + 1;
    println!("{:?}", f);
    println!("{}, {}", xoffs, yoffs);
    println!("{}, {}", xsize, ysize);
    let mut space : Vec<char> = vec!['.'; (xsize * ysize) as usize];
    space[((0 - yoffs) * xsize + 0) as usize] = 'o';
    for seg in segments {
        let orig_space = space.clone();
        let mut x = 0;
        let mut y = 0;
        let mut segno = 0;
        for Segment{dir, length} in seg {
            for a in 0..length {
                if segno != 0 && a == 0 {
                    let x0 = x - xoffs;
                    let y0 = y - yoffs;
                    let i0 = (y0 * xsize + x0) as usize;
                    space[i0] = '+';
                }
                segno += 1;
                match dir {
                    'U' => y += 1,
                    'R' => x += 1,
                    'L' => x -= 1,
                    'D' => y -= 1,
                    _ => panic!("OH NOES")
                }
                let xx = x - xoffs;
                let yy = y - yoffs;
                let i = (yy * xsize + xx) as usize;
                if space[i] != 'o' && orig_space[i] != '.' {
                    space[i] = 'X';
                } else {
                    match dir {
                        'U' => space[i] = '|',
                        'R' => space[i] = '-',
                        'L' => space[i] = '-',
                        'D' => space[i] = '|',
                        _ => panic!("OH NOES")
                    }
                }
            }
        }
    }
    // for y in (f.1).0..=(f.1).1 {
    //     for x in (f.0).0..=(f.0).1 {
    //         let xx = x - xoffs;
    //         let yy = y - yoffs;
    //         let i = ((ysize - yy - 1) * xsize + xx) as usize;
    //         print!("{}", space[i])
    //     }
    //     println!();
    // }
    println!("find mh");
    let mut min_mh = xsize + ysize;
    for x in (f.0).0..=(f.0).1 {
        for y in (f.1).0..=(f.1).1 {
            let xx = x - xoffs;
            let yy = y - yoffs;
            let i = (yy * xsize + xx) as usize;
            if space[i] == 'X' {
                println!("{}, {}, {}, {}, {}", x, y, xx, yy, x.abs() + y.abs());
                min_mh = std::cmp::min(min_mh, x.abs() + y.abs());
            }
        }
    }
    min_mh
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve_input(Path::new(&filename));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        assert_eq!(solve(&vec!["R8,U5,L5,D3".to_string(), "U7,R6,D4,L4".to_string()]), 6);
        assert_eq!(solve(&vec!["R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(), "U62,R66,U55,R34,D71,R55,D58,R83".to_string()]), 159);
        assert_eq!(solve(&vec!["R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(), "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string()]), 135);
    }
}
