use core::fmt::Write;
use image::{GenericImageView, Rgb, RgbImage};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::env;
use std::error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::*;
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Instant;

#[macro_use]
extern crate lazy_static;

extern crate vecmath;

pub use itertools::Itertools;
pub use mod_exp::mod_exp;
// pub use num::integer::*;
// pub use pancurses::*;
pub use petgraph::algo;
pub use petgraph::graph::DiGraph;
pub use petgraph::graph::Graph;
pub use petgraph::graph::UnGraph;
pub use petgraph::graphmap::DiGraphMap;
pub use petgraph::graphmap::GraphMap;
pub use petgraph::graphmap::UnGraphMap;
pub use petgraph::visit;
pub use petgraph::Direction::Outgoing;
// pub use petgraph::*;
// pub use regex::*;
pub use serde_scan::from_str;
pub use serde_scan::scan;

pub use fnv::FnvHashMap;
pub use fnv::FnvHashSet;
pub use rustc_hash::FxHashMap;
pub use rustc_hash::FxHashSet;

pub type Point = self::vecmath::Vector2<i64>;
pub type FPoint = self::vecmath::Vector2<f64>;
pub type Vec3 = self::vecmath::Vector3<i64>;
pub type FVec3 = self::vecmath::Vector3<f64>;
pub type Vec4 = self::vecmath::Vector4<i64>;
pub type FVec4 = self::vecmath::Vector4<f64>;
pub type Mat4 = self::vecmath::Matrix4<i64>;
pub type FMat4 = self::vecmath::Matrix4<f64>;
pub type Mat3 = self::vecmath::Matrix3<i64>;

pub use self::vecmath::mat3_id;
pub use self::vecmath::mat3_inv;
pub use self::vecmath::mat4_id as mat_identity;
pub use self::vecmath::mat4_inv as mat_inv;
pub use self::vecmath::mat4_transposed as mat_transpose;
pub use self::vecmath::row_mat3_mul;
pub use self::vecmath::row_mat3_transform_pos2;
pub use self::vecmath::row_mat3_transform_vec2;
pub use self::vecmath::row_mat4_mul as mat_mul;
pub use self::vecmath::row_mat4_transform as mat_transform;
pub use self::vecmath::vec2_add as point_add;
pub use self::vecmath::vec2_dot as point_dot;
pub use self::vecmath::vec2_neg as point_neg;
pub use self::vecmath::vec2_normalized as point_normalize;
pub use self::vecmath::vec2_scale as point_mul;
pub use self::vecmath::vec2_square_len as point_square_length;
pub use self::vecmath::vec2_sub as point_sub;
pub use self::vecmath::vec3_add as vec_add;
pub use self::vecmath::vec3_cross as vec_cross;
pub use self::vecmath::vec3_dot as vec_dot;
pub use self::vecmath::vec3_neg as vec_neg;
pub use self::vecmath::vec3_normalized as vec_normalize;
pub use self::vecmath::vec3_scale as vec_mul;
pub use self::vecmath::vec3_square_len as vec_square_length;
pub use self::vecmath::vec3_sub as vec_sub;
pub use self::vecmath::vec4_add;
pub use self::vecmath::vec4_neg;
pub use self::vecmath::vec4_sub;

pub fn length(v: FVec3) -> f64 {
    vec_square_length(v).sqrt()
}

pub fn cmul(v1: Vec3, v2: Vec3) -> Vec3 {
    let [x1, y1, z1] = v1;
    let [x2, y2, z2] = v2;
    [x1 * x2, y1 * y2, z1 * z2]
}

pub fn cmul2(v1: Point, v2: Point) -> Point {
    let [x1, y1] = v1;
    let [x2, y2] = v2;
    [x1 * x2, y1 * y2]
}

pub fn inside_extent(p: Point, extent: (Point, Point)) -> bool {
    let min_x = extent.0[0];
    let min_y = extent.0[1];
    let max_x = extent.1[0];
    let max_y = extent.1[1];
    p[0] >= min_x && p[0] <= max_x && p[1] >= min_y && p[1] <= max_y
}

pub const NORTH: Point = [0, -1];
pub const UP: Point = NORTH;
pub const NORTH_EAST: Point = [1, -1];
pub const UP_RIGHT: Point = NORTH_EAST;
pub const EAST: Point = [1, 0];
pub const RIGHT: Point = EAST;
pub const SOUTH_EAST: Point = [1, 1];
pub const DOWN_RIGHT: Point = SOUTH_EAST;
pub const SOUTH: Point = [0, 1];
pub const DOWN: Point = SOUTH;
pub const SOUTH_WEST: Point = [-1, 1];
pub const DOWN_LEFT: Point = SOUTH_WEST;
pub const WEST: Point = [-1, 0];
pub const LEFT: Point = WEST;
pub const NORTH_WEST: Point = [-1, -1];
pub const UP_LEFT: Point = NORTH_WEST;

// Hex directions
// https://www.redblobgames.com/grids/hexagons/
pub const HEX_E: Vec3 = [1, -1, 0];
pub const HEX_W: Vec3 = [-1, 1, 0];
pub const HEX_SE: Vec3 = [0, -1, 1];
pub const HEX_SW: Vec3 = [-1, 0, 1];
pub const HEX_NW: Vec3 = [0, 1, -1];
pub const HEX_NE: Vec3 = [1, 0, -1];

pub const HEX_ALT_SE: Vec3 = [1, -1, 0];
pub const HEX_ALT_NW: Vec3 = [-1, 1, 0];
pub const HEX_ALT_S: Vec3 = [0, -1, 1];
pub const HEX_ALT_SW: Vec3 = [-1, 0, 1];
pub const HEX_ALT_N: Vec3 = [0, 1, -1];
pub const HEX_ALT_NE: Vec3 = [1, 0, -1];

pub const DIRECTIONS: [Point; 4] = [NORTH, EAST, SOUTH, WEST];
pub const DIRECTIONS_INCL_DIAGONALS: [Point; 8] = [
    NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
];
pub const HEX_DIRECTIONS: [Vec3; 6] = [HEX_E, HEX_W, HEX_SW, HEX_SE, HEX_NW, HEX_NE];

pub fn neighbors(p: Point) -> impl Iterator<Item = Point> {
    let mut diter = DIRECTIONS.iter();
    from_fn(move || diter.next().map(|d| point_add(p, *d)))
}

pub fn neighbors_incl_diagonals(p: Point) -> impl Iterator<Item = Point> {
    let mut diter = DIRECTIONS_INCL_DIAGONALS.iter();
    from_fn(move || diter.next().map(|d| point_add(p, *d)))
}

pub fn hex_neighbors(p: Vec3) -> impl Iterator<Item = Vec3> {
    let mut diter = HEX_DIRECTIONS.iter();
    from_fn(move || diter.next().map(|d| vec_add(p, *d)))
}

lazy_static! {
    pub static ref DIRECTION_ROTATE_LEFT: HashMap<Point, Point> = {
        let mut map = HashMap::new();
        map.insert(NORTH, WEST);
        map.insert(WEST, SOUTH);
        map.insert(SOUTH, EAST);
        map.insert(EAST, NORTH);
        map
    };
}

lazy_static! {
    pub static ref DIRECTION_ROTATE_RIGHT: HashMap<Point, Point> = {
        let mut map = HashMap::new();
        map.insert(NORTH, EAST);
        map.insert(EAST, SOUTH);
        map.insert(SOUTH, WEST);
        map.insert(WEST, NORTH);
        map
    };
}

lazy_static! {
    pub static ref DIRECTION_MAP: HashMap<&'static str, Point> = {
        let mut map = HashMap::new();
        map.insert("U", NORTH);
        map.insert("u", NORTH);
        map.insert("D", SOUTH);
        map.insert("d", SOUTH);
        map.insert("R", EAST);
        map.insert("r", EAST);
        map.insert("L", WEST);
        map.insert("l", WEST);
        map.insert("N", NORTH);
        map.insert("n", NORTH);
        map.insert("S", SOUTH);
        map.insert("s", SOUTH);
        map.insert("E", EAST);
        map.insert("e", EAST);
        map.insert("W", WEST);
        map.insert("w", WEST);
        map.insert("NW", NORTH_WEST);
        map.insert("nw", NORTH_WEST);
        map.insert("SW", SOUTH_WEST);
        map.insert("sw", SOUTH_WEST);
        map.insert("NE", NORTH_WEST);
        map.insert("ne", NORTH_WEST);
        map.insert("SE", SOUTH_EAST);
        map.insert("se", SOUTH_EAST);
        map
    };
}

lazy_static! {
    pub static ref HEX_DIRECTION_MAP: HashMap<&'static str, Vec3> = {
        let mut map = HashMap::new();
        map.insert("E", HEX_E);
        map.insert("e", HEX_E);
        map.insert("W", HEX_W);
        map.insert("w", HEX_W);
        map.insert("NW", HEX_NW);
        map.insert("nw", HEX_NW);
        map.insert("SW", HEX_SW);
        map.insert("sw", HEX_SW);
        map.insert("NE", HEX_NW);
        map.insert("ne", HEX_NW);
        map.insert("SE", HEX_SE);
        map.insert("se", HEX_SE);
        map
    };
}

