use aoc;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::iter::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct KeyState {
    state: u32,
}

impl KeyState {
    fn set(&mut self, ch: char) {
        let bit = (ch as u32) - 97;
        self.state |= 1 << bit;
    }

    fn get(&self, ch: char) -> bool {
        let bit = (ch as u32) - 97;
        let mask = 1 << bit;
        (self.state & mask) == mask
    }
}

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
    key_state: KeyState,
    dist: HashMap<(usize, usize), usize>,
    heap: BinaryHeap<State>,
    came_from: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl<'a> Map<'a> {
    fn new(map: &'a Vec<Vec<char>>, key_state: KeyState) -> Map<'a> {
        let mut map = Map {
            map,
            key_state,
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

fn dijkstra_neighbours(state: &Map, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut n = vec![];
    let y = pos.0 as i64;
    let x = pos.1 as i64;
    let w = state.map[0].len() as i64;
    let h = state.map.len() as i64;
    for (ny, nx) in &[(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)] {
        if *nx > w || *ny > h || *ny < 0 || *nx < 0 {
            continue;
        }
        let p = (*ny as usize, *nx as usize);
        let ch = state.map[p.0][p.1];
        if ch == '#' {
            // No action
        } else if ch == '.' {
            n.push((*ny as usize, *nx as usize));
        } else if ch.is_ascii_lowercase() {
            n.push((*ny as usize, *nx as usize));
        } else if ch.is_ascii_uppercase() && state.key_state.get(ch.to_ascii_lowercase()) {
            n.push((*ny as usize, *nx as usize));
        }
    }
    n
}

// Dijkstra's shortest path algorithm.
fn shortest_path(
    state: &mut Map,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<(usize, Vec<(usize, usize)>)> {
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
                    let mut p: Vec<(usize, usize)> = vec![];
                    let mut curr = goal;
                    while curr != start {
                        curr = *state.came_from.get(&curr).unwrap().last().unwrap();
                        p.push(curr)
                    }
                    res = p;
                }
            } else {
                goal_cost = Some(cost);
                let mut p: Vec<(usize, usize)> = vec![];
                let mut curr = goal;
                while curr != start {
                    curr = *state.came_from.get(&curr).unwrap().last().unwrap();
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
        //        println!("neigh: {:?} => {:?}", position, neighbours);
        for neighbour_position in &neighbours {
            let next = State {
                cost: cost + 1,
                position: *neighbour_position,
            };

            let d = if let Some(x) = state.dist.get(&next.position) {
                *x
            } else {
                std::usize::MAX
            };

            // If so, add it to the frontier and continue
            if next.cost < d {
                // Relaxation, we have now found a better way
                state.dist.insert(next.position, next.cost);
                state.heap.push(next);
                // Remember the path
                state.came_from.insert(*neighbour_position, vec![position]);
            } else if next.cost == d {
                state
                    .came_from
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

fn find_keys(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), char> {
    let mut things = HashMap::new();
    let h = map.len();
    let w = map[0].len();
    for y in 0..h {
        for x in 0..w {
            let ch = map[y][x];
            if ch.is_ascii_alphabetic() && ch.is_ascii_lowercase() {
                things.insert((y, x), ch);
            }
        }
    }
    things
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

fn total_cost(paths: &Vec<(usize, Vec<(usize, usize)>)>) -> usize {
    paths.iter().map(|x| x.1.len()).sum()
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct MapState {
    positions: Vec<(usize, usize)>,
    keys: KeyState,
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
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.map_state.positions.cmp(&self.map_state.positions))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &PathState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve<'a>(map: &'a Vec<Vec<char>>, curr: &Vec<(usize, usize)>) -> usize {
    let mut dist = HashMap::new();
    let mut frontier: BinaryHeap<PathState> = BinaryHeap::new();
    let all_keys = find_keys(&map);
    frontier.push(PathState {
        cost: 0,
        map_state: MapState {
            positions: curr.clone(),
            keys: KeyState { state: 0 },
        },
        paths: vec![],
    });
    let mut cached_paths: HashMap<
        ((usize, usize), (usize, usize), KeyState),
        Option<(usize, Vec<(usize, usize)>)>,
    > = HashMap::new();

    let mut goal_cost = None;
    let mut res = vec![];
    let mut last_cost = 0;
    let mut cached = 0;
    let mut total = 0;
    while let Some(PathState {
        cost,
        map_state,
        paths,
    }) = frontier.pop()
    {
        let keys: Vec<_> = all_keys
            .iter()
            .filter(|(_, v)| !map_state.keys.get(**v))
            .collect();
        if cost / 100 != last_cost {
            println!(
                "keys: {:?}, cost: {}, cache: {}%, total: {}",
                keys.len(),
                cost,
                100 * cached / total,
                total
            );
            last_cost = cost / 100;
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
        if let Some(x) = dist.get(&map_state) {
            if cost > *x {
                continue;
            }
        }

        // println!("at {:?}, {}, looking for {:?} keys", map_state.position, cost, keys.len());
        for (pos, key) in keys {
            let pos_len = map_state.positions.len();
            for i in 0..pos_len {
                let rob_pos = map_state.positions[i];
                total += 1;
                let mp = match cached_paths.get(&(rob_pos, *pos, map_state.keys)) {
                    None => {
                        let mut m = Map::new(&map, map_state.keys);
                        let res = shortest_path(&mut m, rob_pos, *pos);
                        cached_paths.insert((rob_pos, *pos, map_state.keys), res.clone());
                        res
                    }
                    Some(x) => {
                        cached += 1;
                        x.clone()
                    }
                };
                if let Some(p) = mp {
                    // println!("found path from {:?} {}, to {} at {:?}", map_state.positions, i, key, pos);
                    // println!("{:?}", p);
                    let mut new_paths = paths.clone();
                    new_paths.push(p.clone());

                    let mut new_keys = map_state.keys;
                    new_keys.set(*key);
                    for p in &p.1 {
                        if let Some(k) = all_keys.get(p) {
                            new_keys.set(*k);
                        }
                    }
                    let mut new_pos = map_state.positions.clone();
                    new_pos[i] = *pos;
                    let next = PathState {
                        cost: total_cost(&new_paths),
                        map_state: MapState {
                            positions: new_pos,
                            keys: new_keys,
                        },
                        paths: new_paths,
                    };

                    let d = if let Some(x) = dist.get(&next.map_state) {
                        *x
                    } else {
                        std::usize::MAX
                    };

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
    }
    goal_cost.unwrap()
}

fn part1(map: &Vec<Vec<char>>) -> usize {
    let curr = find_self(&map).unwrap();
    let cv = vec![curr];
    solve(&map, &cv)
}

fn part2(map: &Vec<Vec<char>>) -> usize {
    let curr = find_self(&map).unwrap();
    let mut m = map.clone();
    m[curr.0][curr.1] = '#';
    m[curr.0][curr.1 + 1] = '#';
    m[curr.0][curr.1 - 1] = '#';
    m[curr.0 - 1][curr.1] = '#';
    m[curr.0 + 1][curr.1] = '#';
    m[curr.0 + 1][curr.1 + 1] = '.';
    m[curr.0 - 1][curr.1 + 1] = '.';
    m[curr.0 + 1][curr.1 - 1] = '.';
    m[curr.0 - 1][curr.1 - 1] = '.';

    let cv = vec![
        (curr.0 + 1, curr.1 + 1),
        (curr.0 - 1, curr.1 + 1),
        (curr.0 + 1, curr.1 - 1),
        (curr.0 - 1, curr.1 - 1),
    ];
    solve(&m, &cv)
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
