use aoc::{Grid, Itertools, Point};
use std::{collections::HashMap, iter::*};

type Parsed = HashMap<Point, char>;

fn part1(data: &Parsed) -> i64 {
    data.iter()
        .filter_map(|(k, v)| if *v == '#' { Some(k) } else { None })
        .combinations(2)
        .map(|x| ((x[0][0] - x[1][0]).abs() + 1) * ((x[0][1] - x[1][1]).abs() + 1))
        .max()
        .unwrap()
}

fn part2(data: &Parsed) -> i64 {
    let ([_min_x, _min_y], [max_ext_x, _max_y]) = data.extents();
    let mut cache: HashMap<Point, bool> = HashMap::new();
    data.iter()
        .filter_map(|(k, v)| if *v == '#' { Some(k) } else { None })
        .combinations(2)
        .filter(|x| {
            let min_x = x[0][0].min(x[1][0]);
            let max_x = x[0][0].max(x[1][0]);
            let min_y = x[0][1].min(x[1][1]);
            let max_y = x[0][1].max(x[1][1]);
            for xx in min_x..=max_x {
                if !is_inside(data, [xx, min_y], max_ext_x, &mut cache) {
                    return false;
                }
                if !is_inside(data, [xx, max_y], max_ext_x, &mut cache) {
                    return false;
                }
            }
            for yy in min_y..=max_y {
                if !is_inside(data, [min_x, yy], max_ext_x, &mut cache) {
                    return false;
                }
                if !is_inside(data, [max_x, yy], max_ext_x, &mut cache) {
                    return false;
                }
            }
            true
        })
        .inspect(|x| println!("{:?}", x))
        .map(|x| ((x[0][0] - x[1][0]).abs() + 1) * ((x[0][1] - x[1][1]).abs() + 1))
        .max()
        .unwrap()
}

fn is_inside(data: &Parsed, p: Point, max_x: i64, cache: &mut HashMap<Point, bool>) -> bool {
    if data.get_value(p).is_some() {
        return true;
    }
    // if let Some(v) = cache.get(&p) {
    //     return *v;
    // }
    let [x, y] = p;
    let mut num = 0;
    let mut on = false;
    for xx in x..=(max_x + 1) {
        let p = [xx, y];
        if data.get_value(p).is_some() {
            if !on {
                num += 1;
            }
            on = true;
        } else {
            on = false;
        }
    }
    if num > 0 && num % 2 != 0 {
        // cache.insert(p, true);
        return true;
    }
    // cache.insert(p, false);
    false
}

fn parse(lines: &[String]) -> Parsed {
    let mut grid: HashMap<Point, char> = HashMap::new();
    let points = lines
        .iter()
        .map(|line| {
            let x: Vec<i64> = aoc::things(line);
            [x[0], x[1]]
        })
        .collect::<Vec<Point>>();
    for i in 0..points.len() {
        grid.line(points[i], points[(i + 1) % points.len()], 'X');
        grid.set_value(points[i], '#');
    }
    for p in &points {
        grid.set_value(*p, '#');
    }
    grid
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 50);
    }
}
