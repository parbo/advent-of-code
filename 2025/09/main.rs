use aoc::{Grid, Itertools, Point};
use std::{collections::HashMap, iter::*};

type Parsed = Vec<Point>;

fn part1(data: &Parsed) -> i64 {
    data.iter()
        .combinations(2)
        .map(|x| ((x[0][0] - x[1][0]).abs() + 1) * ((x[0][1] - x[1][1]).abs() + 1))
        .max()
        .unwrap()
}

fn part2(data: &Parsed) -> i64 {
    data.iter()
        .combinations(2)
        .filter(|x| {
            let min_x = x[0][0].min(x[1][0]);
            let max_x = x[0][0].max(x[1][0]);
            let min_y = x[0][1].min(x[1][1]);
            let max_y = x[0][1].max(x[1][1]);
            for xx in (min_x + 1)..max_x {
                if !is_inside(data, [xx, min_y]) {
                    return false;
                }
                if !is_inside(data, [xx, max_y]) {
                    return false;
                }
            }
            for yy in (min_y + 1)..max_y {
                if !is_inside(data, [min_x, yy]) {
                    return false;
                }
                if !is_inside(data, [max_x, yy]) {
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

fn intersection(a1: Point, a2: Point, b1: Point, b2: Point) -> Option<Point> {
    assert!(a1[0] == a2[0] || a1[1] == a2[1]);
    assert!(b1[0] == b2[0] || b1[1] == b2[1]);
    if a1[0] == a2[0] {
        // a is vertical
        if b1[0] == b2[0] {
            if a1[0] == b1[0] {
                let min_x = a1[0].max(a2[0]).min(b1[0].max(b2[0]));
                let max_x = a1[0].min(a2[0]).max(b1[0].min(b2[0]));
                let min_y = a1[1].min(a2[1]).max(b1[1].min(b2[1]));
                let max_y = a1[1].max(a2[1]).min(b1[1].max(b2[1]));
                return Some([min_x + (max_x - min_x) / 2, min_y + (max_y - min_y) / 2]);
            }
        } else {
            // b is horizontal
            let x = a1[0];
            let y = b1[1];
            return Some([x, y]);
        }
    } else {
        // a is horizontal
        if b1[0] == b2[0] {
            // b is vertical
            let x = b1[0];
            let y = a1[1];
            return Some([x, y]);
        } else {
            // b is horizontal
            if a1[1] == b1[1] {
                let min_x = a1[0].max(a2[0]).min(b1[0].max(b2[0]));
                let max_x = a1[0].min(a2[0]).max(b1[0].min(b2[0]));
                let min_y = a1[1].min(a2[1]).max(b1[1].min(b2[1]));
                let max_y = a1[1].max(a2[1]).min(b1[1].max(b2[1]));
                return Some([min_x + (max_x - min_x) / 2, min_y + (max_y - min_y) / 2]);
            }
            return None;
        }
    }
    None
}

fn is_inside(polygon: &Parsed, point: Point) -> bool {
    let num_vertices = polygon.len();
    let x = point[0];
    let y = point[1];
    let mut inside = false;

    // Store the first point in the polygon and initialize
    // the second point
    let mut p1 = polygon[0];
    let mut p2;

    // Loop through each edge in the polygon
    for i in 1..=num_vertices {
        // Get the next point in the polygon
        p2 = polygon[i % num_vertices];

        // If the point is on the (straight) line segment, consider it inside
        if (x == p1[0] && y <= p1[1].max(p2[1]) && y >= p1[1].min(p2[1]))
            || (y == p1[1] && x <= p1[0].max(p2[0]) && x >= p1[0].min(p2[0]))
        {
            return true;
        }

        // Check if the point is above the minimum y
        // coordinate of the edge
        if y > p1[1].min(p2[1]) {
            // Check if the point is below the maximum y
            // coordinate of the edge
            if y <= p1[1].max(p2[1]) {
                // Check if the point is to the left of the
                // maximum x coordinate of the edge
                if x <= p1[0].max(p2[0]) {
                    // Calculate the x-intersection of the
                    // line connecting the point to the edge
                    let x_intersection = (y - p1[1]) * (p2[0] - p1[0]) / (p2[1] - p1[1]) + p1[0];

                    // Check if the point is on the same
                    // line as the edge or to the left of
                    // the x-intersection
                    if p1[0] == p2[0] || x <= x_intersection {
                        // Flip the inside flag
                        inside = !inside;
                    }
                }
            }
        }

        // Store the current point as the first point for
        // the next iteration
        p1 = p2;
    }

    // Return the value of the inside flag
    inside
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|line| {
            let x: Vec<i64> = aoc::things(line);
            [x[0], x[1]]
        })
        .collect()
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 24);
    }
}
