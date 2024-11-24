use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::*;

type Parsed = Vec<Vec<char>>;

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize, usize), // x, y, depth
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.position.2.cmp(&self.position.2))
            .then_with(|| other.position.1.cmp(&self.position.1))
            .then_with(|| other.position.0.cmp(&self.position.0))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Portals = HashMap<(usize, usize), ((usize, usize), bool)>;

struct Map<'a> {
    map: &'a Vec<Vec<char>>,
    portals: &'a Portals,
    recurse: bool,
    dist: HashMap<(usize, usize, usize), usize>,
    heap: BinaryHeap<State>,
    came_from: HashMap<(usize, usize, usize), (usize, usize, usize)>,
}

impl<'a> Map<'a> {
    fn new(map: &'a Vec<Vec<char>>, portals: &'a Portals, recurse: bool) -> Map<'a> {
        let mut map = Map {
            map,
            portals,
            recurse,
            dist: HashMap::new(),
            heap: BinaryHeap::new(),
            came_from: HashMap::new(),
        };
        map.dist.reserve(1024);
        map.heap.reserve(1024);
        map.came_from.reserve(1024);
        map
    }
}

fn dijkstra_neighbours(state: &Map, pos: (usize, usize, usize)) -> Vec<(usize, usize, usize)> {
    let mut n = vec![];
    let x = pos.0 as i64;
    let y = pos.1 as i64;
    let w = state.map[0].len() as i64;
    let h = state.map.len() as i64;
    for (nx, ny) in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
        if *nx > w || *ny > h || *ny < 0 || *nx < 0 {
            continue;
        }
        let p = (*nx as usize, *ny as usize);
        let ch = state.map[p.1][p.0];
        if ch == '.' {
            n.push((p.0, p.1, pos.2));
        }
    }
    if let Some(((xx, yy), inner)) = state.portals.get(&(pos.0, pos.1)) {
        if state.recurse {
            if *inner {
                n.push((*xx, *yy, pos.2 + 1));
            } else if pos.2 > 0 {
                n.push((*xx, *yy, pos.2 - 1));
            }
        } else {
            n.push((*xx, *yy, pos.2));
        }
    }
    n
}

