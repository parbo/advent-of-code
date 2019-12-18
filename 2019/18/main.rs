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

struct Map<'a> {
    map: &'a Vec<Vec<char>>,
    dist: HashMap<(usize, usize), usize>,
    heap: BinaryHeap<State>,
    came_from: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl<'a> Map<'a> {
    fn new(m: &'a Vec<Vec<char>>) -> Map {
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
                        let mut p: Vec<(usize, usize)> = vec![];
                        let mut curr = goal;
                        while curr != start {
                            curr = *self.came_from.get(&curr).unwrap().last().unwrap();
                            p.push(curr)
                        }
                        res = p;
                    }
                } else {
                    goal_cost = Some(cost);
                    let mut p: Vec<(usize, usize)> = vec![];
                    let mut curr = goal;
                    while curr != start {
                        curr = *self.came_from.get(&curr).unwrap().last().unwrap();
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
    paths.iter().map(|x| x.1.len()).sum()
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct MapState {
    position: (usize, usize),
    map: Vec<Vec<char>>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct PathState {
    cost: usize,
    map_state: MapState,
    paths: Vec<(usize, Vec<(usize, usize)>)>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for PathState {
    fn cmp(&self, other: &PathState) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        let other_keys = find_keys(&other.map_state.map);
        let self_keys = find_keys(&self.map_state.map);
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other_keys.len().cmp(&self_keys.len()))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &PathState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(map: &Vec<Vec<char>>) -> i64 {
    let curr = find_self(&map).unwrap();
    let mut dist = HashMap::new();
    let mut frontier: BinaryHeap<PathState> = BinaryHeap::new();
    frontier.push(PathState {
        cost: 0,
        map_state: MapState {
            position: curr,
            map: map.clone(),
        },
        paths: vec![],
    });
    let mut goal_cost = None;
    let mut res = vec![];
    let mut last_keys = std::usize::MAX;
    while let Some(PathState {
        cost,
        map_state,
        paths,
    }) = frontier.pop()
    {
        let keys = find_keys(&map_state.map);
        if keys.len() != last_keys {
            println!("keys: {:?}, cost: {}", keys.len(), cost);
            last_keys = keys.len();
        }
        if keys.len() == 0 {
            if let Some(gc) = goal_cost {
                if cost == gc {
                    res.push((map_state.clone(), paths.clone()));
                }
            } else {
                goal_cost = Some(cost);
                res.push((map_state.clone(), paths.clone()));
            }
        }

        if let Some(gc) = goal_cost {
            if cost > gc {
                break;
            }
        }

        // Important as we may have already found a better way
        let existing_cost = *dist.entry(map_state.clone()).or_insert(std::usize::MAX);
        if cost > existing_cost {
            // println!("at {:?}, {} > {}", map_state.position, cost, existing_cost);
            continue;
        }

        // println!("at {:?}, {}, looking for {:?} keys", map_state.position, cost, keys.len());
        for (pos, key) in keys {
            let mut m = Map::new(&map_state.map);
            if let Some(p) = m.shortest_path(map_state.position, pos) {
                // println!("found path from {:?} to {} at {:?}", map_state.position, key, pos);
                // println!("{:?}", p);
                let mut new_paths = paths.clone();
                new_paths.push(p.clone());

                let mut new_map = map_state.map.clone();
                remove_thing(&mut new_map, key);
                if key.is_ascii_alphabetic() {
                    remove_thing(&mut new_map, key.to_ascii_uppercase());
                }
                clear_path(&mut new_map, &p.1);
                new_map[pos.0][pos.1] = '.';
                let next = PathState {
                    cost: total_cost(&new_paths),
                    map_state: MapState {
                        position: pos,
                        map: new_map,
                    },
                    paths: new_paths,
                };

                let d = *dist
                    .entry(next.map_state.clone())
                    .or_insert(std::usize::MAX);

                // println!("next: {}, d: {}", next.cost, d);

                // If so, add it to the frontier and continue
                if next.cost < d {
                    // Relaxation, we have now found a better way
                    dist.insert(next.map_state.clone(), next.cost);
                    frontier.push(next);
                }
            }
        }
    }
    println!("{:?}, {:?}", res, goal_cost);
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
