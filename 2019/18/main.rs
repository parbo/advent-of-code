use aoc;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::iter::*;

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize), // y, x
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
            .then_with(|| other.position.0.cmp(&self.position.0))
            .then_with(|| other.position.1.cmp(&self.position.1))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Map {
    map: Vec<Vec<char>>,
    dist: HashMap<(usize, usize), usize>,
    heap: BinaryHeap<State>,
    came_from: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl Map {
    fn new(m: Vec<Vec<char>>) -> Map {
        let mut map = Map {
            map: m,
            dist: HashMap::new(),
            heap: BinaryHeap::new(),
            came_from: HashMap::new(),
        };
        map.dist.reserve(256);
        map.heap.reserve(256);
        map.came_from.reserve(256);
        map
    }

    fn dijkstra_neighbours(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        self.neighbours(pos, |p| {
            self.map[p.0][p.1] == '.'
                || (self.map[p.0][p.1].is_ascii_alphabetic()
                    && self.map[p.0][p.1].is_ascii_lowercase())
        })
    }

    fn neighbours(
        &self,
        pos: (usize, usize),
        filter: impl Fn((usize, usize)) -> bool,
    ) -> Vec<(usize, usize)> {
        let mut n = vec![];
        let y = pos.0 as i64;
        let x = pos.1 as i64;
        let w = self.map[0].len() as i64;
        let h = self.map.len() as i64;
        for (ny, nx) in &[(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)] {
            if *nx > w || *ny > h || *ny < 0 || *nx < 0 {
                continue;
            }
            let p = (*ny as usize, *nx as usize);
            if !filter(p) {
                continue;
            }
            n.push((*ny as usize, *nx as usize));
        }
        n
    }

    // Dijkstra's shortest path algorithm.
    fn shortest_path(
        &mut self,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Option<(usize, Vec<(usize, usize)>)> {
        self.dist.clear();
        self.heap.clear();
        self.came_from.clear();

        // We're at `start`, with a zero cost
        self.dist.insert(start, 0);
        self.heap.push(State {
            cost: 0,
            position: start,
        });

        let mut goal_cost = None;
        let mut res = vec![];

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State { cost, position }) = self.heap.pop() {
            if position == goal {
                if let Some(gc) = goal_cost {
                    if cost == gc {
                        res.extend(self.came_from.get(&goal).unwrap());
                    }
                } else {
                    goal_cost = Some(cost);
                    res.extend(self.came_from.get(&goal).unwrap());
                }
            }

            if let Some(gc) = goal_cost {
                if cost > gc {
                    return Some((gc, res));
                }
            }

            // Important as we may have already found a better way
            if cost > *self.dist.entry(position).or_insert(std::usize::MAX) {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            let neighbours = self.dijkstra_neighbours(position);
            //        println!("neigh: {:?} => {:?}", position, neighbours);
            for neighbour_position in &neighbours {
                let next = State {
                    cost: cost + 1,
                    position: *neighbour_position,
                };

                let d = *self.dist.entry(next.position).or_insert(std::usize::MAX);

                // If so, add it to the frontier and continue
                if next.cost < d {
                    // Relaxation, we have now found a better way
                    self.dist.insert(next.position, next.cost);
                    self.heap.push(next);
                    // Remember the path
                    self.came_from.insert(*neighbour_position, vec![position]);
                } else if next.cost == d {
                    self.came_from
                        .entry(*neighbour_position)
                        .or_insert(vec![])
                        .push(position);
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
}

fn find_keys(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), char> {
    let mut keys = HashMap::new();
    let h = map.len();
    let w = map[0].len();
    for y in 0..h {
        for x in 0..w {
            let ch = map[y][x];
            if ch.is_ascii_alphabetic() && ch.is_ascii_lowercase() {
                keys.insert((y, x), ch);
            }
        }
    }
    keys
}

fn find_self(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let h = map.len();
    let w = map[0].len();
    for y in 0..h {
        for x in 0..w {
            if map[y][x] == '@' {
                return Some((y, x));
            }
        }
    }
    None
}

fn remove_thing(map: &mut Vec<Vec<char>>, thing: char) {
    let h = map.len();
    let w = map[0].len();
    for y in 0..h {
        for x in 0..w {
            let ch = map[y][x];
            if ch == thing {
                map[y][x] = '.';
            }
        }
    }
}

fn clear_path(map: &mut Vec<Vec<char>>, path: &Vec<(usize, usize)>) {
    for p in path {
        map[p.0][p.1] = '.'
    }
}

fn total_cost(paths: &Vec<(usize, Vec<(usize, usize)>)>) -> usize {
    paths.iter().map(|x| x.0).sum()
}

fn part1(map: &Vec<Vec<char>>) -> i64 {
    let curr = find_self(&map).unwrap();
    let mut frontier: Vec<((usize, usize), char, Vec<(usize, Vec<(usize, usize)>)>)> =
        vec![(curr, '@', vec![])];
    let mut solutions = vec![];
    loop {
        if frontier.len() == 0 {
            break;
        }
        let mut new_frontier = vec![];
        for f in frontier.clone() {
	    println!("f: {:?}", f);
            let mut fmap = map.clone();
            remove_thing(&mut fmap, f.1);
            if f.1.is_ascii_alphabetic() {
                remove_thing(&mut fmap, f.1.to_ascii_uppercase());
            }
            if let Some(p) = f.2.last() {
                clear_path(&mut fmap, &p.1);
            }
            let keys = find_keys(&fmap);
            if keys.len() == 0 {
                solutions.push(f.clone());
            }
            for (pos, key) in keys {
                let mut m = Map::new(fmap.clone());
                if let Some(p) = m.shortest_path(curr, pos) {
                    let mut paths = f.2.clone();
                    paths.push(p);
                    new_frontier.push((pos, key, paths));
                }
            }
        }
        frontier = new_frontier;
    }
    println!("{:?}", solutions);
    0
    // let best = solutions
    //     .iter()
    //     .min_by(|a, b| total_cost(&a).cmp(&total_cost(&b)));
    // total_cost(best)
}

fn part2(_: &Vec<Vec<char>>) -> i64 {
    0
}

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|x| x.chars().collect()).collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    // use super::part1;

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&vec![0]), 0);
    // }
}