lazy_static! {
    pub static ref HEX_ALT_DIRECTION_MAP: HashMap<&'static str, Vec3> = {
        let mut map = HashMap::new();
        map.insert("N", HEX_ALT_N);
        map.insert("n", HEX_ALT_N);
        map.insert("S", HEX_ALT_S);
        map.insert("s", HEX_ALT_S);
        map.insert("NW", HEX_ALT_NW);
        map.insert("nw", HEX_ALT_NW);
        map.insert("SW", HEX_ALT_SW);
        map.insert("sw", HEX_ALT_SW);
        map.insert("NE", HEX_ALT_NE);
        map.insert("ne", HEX_ALT_NE);
        map.insert("SE", HEX_ALT_SE);
        map.insert("se", HEX_ALT_SE);
        map
    };
}

#[derive(Debug)]
pub enum ParseError {
    Generic,
    Parse(ParseIntError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Generic => write!(f, "some error"),
            ParseError::Parse(..) => write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ParseError::Parse(ref e) => Some(e),
            ParseError::Generic => None,
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::Parse(err)
    }
}

pub fn split(s: &str, pred: fn(char) -> bool) -> Vec<&str> {
    s.split(pred)
        .map(|w| w.trim())
        .filter(|x| !x.is_empty())
        .collect()
}

pub fn split_w(s: &str) -> Vec<&str> {
    s.split(|c: char| c.is_whitespace())
        .map(|w| w.trim())
        .filter(|x| !x.is_empty())
        .collect()
}

pub fn split_ch(s: &str, c: char) -> Vec<&str> {
    s.split(|x| x == c)
        .map(|w| w.trim())
        .filter(|x| !x.is_empty())
        .collect()
}

pub fn split_str<'a>(s: &'a str, pat: &str) -> Vec<&'a str> {
    s.split(pat)
        .map(|w| w.trim())
        .filter(|x| !x.is_empty())
        .collect()
}

