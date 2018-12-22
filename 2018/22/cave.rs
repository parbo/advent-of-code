use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Equipment {
    ClimbingGear,
    Torch,
    Neither
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct CavePos {
    pos: (i64, i64),
    equipment: Equipment
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    position: CavePos
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

fn rt(e: i64) -> i64 {
    e % 3
}

fn ero(g: i64, depth: i64) -> i64 {
    (g + depth) % 20183
}

struct Cave {
    memo: HashMap<(i64, i64), i64>,
    depth: i64,
}

impl Cave {
    fn new(depth: i64, target: (i64, i64)) -> Cave {
        let mut memo = HashMap::new();
        memo.insert(target, 0);
        Cave {
            memo: memo,
            depth: depth,
        }
    }

    fn geo(&mut self, pos: (i64, i64)) -> i64 {
        {
            if let Some(v) = self.memo.get(&pos) {
                return *v;
            }
        }

        let v = match pos {
            (0, 0) => 0,
            (x, 0) => x * 16807,
            (0, y) => y * 48271,
            (x, y) => {
                let er1 = ero(self.geo((x - 1, y)), self.depth);
                let er2 = ero(self.geo((x, y - 1)), self.depth);
                er1 * er2
            }
        };
        self.memo.insert(pos, v);
        v
    }

    fn neighbours(&mut self, cp: CavePos) -> Vec<(CavePos, i64)> {
        let mut res = vec![];
        for (dx, dy) in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let nx = cp.pos.0 + dx;
            let ny = cp.pos.1 + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            for e in &[Equipment::ClimbingGear, Equipment::Torch, Equipment::Neither] {
                let t = rt(ero(self.geo((nx, ny)), self.depth));
                match t {
                    0 => {
                        if *e == Equipment::Neither {
                            continue;
                        }
                    },
                    1 => {
                        if *e == Equipment::Torch {
                            continue;
                        }
                    },
                    2 => {
                        if *e == Equipment::ClimbingGear {
                            continue;
                        }
                    },
                    _ => panic!()
                }
                let cost = if *e == cp.equipment { 1 } else { 8 };
                res.push((CavePos { pos: (nx, ny), equipment: *e }, cost));
            }
        }
        res
    }

    // Dijkstra's shortest path algorithm.
    fn shortest_path(&mut self, start: CavePos, goal: CavePos) -> Option<(Vec<CavePos>, i64)> {
        // dist[node] = current shortest distance from `start` to `node`
        let mut dist = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut came_from = HashMap::new();

        // We're at `start`, with a zero cost
        dist.insert(start, 0);
        heap.push(State { cost: 0, position: start });

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State { cost, position }) = heap.pop() {
            if position == goal {
                println!("pos: {:?},  goal: {:?}", position, goal);
                let mut path = vec![goal];
                let mut current = goal;
                while let Some(pos) = came_from.get(&current) {
                    path.insert(0, *pos);
                    current = *pos;
                }
                return Some((path, cost));
            }

            // Important as we may have already found a better way
            if cost > *dist.entry(position).or_insert(std::i64::MAX) {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            let nb = self.neighbours(position);
            for (nb_position, nb_cost) in &nb {
                let next = State { cost: cost + *nb_cost, position: *nb_position };

                let d = *dist.entry(next.position).or_insert(std::i64::MAX);

                // If so, add it to the frontier and continue
                if next.cost < d {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist.insert(next.position, next.cost);
                    // Remember the path
                    came_from.insert(*nb_position, position);
                }
            }
        }

        // Goal not reachable
        None
    }
}

fn draw(cave: &mut Cave, path: &Vec<CavePos>, target: (i64, i64)) {
    let max_y = path.iter().map(|c| c.pos.1).max().unwrap();
    let max_x = path.iter().map(|c| c.pos.0).max().unwrap();
    let mut cps : HashMap<(i64, i64), CavePos> = HashMap::new();
    for p in path {
        cps.insert(p.pos, *p);
    }
    // Draw it
    println!("target: {:?}", target);
    for y in 0..(max_y+1) {
        print!("{:4} ", y);
        for x in 0..(max_x+1) {
            if x == 0 && y == 0 {
                print!("M");
            } else if (x as i64, y as i64) == target {
                print!("T");
            } else if let Some(cp) = cps.get(&(x, y)) {
                match cp.equipment {
                    Equipment::ClimbingGear => print!("x"),
                    Equipment::Torch => print!("¤"),
                    Equipment::Neither => print!("o")
                }
            } else {
                let t = rt(ero(cave.geo((x, y)), cave.depth));
                match t {
                    0 => print!("."),
                    1 => print!("="),
                    2 => print!("|"),
                    _ => panic!()
                }
            }
        }
        println!("");
    }
}

fn solve(depth: i64, target: (i64, i64)) {
    let mut cave = Cave::new(depth, target);
    let mut grid = vec![];
    // find the risk
    let mut risk = 0;
    for y in 0..(target.1+1) {
        grid.push(vec![]);
        for x in 0..(target.0+1) {
            let t = rt(ero(cave.geo((x, y)), depth));
            risk += t;
            grid[y as usize].push(t);
        }
    }
    println!("risk: {}", risk);
    // find the target
    let p = cave.shortest_path(
        CavePos { pos: (0, 0), equipment: Equipment::Torch },
        CavePos { pos: target, equipment: Equipment::Torch }
    );
    let (path, cost) = p.unwrap();
    println!("path: {}", path.len());
    draw(&mut cave, &path, target);
    println!("minutes: {}", cost);
    let mut c = 0;
    let mut last = path[0];
    for p in &path[1..] {
        c += 1;
        if p.equipment != last.equipment {
            c += 7;
        }
        last = *p;
    }
    println!("calc minutes: {}", c);
}

fn main() {
    {
        let depth = 510;
        let target = (10, 10);
        solve(depth, target);
    }

    {
        let depth = 5616;
        let target = (10, 785);
        solve(depth, target);
    }
}