// Dijkstra's shortest path algorithm.
#[allow(clippy::type_complexity)]
fn shortest_path(
    state: &mut Map,
    start: (usize, usize, usize),
    goal: (usize, usize, usize),
) -> Option<(usize, Vec<(usize, usize, usize)>)> {
    state.dist.clear();
    state.heap.clear();
    state.came_from.clear();

    // We're at `start`, with a zero cost
    state.dist.insert(start, 0);
    state.heap.push(State {
        cost: 0,
        position: start,
    });

    let mut goal_cost = None;
    let mut res = vec![];

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = state.heap.pop() {
        if position == goal {
            if let Some(gc) = goal_cost {
                if cost == gc {
                    let mut p: Vec<(usize, usize, usize)> = vec![];
                    let mut curr = goal;
                    while curr != start {
                        curr = *state.came_from.get(&curr).unwrap();
                        p.push(curr)
                    }
                    res = p;
                }
            } else {
                goal_cost = Some(cost);
                let mut p: Vec<(usize, usize, usize)> = vec![];
                let mut curr = goal;
                while curr != start {
                    curr = *state.came_from.get(&curr).unwrap();
                    p.push(curr)
                }
                res = p;
            }
        }

        if let Some(gc) = goal_cost {
            if cost > gc {
                return Some((gc, res));
            }
        }

        // Important as we may have already found a better way
        if let Some(x) = state.dist.get(&position) {
            if cost > *x {
                continue;
            }
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        let neighbours = dijkstra_neighbours(state, position);
        for neighbour_position in &neighbours {
            let next = State {
                cost: cost + 1,
                position: *neighbour_position,
            };

            let d = if let Some(x) = state.dist.get(&next.position) {
                *x
            } else {
                usize::MAX
            };

            // If so, add it to the frontier and continue
            if next.cost < d {
                // Relaxation, we have now found a better way
                state.dist.insert(next.position, next.cost);
                state.heap.push(next);
                // Remember the path
                state.came_from.insert(*neighbour_position, position);
            }
        }
    }

    if let Some(gc) = goal_cost {
        return Some((gc, res));
    } else {
        assert_eq!(res.len(), 0);
    }

    // Goal not reachable
    None
}

fn to_label(ch_a: char, ch_b: char, a: (usize, usize), b: (usize, usize)) -> (char, char) {
    let mut v = [(a, ch_a), (b, ch_b)];
    v.sort();
    (v[0].1, v[1].1)
}

#[allow(clippy::type_complexity)]
#[allow(clippy::ptr_arg)]
fn find_portals(
    map: &Vec<Vec<char>>,
) -> (
    (usize, usize),
    (usize, usize),
    HashMap<(usize, usize), ((usize, usize), bool)>,
) {
    let h = map.len();
    let w = map[0].len();
    let mut portals = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut labels: HashMap<(char, char), HashSet<((usize, usize), bool)>> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            let ch_a = map[y][x];
            if ch_a == '.' {
                for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    if x as i64 + dx >= w as i64
                        || y as i64 + dy >= h as i64
                        || y as i64 + dy < 0
                        || x as i64 + dx < 0
                    {
                        continue;
                    }
                    let xx = (x as i64 + dx) as usize;
                    let yy = (y as i64 + dy) as usize;
                    let xxx = (xx as i64 + dx) as usize;
                    let yyy = (yy as i64 + dy) as usize;
                    let ch_b = map[yy][xx];
                    if ch_b.is_ascii_alphabetic() {
                        let ch_c = map[yyy][xxx];
                        let inner = x > 3 && x + 3 < w && y > 3 && y + 3 < h;
                        labels
                            .entry(to_label(ch_b, ch_c, (xx, yy), (xxx, yyy)))
                            .or_default()
                            .insert(((x, y), inner));
                    }
                }
            }
        }
    }
    for (label, pos) in labels {
        if label == ('A', 'A') {
            start = pos.iter().next().unwrap().0;
        } else if label == ('Z', 'Z') {
            end = pos.iter().next().unwrap().0;
        } else {
            let mut it = pos.iter();
            let a = *it.next().unwrap();
            let b = *it.next().unwrap();
            portals.insert(a.0, (b.0, a.1));
            portals.insert(b.0, (a.0, b.1));
        }
    }
    (start, end, portals)
}

#[allow(clippy::needless_range_loop)]
fn part1(map: &Parsed) -> usize {
    let (start, end, portals) = find_portals(map);
    let mut m = Map::new(map, &portals, false);
    if let Some(res) = shortest_path(&mut m, (start.0, start.1, 0), (end.0, end.1, 0)) {
        let h = map.len();
        let w = map[0].len();
        let p: HashSet<_> = res.1.iter().collect();
        for y in 0..h {
            for x in 0..w {
                if p.contains(&(x, y, 0)) {
                    print!("*");
                } else {
                    let ch_a = map[y][x];
                    print!("{}", ch_a);
                }
            }
            println!();
        }
        res.0
    } else {
        0
    }
}

