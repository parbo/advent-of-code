use aoc::{Grid, GridDrawer, Itertools, Point};
use std::{collections::HashMap, iter::*};

type Parsed = HashMap<Point, char>;

fn part1(data: &Parsed) -> i64 {
    data.keys()
        .combinations(2)
        .map(|x| ((x[0][0] - x[1][0]).abs() + 1) * ((x[0][1] - x[1][1]).abs() + 1))
        .max()
        .unwrap()
}

fn part2(data: &Parsed) -> i64 {
    0
}

fn fill(data: &mut Parsed) {
    let ([min_x, min_y], [max_x, max_y]) = data.extents();
    let mut todo = vec![];
    'outer: for y in (min_y + 1)..=(max_y - 1) {
        for x in (min_x + 1)..=(max_x - 1) {
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
            // dbg!(x, y, num);
            if num > 0 && num % 2 != 0 {
                todo.push([x, y]);
                break 'outer;
            }
        }
        dbg!(y);
    }
    dbg!(&todo);
    while let Some(p) = todo.pop() {
        let curr = data.get_value(p);
        if curr.is_none() {
            data.set_value(p, 'X');
            if p[0] > min_x {
                todo.push([p[0] - 1, p[1]]);
            }
            if p[0] < max_x {
                todo.push([p[0] + 1, p[1]]);
            }
            if p[1] > min_y {
                todo.push([p[0], p[1] - 1]);
            }
            if p[1] < max_y {
                todo.push([p[0], p[1] + 1]);
            }
        }
    }
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
    fill(&mut grid);
    // let mut gd = aoc::BitmapGridDrawer::new(
    //     |c| match c {
    //         '#' => [255, 0, 0],
    //         'X' => [0, 255, 0],
    //         _ => [0, 0, 0],
    //     },
    //     "day09",
    // );
    // let mut gd = aoc::PrintGridDrawer::new(|c| c);
    // gd.draw(&grid);
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
