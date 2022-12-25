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

fn seg_steps(seg: &Vec<Segment>, pos: (i64, i64)) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut step = 0;
    for Segment{dir, length} in seg {
        for _a in 0..*length {
            if (x, y) == pos {
                return Some(step);
            }
            match dir {
                'U' => y += 1,
                'R' => x += 1,
                'L' => x -= 1,
                'D' => y -= 1,
                _ => panic!("OH NOES")
            }
            step += 1;
            if (x, y) == pos {
                return Some(step);
            }
        }
    }
    println!("{:?}, {:?}", seg, pos);
    None
}

fn steps(segments: &Vec<Vec<Segment>>, pos: (i64, i64)) -> Vec<i64> {
    let mut s = vec![];
    for seg in segments {
        s.push(seg_steps(&seg, pos).unwrap());
    }
    s
}

fn solve(segments: &Vec<Vec<Segment>>, part: i32) -> i64 {
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
            for a in 0..*length {
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
    let mut result = std::i64::MAX;
    for x in (f.0).0..=(f.0).1 {
        for y in (f.1).0..=(f.1).1 {
            let xx = x - xoffs;
            let yy = y - yoffs;
            let i = (yy * xsize + xx) as usize;
            if space[i] == 'X' {
                if part == 1 {
                    let mh = x.abs() + y.abs();
                    result = std::cmp::min(result, mh);
                } else {
                    let step = steps(&segments, (x, y));
                    let sum = step.iter().sum();
                    result = std::cmp::min(result, sum);
                }
            }
        }
    }
    result
}

fn part1(segments: &Vec<Vec<Segment>>) -> i64 {
    solve(segments, 1)
}

fn part2(segments: &Vec<Vec<Segment>>) -> i64 {
    solve(segments, 2)
}

fn segments(line: &str) -> Vec<Segment> {
    line.split(|c| c == ',').map(|s| s.trim()).map(|v| v.parse::<Segment>().unwrap()).collect()
}

fn input(path: &Path) -> Vec<Vec<Segment>> {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    lines.iter().map(|line| segments(&line)).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args[1].parse::<i32>().unwrap();
    let filename = &args[2];

    let parsed = input(Path::new(&filename));

    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, segments};

    #[test]
    fn test_part1() {
        assert_eq!(part1(&vec![segments("R8,U5,L5,D3"), segments("U7,R6,D4,L4")]), 6);
        assert_eq!(part1(&vec![segments("R75,D30,R83,U83,L12,D49,R71,U7,L72"), segments("U62,R66,U55,R34,D71,R55,D58,R83")]), 159);
        assert_eq!(part1(&vec![segments("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), segments("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")]), 135);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![segments("R8,U5,L5,D3"), segments("U7,R6,D4,L4")]), 30);
        assert_eq!(part2(&vec![segments("R75,D30,R83,U83,L12,D49,R71,U7,L72"), segments("U62,R66,U55,R34,D71,R55,D58,R83")]), 610);
        assert_eq!(part2(&vec![segments("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), segments("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")]), 410);
    }

}