fn part2(map: &Parsed) -> usize {
    let (start, end, portals) = find_portals(map);
    let mut m = Map::new(map, &portals, true);
    if let Some(res) = shortest_path(&mut m, (start.0, start.1, 0), (end.0, end.1, 0)) {
        res.0
    } else {
        0
    }
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let map = vec![
            "         A           ".chars().collect(),
            "         A           ".chars().collect(),
            "  #######.#########  ".chars().collect(),
            "  #######.........#  ".chars().collect(),
            "  #######.#######.#  ".chars().collect(),
            "  #######.#######.#  ".chars().collect(),
            "  #######.#######.#  ".chars().collect(),
            "  #####  B    ###.#  ".chars().collect(),
            "BC...##  C    ###.#  ".chars().collect(),
            "  ##.##       ###.#  ".chars().collect(),
            "  ##...DE  F  ###.#  ".chars().collect(),
            "  #####    G  ###.#  ".chars().collect(),
            "  #########.#####.#  ".chars().collect(),
            "DE..#######...###.#  ".chars().collect(),
            "  #.#########.###.#  ".chars().collect(),
            "FG..#########.....#  ".chars().collect(),
            "  ###########.#####  ".chars().collect(),
            "             Z       ".chars().collect(),
            "             Z       ".chars().collect(),
        ];
        assert_eq!(part1(&map), 23);
    }

    #[test]
    fn test_part2() {
        let map = vec![
            "             Z L X W       C                 "
                .chars()
                .collect(),
            "             Z P Q B       K                 "
                .chars()
                .collect(),
            "  ###########.#.#.#.#######.###############  "
                .chars()
                .collect(),
            "  #...#.......#.#.......#.#.......#.#.#...#  "
                .chars()
                .collect(),
            "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  "
                .chars()
                .collect(),
            "  #.#...#.#.#...#.#.#...#...#...#.#.......#  "
                .chars()
                .collect(),
            "  #.###.#######.###.###.#.###.###.#.#######  "
                .chars()
                .collect(),
            "  #...#.......#.#...#...#.............#...#  "
                .chars()
                .collect(),
            "  #.#########.#######.#.#######.#######.###  "
                .chars()
                .collect(),
            "  #...#.#    F       R I       Z    #.#.#.#  "
                .chars()
                .collect(),
            "  #.###.#    D       E C       H    #.#.#.#  "
                .chars()
                .collect(),
            "  #.#...#                           #...#.#  "
                .chars()
                .collect(),
            "  #.###.#                           #.###.#  "
                .chars()
                .collect(),
            "  #.#....OA                       WB..#.#..ZH"
                .chars()
                .collect(),
            "  #.###.#                           #.#.#.#  "
                .chars()
                .collect(),
            "CJ......#                           #.....#  "
                .chars()
                .collect(),
            "  #######                           #######  "
                .chars()
                .collect(),
            "  #.#....CK                         #......IC"
                .chars()
                .collect(),
            "  #.###.#                           #.###.#  "
                .chars()
                .collect(),
            "  #.....#                           #...#.#  "
                .chars()
                .collect(),
            "  ###.###                           #.#.#.#  "
                .chars()
                .collect(),
            "XF....#.#                         RF..#.#.#  "
                .chars()
                .collect(),
            "  #####.#                           #######  "
                .chars()
                .collect(),
            "  #......CJ                       NM..#...#  "
                .chars()
                .collect(),
            "  ###.#.#                           #.###.#  "
                .chars()
                .collect(),
            "RE....#.#                           #......RF"
                .chars()
                .collect(),
            "  ###.###        X   X       L      #.#.#.#  "
                .chars()
                .collect(),
            "  #.....#        F   Q       P      #.#.#.#  "
                .chars()
                .collect(),
            "  ###.###########.###.#######.#########.###  "
                .chars()
                .collect(),
            "  #.....#...#.....#.......#...#.....#.#...#  "
                .chars()
                .collect(),
            "  #####.#.###.#######.#######.###.###.#.#.#  "
                .chars()
                .collect(),
            "  #.......#.......#.#.#.#.#...#...#...#.#.#  "
                .chars()
                .collect(),
            "  #####.###.#####.#.#.#.#.###.###.#.###.###  "
                .chars()
                .collect(),
            "  #.......#.....#.#...#...............#...#  "
                .chars()
                .collect(),
            "  #############.#.#.###.###################  "
                .chars()
                .collect(),
            "               A O F   N                     "
                .chars()
                .collect(),
            "               A A D   M                     "
                .chars()
                .collect(),
        ];
        assert_eq!(part2(&map), 396);
    }
}