pub fn split_by_empty_line<'a, I, J>(lines: I) -> Vec<Vec<&'a str>>
where
    I: IntoIterator<Item = &'a J>,
    J: AsRef<str> + 'a,
{
    lines
        .into_iter()
        .group_by(|line| !AsRef::as_ref(line).is_empty())
        .into_iter()
        .map(|(_, group)| {
            group
                .map(|s| AsRef::as_ref(s))
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .filter(|g| !g.is_empty())
        .collect()
}

pub fn parse_to_vec_of<'a, I, J, T, E>(items: I) -> Result<Vec<T>, ParseError>
where
    I: IntoIterator<Item = &'a J>,
    J: AsRef<str> + 'a,
    T: FromStr<Err = E>,
    E: Into<ParseError>,
{
    items
        .into_iter()
        .map(|x| T::from_str(AsRef::as_ref(x)).map_err(|x| -> ParseError { x.into() }))
        .collect()
}

pub fn parse_grid<'a, I, J>(lines: I) -> Vec<Vec<char>>
where
    I: IntoIterator<Item = &'a J>,
    J: AsRef<str> + 'a,
{
    lines
        .into_iter()
        .map(|x| AsRef::as_ref(x).chars().collect())
        .collect()
}

pub fn parse_grid_to<'a, I, J, T>(lines: I, f: fn(char) -> T) -> Vec<Vec<T>>
where
    I: IntoIterator<Item = &'a J>,
    J: AsRef<str> + 'a,
{
    lines
        .into_iter()
        .map(|x| AsRef::as_ref(x).chars().map(f).collect())
        .collect()
}

pub fn parse_grid_to_sparse<'a, I, J, T>(lines: I, f: fn(char) -> Option<T>) -> HashMap<Point, T>
where
    I: IntoIterator<Item = &'a J>,
    J: AsRef<str> + 'a,
{
    let mut grid = HashMap::new();
    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in AsRef::as_ref(line).chars().enumerate() {
            if let Some(t) = f(c) {
                grid.insert([x as i64, y as i64], t);
            }
        }
    }
    grid
}

pub fn grid_to_undirected_graph<T>(
    grid: &dyn Grid<T>,
    is_node: fn(&Point, &T) -> bool,
    get_edge_cost: fn(&Point, &T, &Point, &T) -> Option<i64>,
    directions: usize,
) -> UnGraphMap<Point, i64>
where
    T: PartialEq + Copy,
{
    let directions = match directions {
        4 => DIRECTIONS.to_vec(),
        8 => DIRECTIONS_INCL_DIAGONALS.to_vec(),
        _ => panic!(),
    };

    let mut graph = UnGraphMap::new();
    let (min, max) = grid.extents();

    for y in min[1]..=max[1] {
        for x in min[0]..=max[0] {
            let p: Point = [x as i64, y as i64];
            if let Some(c) = grid.get_value(p) {
                if is_node(&p, &c) {
                    let gp = graph.add_node(p);
                    for d in &directions {
                        let np = point_add(p, *d);
                        if np[0] >= min[0] && np[0] <= max[0] && np[1] >= min[1] && np[1] <= max[1]
                        {
                            if let Some(nc) = grid.get_value(np) {
                                if is_node(&np, &nc) {
                                    if let Some(e) = get_edge_cost(&p, &c, &np, &nc) {
                                        let gnp = graph.add_node(np);
                                        // Make sure it's an undirected graph
                                        if let Some(ew) = graph.edge_weight(gp, gnp) {
                                            assert_eq!(e, *ew);
                                        }
                                        graph.add_edge(gp, gnp, e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    graph
}

pub fn grid_to_directed_graph<T>(
    grid: &dyn Grid<T>,
    is_node: fn(&Point, &T) -> bool,
    get_edge_cost: fn(&Point, &T, &Point, &T) -> Option<i64>,
    directions: usize,
) -> DiGraphMap<Point, i64>
where
    T: PartialEq + Copy,
{
    let directions = match directions {
        4 => DIRECTIONS.to_vec(),
        8 => DIRECTIONS_INCL_DIAGONALS.to_vec(),
        _ => panic!(),
    };

    let mut graph = DiGraphMap::new();
    let (min, max) = grid.extents();

    for y in min[1]..=max[1] {
        for x in min[0]..=max[0] {
            let p: Point = [x as i64, y as i64];
            if let Some(c) = grid.get_value(p) {
                if is_node(&p, &c) {
                    let gp = graph.add_node(p);
                    for d in &directions {
                        let np = point_add(p, *d);
                        if np[0] >= min[0] && np[0] <= max[0] && np[1] >= min[1] && np[1] <= max[1]
                        {
                            if let Some(nc) = grid.get_value(np) {
                                if is_node(&np, &nc) {
                                    if let Some(e) = get_edge_cost(&p, &c, &np, &nc) {
                                        let gnp = graph.add_node(np);
                                        graph.add_edge(gp, gnp, e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    graph
}

pub fn astar_graph<T: petgraph::EdgeType>(
    graph: &GraphMap<Point, i64, T>,
    start: Point,
    goal: Point,
) -> Option<(i64, Vec<Point>)> {
    petgraph::algo::astar(
        &graph,
        start,
        |finish| finish == goal,                             // is finish
        |(_n1, _n2, e)| *e,                                  // true cost
        |n| (goal[0] - n[0]).abs() + (goal[1] - n[1]).abs(), // estimated cost: manhattan distance}
    )
}

pub fn manhattan(n: Point, goal: Point) -> i64 {
    (goal[0] - n[0]).abs() + (goal[1] - n[1]).abs()
}

pub fn manhattan_vec3(n: Vec3, goal: Vec3) -> i64 {
    (goal[0] - n[0]).abs() + (goal[1] - n[1]).abs() + (goal[2] - n[2]).abs()
}

pub fn manhattan_vec4(n: Vec4, goal: Vec4) -> i64 {
    (goal[0] - n[0]).abs()
        + (goal[1] - n[1]).abs()
        + (goal[2] - n[2]).abs()
        + (goal[3] - n[3]).abs()
}

pub fn manhattan_hex_cube(n: Vec3, goal: Vec3) -> i64 {
    ((goal[0] - n[0]).abs() + (goal[1] - n[1]).abs() + (goal[2] - n[2]).abs()) / 2
}

pub fn astar_grid<T>(
    grid: &dyn Grid<T>,
    is_node: fn(&Point, &T) -> bool,
    get_edge_cost: fn(&Point, &T, &Point, &T) -> Option<i64>,
    start: Point,
    goal: Point,
) -> Option<(i64, Vec<Point>)>
where
    T: PartialEq + Copy,
{
    let mut frontier = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut gscore = HashMap::new();
    let mut fscore = HashMap::new();
    gscore.insert(start, 0);
    fscore.insert(start, manhattan(start, goal));
    frontier.push(Reverse((manhattan(start, goal), start)));
    while let Some(Reverse((_est, current))) = frontier.pop() {
        if current == goal {
            let mut path = vec![goal];
            let mut curr = goal;
            while curr != start {
                curr = came_from[&curr];
                path.push(curr)
            }
            return Some((gscore.get_value(goal).unwrap(), path));
        }
        let g = *gscore.entry(current).or_insert(i64::MAX);
        let curr_val = grid.get_value(current).unwrap();
        for nb in neighbors(current) {
            if let Some(value) = grid.get_value(nb) {
                if is_node(&nb, &value) {
                    if let Some(edge_cost) = get_edge_cost(&current, &curr_val, &nb, &value) {
                        let new_g = g + edge_cost;
                        let nb_g = gscore.entry(nb).or_insert(i64::MAX);
                        if new_g < *nb_g {
                            came_from.insert(nb, current);
                            *nb_g = new_g;
                            let new_f = new_g + manhattan(goal, nb);
                            *fscore.entry(nb).or_insert(i64::MAX) = new_f;
                            frontier.push(Reverse((new_f, nb)));
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn dijkstra_grid<T>(
    grid: &dyn Grid<T>,
    is_node: fn(&Point, &T) -> bool,
    get_edge_cost: fn(&Point, &T, &Point, &T) -> Option<i64>,
    start: Point,
    goal: Point,
) -> Option<(i64, Vec<Point>)>
where
    T: PartialEq + Copy,
{
    let mut frontier = BinaryHeap::new();
    let mut visited: HashSet<Point> = HashSet::new();
    let mut came_from = HashMap::new();
    frontier.push(Reverse((0, start)));
    while let Some(Reverse((score, current))) = frontier.pop() {
        if visited.contains(&current) {
            continue;
        }
        if current == goal {
            let mut path = vec![goal];
            let mut curr = goal;
            while curr != start {
                curr = came_from[&curr];
                path.push(curr)
            }
            return Some((score, path.into_iter().rev().collect()));
        }
        let curr_val = grid.get_value(current).unwrap();
        for nb in neighbors(current) {
            if visited.contains(&nb) {
                continue;
            }
            if let Some(value) = grid.get_value(nb) {
                if is_node(&nb, &value) {
                    if let Some(edge_cost) = get_edge_cost(&current, &curr_val, &nb, &value) {
                        let new_score = score + edge_cost;
                        came_from.insert(nb, current);
                        frontier.push(Reverse((new_score, nb)));
                    }
                }
            }
        }
        visited.insert(current);
    }
    None
}

pub fn get_char(s: &str, ix: usize) -> Option<char> {
    s.chars().nth(ix)
}

pub fn parse_char(s: &str, ix: usize) -> Result<char, ParseError> {
    get_char(s, ix).ok_or(ParseError::Generic)
}

pub fn parse_point(s: &str) -> Result<Point, ParseError> {
    let parts: Vec<&str> = s.split(|x| x == ',').map(|w| w.trim()).collect();
    if parts.len() == 2 {
        Ok([parts[0].parse()?, parts[1].parse()?])
    } else {
        Err(ParseError::Generic)
    }
}

pub fn cum_sum<T: num::Num + Copy>(a: &[T]) -> Vec<T> {
    a.iter()
        .scan(T::zero(), |state, x| {
            *state = *state + *x;
            Some(*state)
        })
        .collect()
}

pub fn range_sum_inclusive<T: num::Num + Copy>(cum_sum: &[T], a: usize, b: usize) -> T {
    if b < a {
        T::zero()
    } else if a == 0 {
        cum_sum[b]
    } else {
        cum_sum[b] - cum_sum[a - 1]
    }
}

pub fn range_sum<T: num::Num + Copy>(cum_sum: &[T], a: usize, b: usize) -> T {
    if b > 0 {
        range_sum_inclusive(cum_sum, a, b - 1)
    } else {
        T::zero()
    }
}

pub fn egcd<T>(a: T, b: T) -> (T, T, T)
where
    T: std::cmp::PartialEq + num::Signed + Copy,
{
    if a == T::zero() {
        (b, T::zero(), T::one())
    } else {
        let (gcd, x, y) = egcd(b % a, a);
        (gcd, y - (b / a) * x, x)
    }
}

pub fn mod_inv<T>(x: T, n: T) -> Option<T>
where
    T: std::cmp::PartialEq + num::Signed + Copy,
{
    let (g, x, _) = egcd(x, n);
    if g == T::one() {
        Some((x % n + n) % n)
    } else {
        None
    }
}

pub fn chinese_remainder<'a, T>(residues: &[T], modulii: &'a [T]) -> Option<T>
where
    T: 'a + std::cmp::PartialEq + num::Signed + Copy + std::iter::Product<&'a T>,
{
    let prod = modulii.iter().product::<T>();

    let mut sum = T::zero();

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum = sum + residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

pub struct GridIteratorHelper {
    extents: (Point, Point),
    curr: Option<Point>,
}

impl Iterator for GridIteratorHelper {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some([x, y]) = self.curr {
            let c = if x < self.extents.1[0] {
                Some([x + 1, y])
            } else if y < self.extents.1[1] {
                Some([self.extents.0[0], y + 1])
            } else {
                None
            };
            let curr = self.curr;
            self.curr = c;
            curr
        } else {
            None
        }
    }
}

pub trait Grid<T>
where
    T: PartialEq + Copy,
{
    fn get_value(&self, pos: Point) -> Option<T>;
    fn set_value(&mut self, pos: Point, value: T);
    fn extents(&self) -> (Point, Point);
    fn points(&self) -> GridIteratorHelper {
        let extents = self.extents();
        GridIteratorHelper {
            extents,
            curr: Some(extents.0),
        }
    }
    fn flip_horizontal(&mut self);
    fn flip_vertical(&mut self);
    fn transpose(&mut self);
    fn rotate_90_cw(&mut self) {
        self.transpose();
        self.flip_horizontal();
    }
    fn rotate_180_cw(&mut self) {
        self.flip_vertical();
        self.flip_horizontal();
    }
    fn rotate_270_cw(&mut self) {
        self.transpose();
        self.flip_vertical();
    }
    fn fill(&mut self, pos: Point, value: T) {
        let ([min_x, min_y], [max_x, max_y]) = self.extents();
        if let Some(old) = self.get_value(pos) {
            if value != old {
                let mut todo = vec![];
                todo.push(pos);
                while let Some(p) = todo.pop() {
                    if let Some(curr) = self.get_value(p) {
                        if curr == old {
                            self.set_value(p, value);
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
            }
        }
    }
    fn line(&mut self, a: Point, b: Point, value: T) {
        let line = plot_line(a, b);
        for p in line {
            self.set_value(p, value);
        }
    }
    fn blit(&mut self, pos: Point, g: &dyn Grid<T>) {
        let (start, end) = g.extents();
        self.blit_rect(pos, g, start, end);
    }
    fn blit_rect(&mut self, pos: Point, g: &dyn Grid<T>, start: Point, end: Point) {
        let ([min_x, min_y], [max_x, max_y]) = g.extents();
        let min_xx = min_x.max(start[0]);
        let min_yy = min_y.max(start[1]);
        let max_xx = max_x.min(end[0]);
        let max_yy = max_y.min(end[1]);
        for (dy, yy) in (min_yy..=max_yy).enumerate() {
            for (dx, xx) in (min_xx..=max_xx).enumerate() {
                let [xxx, yyy] = point_add(pos, [dx as i64, dy as i64]);
                if let Some(v) = g.get_value([xx, yy]) {
                    self.set_value([xxx, yyy], v);
                }
            }
        }
    }
    fn text_ch(&mut self, c: char, pos: Point, value: T) {
        let mut tmp = [0u8; 4];
        let s = c.encode_utf8(&mut tmp);
        self.text(s, pos, value);
    }

    fn text(&mut self, a: &str, mut pos: Point, value: T) {
        let (w, h) = SMALLFONT.glyph_size();
        for c in a.chars() {
            if let Some(glyph) = SMALLFONT.glyph(c as u32) {
                for y in 0..h {
                    for x in 0..w {
                        let byte = y + (x / 8);
                        let bit = 7 - (x % 8);
                        if (glyph[byte as usize] & (1 << bit)) == 0 {
                            // Do nothing
                        } else {
                            self.set_value(point_add(pos, [x as i64, y as i64]), value);
                        }
                    }
                }
            }
            pos[0] += w as i64;
        }
    }
}

pub trait GridConvert<G, T, U>
where
    Self: Grid<T> + Clone + Sized,
    T: PartialEq + Copy,
    U: PartialEq + Copy,
{
    fn blit_rect_convert(
        &mut self,
        pos: Point,
        g: &dyn Grid<U>,
        start: Point,
        end: Point,
        convert: fn(U) -> T,
    );
}

impl<G, T, U> GridConvert<G, T, U> for G
where
    G: Grid<T> + Clone + Sized,
    T: PartialEq + Copy,
    U: PartialEq + Copy,
{
    // pos is position to blit to, start/end is the rect to copy from grid
    fn blit_rect_convert(
        &mut self,
        pos: Point,
        g: &dyn Grid<U>,
        start: Point,
        end: Point,
        convert: fn(U) -> T,
    ) {
        let ([min_x, min_y], [max_x, max_y]) = g.extents();
        let min_xx = min_x.max(start[0]);
        let min_yy = min_y.max(start[1]);
        let max_xx = max_x.min(end[0]);
        let max_yy = max_y.min(end[1]);
        for (dy, yy) in (min_yy..=max_yy).enumerate() {
            for (dx, xx) in (min_xx..=max_xx).enumerate() {
                let [xxx, yyy] = point_add(pos, [dx as i64, dy as i64]);
                if let Some(v) = g.get_value([xx, yy]) {
                    self.set_value([xxx, yyy], convert(v));
                }
            }
        }
    }
}

pub struct GridFlipIteratorHelper<G, T>
where
    G: Grid<T> + Clone,
    T: PartialEq + Copy,
{
    rot: usize,
    flip: bool,
    phantom: PhantomData<T>,
    grid: G,
}

impl<G, T> Iterator for GridFlipIteratorHelper<G, T>
where
    G: Grid<T> + Clone,
    T: PartialEq + Copy,
{
    type Item = G;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rot > 3 {
            return None;
        }
        let mut g = self.grid.clone();
        match self.rot {
            0 => {}
            1 => g.rotate_90_cw(),
            2 => g.rotate_180_cw(),
            3 => g.rotate_270_cw(),
            _ => panic!(),
        }
        if self.flip {
            g.flip_horizontal();
        }
        if !self.flip {
            self.flip = true;
        } else {
            self.flip = false;
            self.rot += 1;
        }
        Some(g)
    }
}

pub trait GridTranspose<G, T>
where
    Self: Grid<T> + Clone + Sized,
    T: PartialEq + Copy,
{
    // Note: consumes self
    fn into_transpositions(self) -> GridFlipIteratorHelper<Self, T>;
    fn transpositions(&self) -> GridFlipIteratorHelper<Self, T>;
}

impl<G, T> GridTranspose<G, T> for G
where
    G: Grid<T> + Clone + Sized,
    T: PartialEq + Copy,
{
    fn into_transpositions(self) -> GridFlipIteratorHelper<Self, T> {
        let grid = self;
        GridFlipIteratorHelper {
            rot: 0,
            flip: false,
            phantom: PhantomData,
            grid,
        }
    }
    fn transpositions(&self) -> GridFlipIteratorHelper<Self, T> {
        let grid = self.clone();
        GridFlipIteratorHelper {
            rot: 0,
            flip: false,
            phantom: PhantomData,
            grid,
        }
    }
}

impl<S: ::std::hash::BuildHasher, T> Grid<T> for HashMap<Point, T, S>
where
    T: Clone + Copy + Default + PartialEq,
{
    fn get_value(&self, pos: Point) -> Option<T> {
        self.get(&pos).copied()
    }
    fn set_value(&mut self, pos: Point, value: T) {
        *self.entry(pos).or_insert(value) = value;
    }
    fn extents(&self) -> (Point, Point) {
        let min_x = self.iter().map(|(p, _v)| p[0]).min().unwrap_or(0);
        let min_y = self.iter().map(|(p, _v)| p[1]).min().unwrap_or(0);
        let max_x = self.iter().map(|(p, _v)| p[0]).max().unwrap_or(0);
        let max_y = self.iter().map(|(p, _v)| p[1]).max().unwrap_or(0);
        ([min_x, min_y], [max_x, max_y])
    }
    fn flip_horizontal(&mut self) {
        let ([min_x, _min_y], [max_x, _max_y]) = self.extents();
        let mut new_grid = HashMap::new();
        for ([x, y], v) in self.iter() {
            let new_x = max_x - (x - min_x);
            new_grid.insert([new_x, *y], *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn flip_vertical(&mut self) {
        let ([_min_x, min_y], [_max_x, max_y]) = self.extents();
        let mut new_grid = HashMap::new();
        for ([x, y], v) in self.iter() {
            let new_y = max_y - (y - min_y);
            new_grid.insert([*x, new_y], *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn transpose(&mut self) {
        let mut new_grid = HashMap::new();
        for ([x, y], v) in self.iter() {
            new_grid.insert([*y, *x], *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
}

impl<T> Grid<T> for BTreeMap<Point, T>
where
    T: Clone + Copy + Default + PartialEq,
{
    fn get_value(&self, pos: Point) -> Option<T> {
        self.get(&pos).copied()
    }
    fn set_value(&mut self, pos: Point, value: T) {
        *self.entry(pos).or_insert(value) = value;
    }
    fn extents(&self) -> (Point, Point) {
        let min_x = self.iter().map(|(p, _v)| p[0]).min().unwrap_or(0);
        let min_y = self.iter().map(|(p, _v)| p[1]).min().unwrap_or(0);
        let max_x = self.iter().map(|(p, _v)| p[0]).max().unwrap_or(0);
        let max_y = self.iter().map(|(p, _v)| p[1]).max().unwrap_or(0);
        ([min_x, min_y], [max_x, max_y])
    }
    fn flip_horizontal(&mut self) {
        let ([min_x, _min_y], [max_x, _max_y]) = self.extents();
        let mut new_grid = HashMap::new();
        for ([x, y], v) in self.iter() {
            let new_x = max_x - (x - min_x);
            new_grid.insert([new_x, *y], *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn flip_vertical(&mut self) {
        let ([_min_x, min_y], [_max_x, max_y]) = self.extents();
        let mut new_grid = HashMap::new();
        for ([x, y], v) in self.iter() {
            let new_y = max_y - (y - min_y);
            new_grid.insert([*x, new_y], *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn transpose(&mut self) {
        let mut new_grid = HashMap::new();
        for ([x, y], v) in self.iter() {
            new_grid.insert([*y, *x], *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
}

impl<T> Grid<T> for Vec<Vec<T>>
where
    T: Clone + Copy + Default + PartialEq,
{
    fn get_value(&self, pos: Point) -> Option<T> {
        let [x, y] = pos;
        if let Some(line) = self.get(y as usize) {
            if let Some(c) = line.get(x as usize) {
                return Some(*c);
            }
        }
        None
    }
    fn set_value(&mut self, pos: Point, value: T) {
        let [x, y] = pos;
        if let Some(line) = self.get_mut(y as usize) {
            if let Some(c) = line.get_mut(x as usize) {
                *c = value;
            }
        }
    }
    fn extents(&self) -> (Point, Point) {
        if !self.is_empty() && !self[0].is_empty() {
            return (
                [0, 0],
                [(self[0].len() - 1) as i64, (self.len() - 1) as i64],
            );
        }
        ([0, 0], [0, 0])
    }
    fn flip_horizontal(&mut self) {
        let ([min_x, min_y], [max_x, max_y]) = self.extents();
        let mut new_vec = self.clone();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let v = self[y as usize][x as usize];
                let new_x = max_x - (x - min_x);
                new_vec[y as usize][new_x as usize] = v;
            }
        }
        *self = new_vec;
    }
    fn flip_vertical(&mut self) {
        let ([min_x, min_y], [max_x, max_y]) = self.extents();
        let mut new_vec = self.clone();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let v = self[y as usize][x as usize];
                let new_y = max_y - (y - min_y);
                new_vec[new_y as usize][x as usize] = v;
            }
        }
        *self = new_vec;
    }
    fn transpose(&mut self) {
        let ([min_x, min_y], [max_x, max_y]) = self.extents();
        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;
        // Make a vec with the transposed dimensions
        let mut new_vec = Vec::with_capacity(width);
        for _ in min_x..=max_x {
            let mut row = Vec::with_capacity(height);
            row.resize_with(height, Default::default);
            new_vec.push(row);
        }
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let v = self[y as usize][x as usize];
                new_vec[x as usize][y as usize] = v;
            }
        }
        *self = new_vec;
    }
}

impl Grid<[u8; 3]> for image::RgbImage {
    fn get_value(&self, pos: Point) -> Option<[u8; 3]> {
        let x = pos[0] as u32;
        let y = pos[1] as u32;
        // This is [min, max[
        let (min_x, min_y, max_x, max_y) = self.bounds();
        if x >= min_x && x < max_x && y >= min_y && y < max_y {
            let Rgb(rgb) = self.get_pixel(x, y);
            Some(*rgb)
        } else {
            None
        }
    }
    fn set_value(&mut self, pos: Point, value: [u8; 3]) {
        let x = pos[0] as u32;
        let y = pos[1] as u32;
        // This is [min, max[
        let (min_x, min_y, max_x, max_y) = self.bounds();
        if x >= min_x && x < max_x && y >= min_y && y < max_y {
            self.put_pixel(x, y, Rgb(value));
        }
    }
    fn extents(&self) -> (Point, Point) {
        // This is [min, max[
        let (min_x, min_y, max_x, max_y) = self.bounds();
        (
            [min_x as i64, min_y as i64],
            [max_x as i64 - 1, max_y as i64 - 1],
        )
    }
    fn flip_horizontal(&mut self) {
        image::imageops::flip_horizontal_in_place(self);
    }
    fn flip_vertical(&mut self) {
        image::imageops::flip_vertical_in_place(self);
    }
    fn rotate_90_cw(&mut self) {
        let new = image::imageops::rotate90(self);
        *self = new;
    }
    fn rotate_180_cw(&mut self) {
        image::imageops::rotate180_in_place(self);
    }
    fn rotate_270_cw(&mut self) {
        let new = image::imageops::rotate270(self);
        *self = new;
    }
    fn transpose(&mut self) {
        self.rotate_270_cw();
        self.flip_horizontal()
    }
}

pub trait GridDrawer<G, T>
where
    G: Grid<T>,
    T: PartialEq + Copy,
{
    fn draw(&mut self, area: &G);
}

pub struct NopGridDrawer {}

impl<G, T> GridDrawer<G, T> for NopGridDrawer
where
    G: Grid<T>,
    T: PartialEq + Copy,
{
    fn draw(&mut self, _: &G) {}
}

pub struct PrintGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    to_ch: F,
    phantom: PhantomData<T>,
}

impl<F, T> PrintGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    pub fn new(to_ch: F) -> PrintGridDrawer<F, T> {
        PrintGridDrawer {
            to_ch,
            phantom: PhantomData,
        }
    }

    fn to_char(&self, col: T) -> char {
        (self.to_ch)(col)
    }
}

impl<F, G, T> GridDrawer<G, T> for PrintGridDrawer<F, T>
where
    F: Fn(T) -> char,
    G: Grid<T>,
    T: PartialEq + Copy,
{
    fn draw(&mut self, area: &G) {
        let ([min_x, min_y], [max_x, max_y]) = area.extents();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let ch = if let Some(x) = area.get_value([x, y]) {
                    self.to_char(x)
                } else {
                    '.'
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

pub struct CursesGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    window: pancurses::Window,
    to_ch: F,
    phantom: PhantomData<T>,
}

impl<F, T> CursesGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    pub fn new(to_ch: F) -> CursesGridDrawer<F, T> {
        let window = pancurses::initscr();
        pancurses::nl();
        pancurses::noecho();
        pancurses::curs_set(0);
        window.keypad(true);
        window.scrollok(true);
        window.nodelay(true);
        CursesGridDrawer {
            window,
            to_ch,
            phantom: PhantomData,
        }
    }

    fn to_char(&self, col: T) -> char {
        (self.to_ch)(col)
    }
}

impl<F, T> Drop for CursesGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

impl<F, G, T> GridDrawer<G, T> for CursesGridDrawer<F, T>
where
    F: Fn(T) -> char,
    G: Grid<T>,
    T: PartialEq + Copy,
{
    fn draw(&mut self, area: &G) {
        self.window.clear();
        let ([min_x, _], [min_y, _]) = area.extents();
        for p in area.points() {
            let ch = if let Some(x) = area.get_value(p) {
                self.to_char(x)
            } else {
                ' '
            };
            self.window
                .mvaddch((p[1] - min_y) as i32, (p[0] - min_x) as i32, ch);
        }
        if let Some(pancurses::Input::Character(c)) = self.window.getch() {
            if c == 'q' {
                pancurses::endwin();
                std::process::exit(0);
            }
        }
        self.window.refresh();
    }
}

pub struct BitmapSpriteGridDrawer<F, G, T>
where
    F: Fn(T) -> Vec<[u8; 3]>,
    G: Grid<T>,
    T: PartialEq + Copy,
{
    sprite_dimension: (i64, i64),
    to_sprite: F,
    basename: String,
    frame: usize,
    rect: Option<(Point, Point)>,
    bg: [u8; 3],
    unset: Option<T>,
    image: Option<RgbImage>,
    phantom: PhantomData<T>,
    phantom_g: PhantomData<G>,
}

// These can be converted to movies with:
// ffmpeg -i "basename_%06d.png" -filter_complex "[0:v] palettegen" basename_palette.png
// ffmpeg -framerate 25 -i "basename_%06d.png" -i basename_palette.png -filter_complex "[0:v][1:v] paletteuse" basename.gif
// You can change the start number with the -start_number input option.
impl<F, G, T> BitmapSpriteGridDrawer<F, G, T>
where
    F: Fn(T) -> Vec<[u8; 3]>,
    G: Grid<T>,
    T: PartialEq + Copy,
{
    pub fn new(
        sprite_dimension: (i64, i64),
        to_sprite: F,
        basename: &str,
    ) -> BitmapSpriteGridDrawer<F, G, T> {
        // TODO: error handling
        let path = Path::new(basename);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("could not create folder");
        }
        BitmapSpriteGridDrawer {
            sprite_dimension,
            to_sprite,
            frame: 0,
            basename: basename.into(),
            rect: None,
            bg: [255, 255, 255],
            unset: None,
            image: None,
            phantom: PhantomData,
            phantom_g: PhantomData,
        }
    }

    pub fn set_rect(&mut self, r: (Point, Point)) {
        self.rect = Some(r);
    }

    pub fn set_bg(&mut self, bg: [u8; 3]) {
        self.bg = bg;
    }

    pub fn set_unset(&mut self, unset: T) {
        self.unset = Some(unset);
    }

    pub fn save_image(&self) {
        let path = Path::new(&self.basename);
        let filename = if let Some(parent) = path.parent() {
            parent.join(&format!(
                "{}_{:06}.png",
                path.file_name().unwrap().to_str().unwrap(),
                self.frame
            ))
        } else {
            PathBuf::from(&format!("{}_{}.png", self.basename, self.frame))
        };
        if let Some(image) = &self.image {
            image.save(filename).unwrap();
        }
    }

    pub fn draw_grid(&mut self, area: &G) {
        self.frame += 1;
        let ([mut min_x, mut min_y], [mut max_x, mut max_y]) = area.extents();
        // "clip" to rect
        if let Some(([cmin_x, cmin_y], [cmax_x, cmax_y])) = self.rect {
            min_x = cmin_x;
            min_y = cmin_y;
            max_x = cmax_x;
            max_y = cmax_y;
        }
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let pixelw = width * self.sprite_dimension.0;
        let pixelh = height * self.sprite_dimension.1;
        let mut buffer = vec![255; (3 * pixelw * pixelh) as usize];
        buffer.chunks_mut(3).for_each(|c| {
            c[0] = self.bg[0];
            c[1] = self.bg[1];
            c[2] = self.bg[2]
        });
        let mut image = RgbImage::from_raw(pixelw as u32, pixelh as u32, buffer).unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let val = if let Some(v) = area.get_value([x, y]) {
                    Some(v)
                } else {
                    self.unset
                };
                if let Some(value) = val {
                    let sprite = self.to_sprite(value);
                    let mut yy = (y - min_y) * self.sprite_dimension.1;
                    let mut xx = (x - min_x) * self.sprite_dimension.0;
                    let xxx = xx;
                    for col in &sprite {
                        let rgb = Rgb(*col);
                        image.put_pixel(xx as u32, yy as u32, rgb);
                        xx += 1;
                        if xx - xxx >= self.sprite_dimension.0 {
                            xx = (x - min_x) * self.sprite_dimension.0;
                            yy += 1
                        }
                    }
                }
            }
        }
        self.image = Some(image);
    }

    pub fn put_pixel(&mut self, p: Point, rgb: [u8; 3]) {
        if let Some(ref mut image) = self.image {
            let x = p[0] as u32;
            let y = p[1] as u32;
            if x < image.width() && y < image.height() {
                image.put_pixel(x, y, Rgb(rgb));
            }
        }
    }

    fn to_sprite(&self, value: T) -> Vec<[u8; 3]> {
        (self.to_sprite)(value)
    }
}

impl<F, G, T> GridDrawer<G, T> for BitmapSpriteGridDrawer<F, G, T>
where
    F: Fn(T) -> Vec<[u8; 3]>,
    G: Grid<T>,
    T: PartialEq + Copy,
{
    fn draw(&mut self, area: &G) {
        self.draw_grid(area);
        self.save_image();
    }
}

pub struct BitmapGridDrawer<F, G, T>
where
    F: Fn(T) -> [u8; 3],
    G: Grid<T>,
    T: PartialEq + Copy,
{
    to_color: F,
    basename: String,
    frame: usize,
    rect: Option<(Point, Point)>,
    bg: [u8; 3],
    image: Option<RgbImage>,
    phantom: PhantomData<T>,
    phantom_g: PhantomData<G>,
}

// These can be converted to movies with:
// ffmpeg -i "basename_%06d.png" -filter_complex "[0:v] palettegen" basename_palette.png
// ffmpeg -framerate 25 -i "basename_%06d.png" -i basename_palette.png -filter_complex "[0:v][1:v] paletteuse" basename.gif
// You can change the start number with the -start_number input option.
impl<F, G, T> BitmapGridDrawer<F, G, T>
where
    F: Fn(T) -> [u8; 3],
    G: Grid<T>,
    T: PartialEq + Copy,
{
    pub fn new(to_color: F, basename: &str) -> BitmapGridDrawer<F, G, T> {
        // TODO: error handling
        let path = Path::new(basename);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("could not create folder");
        }
        BitmapGridDrawer {
            to_color,
            frame: 0,
            basename: basename.into(),
            rect: None,
            bg: [255, 255, 255],
            image: None,
            phantom: PhantomData,
            phantom_g: PhantomData,
        }
    }

    pub fn set_rect(&mut self, r: (Point, Point)) {
        self.rect = Some(r);
    }

    pub fn set_bg(&mut self, bg: [u8; 3]) {
        self.bg = bg;
    }

    pub fn save_image(&self) {
        let path = Path::new(&self.basename);
        let filename = if let Some(parent) = path.parent() {
            parent.join(&format!(
                "{}_{:06}.png",
                path.file_name().unwrap().to_str().unwrap(),
                self.frame
            ))
        } else {
            PathBuf::from(&format!("{}_{}.png", self.basename, self.frame))
        };
        if let Some(image) = &self.image {
            image.save(filename).unwrap();
        }
    }

    pub fn draw_grid(&mut self, area: &G) {
        self.frame += 1;
        let ([mut min_x, mut min_y], [mut max_x, mut max_y]) = area.extents();
        // "clip" to rect
        if let Some(([cmin_x, cmin_y], [cmax_x, cmax_y])) = self.rect {
            min_x = cmin_x;
            min_y = cmin_y;
            max_x = cmax_x;
            max_y = cmax_y;
        }
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        // Default bg is white
        let mut buffer = vec![255; (3 * width * height) as usize];
        buffer.chunks_mut(3).for_each(|c| {
            c[0] = self.bg[0];
            c[1] = self.bg[1];
            c[2] = self.bg[2]
        });
        let mut image = RgbImage::from_raw(width as u32, height as u32, buffer).unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(value) = area.get_value([x, y]) {
                    let color = self.to_color(value);
                    let yy = y - min_y;
                    let xx = x - min_x;
                    let rgb = Rgb(color);
                    image.put_pixel(xx as u32, yy as u32, rgb);
                }
            }
        }
        self.image = Some(image);
    }

    pub fn put_pixel(&mut self, p: Point, rgb: [u8; 3]) {
        if let Some(ref mut image) = self.image {
            let x = p[0] as u32;
            let y = p[1] as u32;
            if x < image.width() && y < image.height() {
                image.put_pixel(x, y, Rgb(rgb));
            }
        }
    }

    fn to_color(&self, value: T) -> [u8; 3] {
        (self.to_color)(value)
    }
}

impl<F, G, T> GridDrawer<G, T> for BitmapGridDrawer<F, G, T>
where
    F: Fn(T) -> [u8; 3],
    G: Grid<T>,
    T: PartialEq + Copy,
{
    fn draw(&mut self, area: &G) {
        self.draw_grid(area);
        self.save_image();
    }
}

// Bresenham
// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
pub fn plot_line(a: Point, b: Point) -> Vec<Point> {
    let [mut x0, mut y0] = a;
    let [x1, y1] = b;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy; /* error value e_xy */
    let mut out = vec![];
    loop {
        out.push([x0, y0]);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        /* e_xy+e_x > 0 */
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        /* e_xy+e_y < 0 */
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
    out
}

// Iterates in axial coordinates
pub struct HexGridIteratorHelper {
    extents: (Point, Point),
    curr: Option<Point>,
}

impl Iterator for HexGridIteratorHelper {
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some([x, y]) = self.curr {
            let c = if x < self.extents.1[0] {
                Some([x + 1, y])
            } else if y < self.extents.1[1] {
                Some([self.extents.0[0], y + 1])
            } else {
                None
            };
            let curr = self.curr;
            self.curr = c;
            curr.map(axial_to_cube)
        } else {
            None
        }
    }
}

pub trait HexGrid<T>
where
    T: PartialEq + Copy,
{
    fn get_value(&self, pos: Vec3) -> Option<T>;
    fn set_value(&mut self, pos: Vec3, value: T);
    // Extents in axial coordinates
    fn axial_extents(&self) -> (Point, Point);
    // Extents in oddr coordinates
    fn oddr_extents(&self) -> (Point, Point);
    fn points(&self) -> HexGridIteratorHelper {
        let extents = self.axial_extents();
        HexGridIteratorHelper {
            extents,
            curr: Some(extents.0),
        }
    }
    fn flip_horizontal(&mut self);
    fn flip_vertical(&mut self);
    fn flip_x(&mut self);
    fn flip_y(&mut self);
    fn flip_z(&mut self);
    fn rotate_60_cw(&mut self);
    fn rotate_120_cw(&mut self);
    fn rotate_180_cw(&mut self);
    fn rotate_240_cw(&mut self);
    fn rotate_300_cw(&mut self);
}

impl<S: ::std::hash::BuildHasher, T> HexGrid<T> for HashMap<Vec3, T, S>
where
    T: Clone + Copy + Default + PartialEq,
{
    fn get_value(&self, pos: Vec3) -> Option<T> {
        self.get(&pos).copied()
    }
    fn set_value(&mut self, pos: Vec3, value: T) {
        *self.entry(pos).or_insert(value) = value;
    }
    fn axial_extents(&self) -> (Point, Point) {
        let min_q = self
            .iter()
            .map(|(p, _v)| cube_to_axial(*p)[0])
            .min()
            .unwrap_or(0);
        let min_r = self
            .iter()
            .map(|(p, _v)| cube_to_axial(*p)[1])
            .min()
            .unwrap_or(0);
        let max_q = self
            .iter()
            .map(|(p, _v)| cube_to_axial(*p)[0])
            .max()
            .unwrap_or(0);
        let max_r = self
            .iter()
            .map(|(p, _v)| cube_to_axial(*p)[1])
            .max()
            .unwrap_or(0);
        ([min_q, min_r], [max_q, max_r])
    }
    fn oddr_extents(&self) -> (Point, Point) {
        let min_x = self
            .iter()
            .map(|(p, _v)| cube_to_oddr(*p)[0])
            .min()
            .unwrap_or(0);
        let min_y = self
            .iter()
            .map(|(p, _v)| cube_to_oddr(*p)[1])
            .min()
            .unwrap_or(0);
        let max_x = self
            .iter()
            .map(|(p, _v)| cube_to_oddr(*p)[0])
            .max()
            .unwrap_or(0);
        let max_y = self
            .iter()
            .map(|(p, _v)| cube_to_oddr(*p)[1])
            .max()
            .unwrap_or(0);
        ([min_x, min_y], [max_x, max_y])
    }
    fn flip_horizontal(&mut self) {
        let ([min_x, _min_y], [max_x, _max_y]) = self.oddr_extents();
        let mut new_grid = HashMap::new();
        for (p, v) in self.iter() {
            let p = cube_to_oddr(*p);
            let new_x = max_x - (p[0] - min_x);
            let p = oddr_to_cube([new_x, p[1]]);
            new_grid.insert(p, *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn flip_vertical(&mut self) {
        let ([_min_x, min_y], [_max_x, max_y]) = self.oddr_extents();
        let mut new_grid = HashMap::new();
        for (p, v) in self.iter() {
            let p = cube_to_oddr(*p);
            let new_y = max_y - (p[1] - min_y);
            let p = oddr_to_cube([p[0], new_y]);
            new_grid.insert(p, *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn flip_x(&mut self) {
        let ([min_x, min_y], [max_x, max_y]) = self.oddr_extents();
        let mid_x = (max_x - min_x + 1) / 2;
        let mid_y = (max_y - min_y + 1) / 2;
        let pivot = oddr_to_cube([mid_x, mid_y]);
        let mut new_grid = HashMap::new();
        for ([x, y, z], v) in self.iter() {
            let p = vec_sub([*x, *y, *z], pivot);
            let p = vec_add([p[0], p[2], p[1]], pivot);
            new_grid.insert(p, *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn flip_y(&mut self) {
        let mut new_grid = HashMap::new();
        let ([min_x, min_y], [max_x, max_y]) = self.oddr_extents();
        let mid_x = (max_x - min_x + 1) / 2;
        let mid_y = (max_y - min_y + 1) / 2;
        let pivot = oddr_to_cube([mid_x, mid_y]);
        for ([x, y, z], v) in self.iter() {
            let p = vec_sub([*x, *y, *z], pivot);
            let p = vec_add([p[2], p[1], p[0]], pivot);
            new_grid.insert(p, *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn flip_z(&mut self) {
        let mut new_grid = HashMap::new();
        let ([min_x, min_y], [max_x, max_y]) = self.oddr_extents();
        let mid_x = (max_x - min_x + 1) / 2;
        let mid_y = (max_y - min_y + 1) / 2;
        let pivot = oddr_to_cube([mid_x, mid_y]);
        for ([x, y, z], v) in self.iter() {
            let p = vec_sub([*x, *y, *z], pivot);
            let p = vec_add([p[1], p[0], p[2]], pivot);
            new_grid.insert(p, *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn rotate_60_cw(&mut self) {
        let mut new_grid = HashMap::new();
        let ([min_x, min_y], [max_x, max_y]) = self.oddr_extents();
        let mid_x = (max_x - min_x + 1) / 2;
        let mid_y = (max_y - min_y + 1) / 2;
        let pivot = oddr_to_cube([mid_x, mid_y]);
        for ([x, y, z], v) in self.iter() {
            let p = vec_sub([*x, *y, *z], pivot);
            let p = vec_add([-p[2], -p[0], -p[1]], pivot);
            new_grid.insert(p, *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn rotate_120_cw(&mut self) {
        let mut new_grid = HashMap::new();
        let ([min_x, min_y], [max_x, max_y]) = self.oddr_extents();
        let mid_x = (max_x - min_x + 1) / 2;
        let mid_y = (max_y - min_y + 1) / 2;
        let pivot = oddr_to_cube([mid_x, mid_y]);
        for ([x, y, z], v) in self.iter() {
            let p = vec_sub([*x, *y, *z], pivot);
            let p = vec_add([p[1], p[2], p[0]], pivot);
            new_grid.insert(p, *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn rotate_180_cw(&mut self) {
        let mut new_grid = HashMap::new();
        let ([min_x, min_y], [max_x, max_y]) = self.oddr_extents();
        let mid_x = (max_x - min_x + 1) / 2;
        let mid_y = (max_y - min_y + 1) / 2;
        let pivot = oddr_to_cube([mid_x, mid_y]);
        for ([x, y, z], v) in self.iter() {
            let p = vec_sub([*x, *y, *z], pivot);
            let p = vec_add([-p[0], -p[1], -p[2]], pivot);
            new_grid.insert(p, *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn rotate_240_cw(&mut self) {
        let mut new_grid = HashMap::new();
        let ([min_x, min_y], [max_x, max_y]) = self.oddr_extents();
        let mid_x = (max_x - min_x + 1) / 2;
        let mid_y = (max_y - min_y + 1) / 2;
        let pivot = oddr_to_cube([mid_x, mid_y]);
        for ([x, y, z], v) in self.iter() {
            let p = vec_sub([*x, *y, *z], pivot);
            let p = vec_add([p[2], p[0], p[1]], pivot);
            new_grid.insert(p, *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
    fn rotate_300_cw(&mut self) {
        let mut new_grid = HashMap::new();
        let ([min_x, min_y], [max_x, max_y]) = self.oddr_extents();
        let mid_x = (max_x - min_x + 1) / 2;
        let mid_y = (max_y - min_y + 1) / 2;
        let pivot = oddr_to_cube([mid_x, mid_y]);
        for ([x, y, z], v) in self.iter() {
            let p = vec_sub([*x, *y, *z], pivot);
            let p = vec_add([-p[1], -p[2], -p[0]], pivot);
            new_grid.insert(p, *v);
        }
        self.clear();
        for (k, v) in new_grid {
            self.insert(k, v);
        }
    }
}

pub fn axial_to_cube(axial: Point) -> Vec3 {
    let x = axial[0];
    let z = axial[1];
    let y = -x - z;
    [x, y, z]
}

pub fn cube_to_axial(cube: Vec3) -> Point {
    let q = cube[0];
    let r = cube[2];
    [q, r]
}

pub fn cube_to_oddr(cube: Vec3) -> Point {
    let col = cube[0] + (cube[2] - (cube[2].rem_euclid(2))) / 2;
    let row = cube[2];
    [col, row]
}

pub fn oddr_to_cube(oddr: Point) -> Vec3 {
    let x = oddr[0] - (oddr[1] - (oddr[1].rem_euclid(2))) / 2;
    let z = oddr[1];
    let y = -x - z;
    [x, y, z]
}

pub trait HexGridDrawer<G, T>
where
    G: HexGrid<T>,
    T: PartialEq + Copy + Default,
{
    fn draw(&mut self, area: &G);
    // Convert to offset coordinate based sparse grid for printing
    fn convert(&self, g: &G) -> HashMap<Point, T> {
        let mut gg: HashMap<Point, T> = HashMap::new();
        // Convert coords
        for p in g.points() {
            if let Some(v) = g.get_value(p) {
                gg.set_value(cube_to_oddr(p), v);
            }
        }
        gg
    }
}

pub struct NopHexGridDrawer {}

impl<G, T> HexGridDrawer<G, T> for NopHexGridDrawer
where
    G: HexGrid<T>,
    T: PartialEq + Copy + Default,
{
    fn draw(&mut self, _: &G) {}
}

pub struct PrintHexGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    to_ch: F,
    phantom: PhantomData<T>,
}

impl<F, T> PrintHexGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    pub fn new(to_ch: F) -> PrintHexGridDrawer<F, T> {
        PrintHexGridDrawer {
            to_ch,
            phantom: PhantomData,
        }
    }

    fn to_char(&self, col: T) -> char {
        let ch = (self.to_ch)(col);
        if ch == char::default() {
            ' '
        } else {
            ch
        }
    }
}

impl<F, G, T> HexGridDrawer<G, T> for PrintHexGridDrawer<F, T>
where
    F: Fn(T) -> char,
    G: HexGrid<T>,
    T: PartialEq + Copy + Default,
{
    fn draw(&mut self, area: &G) {
        let g = self.convert(area);
        let ([min_x, min_y], [max_x, max_y]) = g.extents();
        if min_y.rem_euclid(2) == 0 {
            print!(" ");
            for _ in min_x..=max_x {
                print!("/ \\ ");
            }
            println!("/");
        }
        for y in min_y..=max_y {
            if y.rem_euclid(2) != 0 {
                print!(" ");
                for _ in min_x..=max_x {
                    print!("\\ / ");
                }
                print!("\\");
                println!();
            }
            if y.rem_euclid(2) != 0 {
                print!("  ");
            }
            for x in min_x..=max_x {
                let p = [x as i64, y as i64];
                let d = T::default();
                let c = g.get(&p).unwrap_or(&d);
                print!("| {} ", self.to_char(*c));
            }
            print!("|");
            if y.rem_euclid(2) != 0 {
                println!();
                print!(" ");
                for _ in min_x..=max_x {
                    print!("/ \\ ");
                }
                print!("/");
            }
            println!();
        }
    }
}

pub struct CursesHexGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    window: pancurses::Window,
    to_ch: F,
    phantom: PhantomData<T>,
    w: i32,
    h: i32,
}

impl<F, T> CursesHexGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    pub fn new(to_ch: F) -> CursesHexGridDrawer<F, T> {
        let window = pancurses::initscr();
        pancurses::nl();
        pancurses::noecho();
        pancurses::curs_set(0);
        window.keypad(true);
        window.scrollok(true);
        window.nodelay(true);
        CursesHexGridDrawer {
            window,
            to_ch,
            phantom: PhantomData,
            w: 0,
            h: 0,
        }
    }

    fn to_char(&self, col: T) -> char {
        let ch = (self.to_ch)(col);
        if ch == char::default() {
            ' '
        } else {
            ch
        }
    }

    fn put(&self, x: i32, y: i32, c: char) {
        if x >= 0 && x < self.w && y >= 0 && y < self.h && c != ' ' {
            self.window.mvaddch(y, x, c);
        }
    }
    fn put_str(&self, x: i32, y: i32, s: &str) {
        for (ii, c) in s.chars().enumerate() {
            let i = ii as i32;
            self.put(x + i, y, c);
        }
    }
}

impl<F, T> Drop for CursesHexGridDrawer<F, T>
where
    F: Fn(T) -> char,
{
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

impl<F, G, T> HexGridDrawer<G, T> for CursesHexGridDrawer<F, T>
where
    F: Fn(T) -> char,
    G: HexGrid<T>,
    T: PartialEq + Copy + Default,
{
    fn draw(&mut self, area: &G) {
        self.window.clear();
        let grid = self.convert(area);
        let ([min_x, min_y], [max_x, max_y]) = grid.extents();
        self.w = self.window.get_max_x();
        self.h = self.window.get_max_y();
        let ww = (4 * (max_x - min_x + 1) + 3) as i32;
        let hh = (2 * (max_y - min_y + 1)) as i32;
        let xoffs = (self.w - ww) / 2;
        let yoffs = (self.h - hh) / 2;
        let mut xx = xoffs as i32;
        let mut yy = yoffs as i32;
        if min_y.rem_euclid(2) == 0 {
            self.put(xx, yy, ' ');
            xx += 1;
            for _ in min_x..=max_x {
                self.put_str(xx, yy, "/ \\ ");
                xx += 4;
            }
            self.put(xx, yy, '/');
        }
        xx = xoffs;
        yy += 1;
        for y in min_y..=max_y {
            if y.rem_euclid(2) != 0 {
                self.put(xx, yy, ' ');
                xx += 1;
                for _ in min_x..=max_x {
                    self.put_str(xx, yy, "\\ / ");
                    xx += 4;
                }
                self.put(xx, yy, '\\');
                xx = xoffs;
                yy += 1;
            }
            if y.rem_euclid(2) != 0 {
                self.put_str(xx, yy, "  ");
                xx += 2;
            }
            for x in min_x..=max_x {
                let p = [x as i64, y as i64];
                let d = T::default();
                let c = grid.get(&p).unwrap_or(&d);
                let s = format!("| {} ", self.to_char(*c));
                self.put_str(xx, yy, &s);
                xx += s.len() as i32;
            }
            self.put(xx, yy, '|');
            // xx += 1;
            if y.rem_euclid(2) != 0 {
                xx = xoffs;
                yy += 1;
                self.put(xx, yy, ' ');
                xx += 1;
                for _ in min_x..=max_x {
                    self.put_str(xx, yy, "/ \\ ");
                    xx += 4;
                }
                self.put(xx, yy, '/');
                // xx += 1;
            }
            xx = xoffs;
            yy += 1;
            if yy > self.h {
                break;
            }
        }
        if let Some(pancurses::Input::Character(c)) = self.window.getch() {
            if c == 'q' {
                pancurses::endwin();
                std::process::exit(0);
            }
        }
        self.window.refresh();
    }
}

pub struct BitmapHexGridDrawer<F, G, T>
where
    F: Fn(T) -> [u8; 3],
    G: HexGrid<T>,
    T: PartialEq + Copy,
{
    to_color: F,
    basename: String,
    frame: usize,
    image: Option<RgbImage>,
    hexagon: Vec<Vec<[u8; 3]>>,
    phantom: PhantomData<T>,
    phantom_g: PhantomData<G>,
}

// These can be converted to movies with:
// ffmpeg -i "basename_%06d.png" -filter_complex "[0:v] palettegen" basename_palette.png
// ffmpeg -framerate 25 -i "basename_%06d.png" -i basename_palette.png -filter_complex "[0:v][1:v] paletteuse" basename.gif
// You can change the start number with the -start_number input option.
impl<F, G, T> BitmapHexGridDrawer<F, G, T>
where
    F: Fn(T) -> [u8; 3],
    G: HexGrid<T>,
    T: PartialEq + Copy + Default,
{
    pub fn new(to_color: F, basename: &str) -> BitmapHexGridDrawer<F, G, T> {
        // TODO: error handling
        let path = Path::new(basename);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("could not create folder");
        }
        // Make a hexagon
        let mut hex = vec![vec![[255, 255, 255]; 7]; 10];
        hex.set_value([3, 0], [180, 180, 180]);
        hex.set_value([2, 1], [180, 180, 180]);
        hex.set_value([4, 1], [180, 180, 180]);
        hex.set_value([1, 1], [180, 180, 180]);
        hex.set_value([5, 1], [180, 180, 180]);
        hex.set_value([0, 2], [180, 180, 180]);
        hex.set_value([6, 2], [180, 180, 180]);
        hex.set_value([0, 3], [180, 180, 180]);
        hex.set_value([6, 3], [180, 180, 180]);
        hex.set_value([0, 4], [180, 180, 180]);
        hex.set_value([6, 4], [180, 180, 180]);
        hex.set_value([0, 5], [180, 180, 180]);
        hex.set_value([6, 5], [180, 180, 180]);
        hex.set_value([1, 6], [180, 180, 180]);
        hex.set_value([5, 6], [180, 180, 180]);
        hex.set_value([2, 6], [180, 180, 180]);
        hex.set_value([4, 6], [180, 180, 180]);
        hex.set_value([3, 7], [180, 180, 180]);
        BitmapHexGridDrawer {
            to_color,
            frame: 0,
            basename: basename.into(),
            image: None,
            hexagon: hex,
            phantom: PhantomData,
            phantom_g: PhantomData,
        }
    }

    pub fn save_image(&self) {
        let path = Path::new(&self.basename);
        let filename = if let Some(parent) = path.parent() {
            parent.join(&format!(
                "{}_{:06}.png",
                path.file_name().unwrap().to_str().unwrap(),
                self.frame
            ))
        } else {
            PathBuf::from(&format!("{}_{}.png", self.basename, self.frame))
        };
        if let Some(image) = &self.image {
            image.save(filename).unwrap();
        }
    }

    pub fn draw_grid(&mut self, area: &G) {
        self.frame += 1;
        let g = self.convert(area);
        let ([min_x, min_y], [max_x, max_y]) = g.extents();
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let pixelw = (width + 1) * 6;
        let pixelh = (height + 1) * 5;
        let buffer = vec![255; (3 * pixelw * pixelh) as usize];
        let mut image = RgbImage::from_raw(pixelw as u32, pixelh as u32, buffer).unwrap();
        for y in min_y..=max_y {
            let (xoffs, yoffs) = if y.rem_euclid(2) != 0 { (3, 0) } else { (0, 0) };
            for x in min_x..=max_x {
                image.blit(
                    [
                        ((x - min_x) * 6 + xoffs) as i64,
                        ((y - min_y) * 5 + yoffs) as i64,
                    ],
                    &self.hexagon,
                );
            }
        }
        // fill them in
        for y in min_y..=max_y {
            let (xoffs, yoffs) = if y.rem_euclid(2) != 0 { (3, 0) } else { (0, 0) };
            for x in min_x..=max_x {
                let p = [x as i64, y as i64];
                if let Some(c) = g.get(&p) {
                    image.fill(
                        [
                            ((x - min_x) * 6 + xoffs + 3) as i64,
                            ((y - min_y) * 5 + yoffs + 3) as i64,
                        ],
                        (self.to_color)(*c),
                    );
                }
            }
        }
        self.image = Some(image);
    }

    pub fn put_pixel(&mut self, p: Point, rgb: [u8; 3]) {
        if let Some(ref mut image) = self.image {
            let x = p[0] as u32;
            let y = p[1] as u32;
            if x < image.width() && y < image.height() {
                image.put_pixel(x, y, Rgb(rgb));
            }
        }
    }
}

impl<F, G, T> HexGridDrawer<G, T> for BitmapHexGridDrawer<F, G, T>
where
    F: Fn(T) -> [u8; 3],
    G: HexGrid<T>,
    T: PartialEq + Copy + Default,
{
    fn draw(&mut self, area: &G) {
        self.draw_grid(area);
        self.save_image();
    }
}

// psf font parser from https://gist.github.com/hinzundcode/0480c5c8aa220cd43cc8da634119a3c0
const PSF2_MAGIC: [u8; 4] = [0x72, 0xb5, 0x4a, 0x86];

#[derive(Debug)]
pub enum FontError {
    OutOfBounds,
    InvalidMagic,
}

#[repr(C, packed)]
struct PSF2Header {
    magic: [u8; 4],
    version: u32,
    header_size: u32,
    flags: u32,
    length: u32,
    char_size: u32,
    height: u32,
    width: u32,
}

pub struct PSF2Font<'a> {
    data: &'a [u8],
    header: &'a PSF2Header,
}

impl<'a> PSF2Font<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, FontError> {
        if data.len() < std::mem::size_of::<PSF2Header>() {
            return Err(FontError::OutOfBounds);
        }

        let header = unsafe { &(*(data.as_ptr() as *const PSF2Header)) };

        if header.magic != PSF2_MAGIC {
            return Err(FontError::InvalidMagic);
        }

        let last_glyph_pos = header.header_size + header.char_size * (header.length - 1);
        if data.len() < last_glyph_pos as usize {
            return Err(FontError::OutOfBounds);
        }

        Ok(PSF2Font { data, header })
    }

    pub fn glyph_size(&self) -> (u32, u32) {
        (self.header.width, self.header.height)
    }

    pub fn glyph_count(&self) -> u32 {
        self.header.length
    }

    pub fn glyph(&self, index: u32) -> Option<&[u8]> {
        if index >= self.header.length {
            return None;
        }

        let length = self.header.char_size as usize;
        let offset = self.header.header_size as usize + index as usize * length;
        Some(&self.data[offset..(offset + length)])
    }
}

lazy_static! {
    pub static ref SMALLFONT: PSF2Font<'static> =
        PSF2Font::parse(include_bytes!("../fonts/Tamsyn5x9r.psf")).unwrap();
}

pub fn read_lines_from(filename: &str) -> Vec<String> {
    let input = File::open(Path::new(filename)).unwrap();
    let buffered = BufReader::new(input);
    buffered
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.trim_end_matches('\n').to_string())
        .collect()
}

pub fn read_lines() -> (i32, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let bin = Path::new(&args[0]);
    let day = &bin.file_stem().unwrap().to_str().unwrap()[3..];
    let part = if args.len() > 1 {
        args[1].parse::<i32>().unwrap()
    } else {
        -1
    };
    let filename = if args.len() > 2 {
        args[2].to_string()
    } else {
        format!("{}/input.txt", day)
    };
    println!("reading from {}", filename);

    (part, read_lines_from(&filename))
}

pub fn run_main<T, F, G, H, A, B>(parse: F, part1: G, part2: H)
where
    F: Fn(&[String]) -> T,
    G: Fn(&T) -> A,
    H: Fn(&T) -> B,
    A: std::fmt::Display,
    B: std::fmt::Display,
{
    let start_time = Instant::now();
    let (part, lines) = read_lines();
    let io_time = Instant::now();
    let parsed = parse(&lines);
    let parse_time = Instant::now();
    let parts = if part > 0 { vec![part] } else { vec![1, 2] };
    println!(
        "read: {:?}, parse: {:?}\n",
        io_time.duration_since(start_time),
        parse_time.duration_since(io_time),
    );
    for part in parts {
        let part_time = Instant::now();
        if part == 1 {
            let result = part1(&parsed);
            let done_time = Instant::now();
            println!(
                "Part 1: {:<30} ({:?})",
                result,
                done_time.duration_since(part_time)
            );
        } else {
            let result = part2(&parsed);
            let done_time = Instant::now();
            println!(
                "Part 2: {:<30} ({:?})",
                result,
                done_time.duration_since(part_time)
            );
        };
    }
}

pub fn to_hex(data: &[u8]) -> String {
    let mut s = String::with_capacity(2 * data.len());
    for byte in data {
        write!(s, "{:02x}", byte).unwrap();
    }
    s
}

pub fn things<T>(s: &str) -> Vec<T>
where
    T: std::str::FromStr,
{
    s.split(char::is_whitespace)
        .filter_map(|x| x.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_by_empty_line() {
        let result =
            split_by_empty_line(&["apa", "giraff", "", "elefant", "", "lejon", "tiger", ""]);
        let expected = vec![
            vec!["apa", "giraff"],
            vec!["elefant"],
            vec!["lejon", "tiger"],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_chinese_remainder() {
        let modulii = [3, 5, 7];
        let residues = [2, 3, 2];
        assert_eq!(chinese_remainder(&residues, &modulii), Some(23));
    }

    #[test]
    fn test_flip() {
        // Vecs
        let orig_g: Vec<Vec<char>> = vec!["####".chars().collect(), "#   ".chars().collect()];
        let mut g = orig_g.clone();
        let expected: Vec<Vec<char>> = vec!["#   ".chars().collect(), "####".chars().collect()];
        g.flip_vertical();
        assert_eq!(g, expected);
        let mut g = orig_g;
        let expected: Vec<Vec<char>> = vec!["####".chars().collect(), "   #".chars().collect()];
        g.flip_horizontal();
        assert_eq!(g, expected);
        // Hashmaps
        let orig_g: HashMap<Point, char> = vec![
            ([-1, 0], '#'),
            ([0, 0], '#'),
            ([1, 0], '#'),
            ([2, 0], '#'),
            ([-1, 1], '#'),
        ]
        .into_iter()
        .collect();
        let mut g = orig_g.clone();
        let expected: HashMap<Point, char> = vec![
            ([-1, 1], '#'),
            ([0, 1], '#'),
            ([1, 1], '#'),
            ([2, 1], '#'),
            ([-1, 0], '#'),
        ]
        .into_iter()
        .collect();
        g.flip_vertical();
        assert_eq!(g, expected);
        let mut g = orig_g;
        let expected: HashMap<Point, char> = vec![
            ([-1, 0], '#'),
            ([0, 0], '#'),
            ([1, 0], '#'),
            ([2, 0], '#'),
            ([2, 1], '#'),
        ]
        .into_iter()
        .collect();
        g.flip_horizontal();
        assert_eq!(g, expected);
    }

    #[test]
    fn test_parse_point_ok() {
        let parsed = parse_point("12,42");
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), [12, 42]);
    }

    #[test]
    fn test_parse_point_fail() {
        assert!(parse_point("1242").is_err());
        assert!(parse_point("12,,42").is_err());
        assert!(parse_point("12,42,").is_err());
        assert!(parse_point("12,a2").is_err());
        assert!(parse_point("a2,42").is_err());
    }

    #[test]
    fn test_things() {
        let [a,b,c,d] = things::<i64>("apa 1 giraff 3 elefant 5 6")[..] else {
            unreachable!()
        };
        assert_eq!(a, 1);
        assert_eq!(b, 3);
        assert_eq!(c, 5);
        assert_eq!(d, 6);
    }
}
