use std::env;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::*;
use std::path::Path;

pub trait Neighbours {
    fn neighbours(&self, pos: (usize, usize)) -> Vec<(usize, usize)>;
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize)  // note: this is y, x for comparison reasons
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(n: &Neighbours, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut came_from = HashMap::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            let mut path = vec![goal];
            let mut current = goal;
            while let Some(pos) = came_from.get(&current) {
                path.push(*pos);
                current = *pos;
            }
            return Some(path);
        }

        // Important as we may have already found a better way
        if cost > *dist.entry(position).or_insert(std::usize::MAX) {
            continue;
        }

        println!("pos: {:?}", position);

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        let neighbours = n.neighbours(position);
        for neighbour_position in &neighbours {
            let next = State { cost: cost + 1, position: *neighbour_position };

            let d = *dist.entry(next.position).or_insert(std::usize::MAX);

            // If so, add it to the frontier and continue
            if next.cost < d {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(next.position, next.cost);
                // Remember the path
                came_from.insert(*neighbour_position, position);
            }
        }
    }

    // Goal not reachable
    None
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Stats {
    attack: i64,
    hp: i64
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Entity {
    Elf(Stats),
    Goblin(Stats),
    Wall,
    Floor
}

struct Map {
    map: Vec<Vec<Entity>>
}

impl Neighbours for Map {
    fn neighbours(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut n = vec![];
        let y = pos.0 as i64;
        let x = pos.1 as i64;
        let w = self.map[0].len() as i64;
        let h = self.map.len() as i64;
        for (ny, nx) in &[(y - 1, x -1),
                          (y - 1, x),
                          (y - 1, x + 1),
                          (y, x - 1),
                          (y, x + 1),
                          (y + 1, x - 1),
                          (y + 1, x),
                          (y + 1, x + 1)] {
            if *nx > w || *ny > h || *ny < 0 || *nx < 0 {
                continue;
            }
            if self.map[*ny as usize][*nx as usize] != Entity::Floor {
                continue;
            }
            n.push((*ny as usize, *nx as usize));
        }
        n
    }
}

fn solve(path: &Path) {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let lines : Vec<String> = buffered.lines().filter_map(Result::ok).collect();
    let mut grid = vec![];
    let mut max_w = 0;
    for line in lines {
        let v : Vec<char> = line.chars().collect();
        max_w = std::cmp::max(v.len(), max_w);
        grid.push(v);
    }
    for row in &mut grid {
        row.resize(max_w, ' ');
    }
    let mut m = vec![];
    for row in &grid {
        let mut map_row = vec![];
        for col in row {
            let entity = match col {
                '#' => Entity::Wall,
                '.' => Entity::Floor,
                'E' => Entity::Elf(Stats { attack: 3, hp: 200 }),
                'G' => Entity::Goblin(Stats { attack: 3, hp: 200 }),
                _ => panic!()
            };
            map_row.push(entity);
        }
        m.push(map_row);
    }
    let map = Map { map: m };

    let path = shortest_path(&map, (13, 22), (16, 23));
    println!("{:?}", path);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
