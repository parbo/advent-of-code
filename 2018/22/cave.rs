use std::path::Path;
use std::fs::File;
use std::io::Write;
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
    h_cost: i64,
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
        other.h_cost.cmp(&self.h_cost)
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

fn allowed(t: i64, e: Equipment) -> bool {
    match t {
        0 => {
            if e == Equipment::Neither {
                return false;
            }
        },
        1 => {
            if e == Equipment::Torch {
                return false;
            }
        },
        2 => {
            if e == Equipment::ClimbingGear {
                return false;
            }
        },
        _ => panic!()
    }
    return true;
}

fn manhattan(a: (i64, i64), b: (i64, i64)) -> i64 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs())
}

struct Cave {
    memo: Vec<i64>,
    memo_stride: i64,
    depth: i64,
    target: (i64, i64)
}

impl Cave {
    fn new(depth: i64, target: (i64, i64)) -> Cave {
        let mut memo = vec![];
        let stride = 2 * std::cmp::max(target.0, target.1);
        memo.resize((stride * stride) as usize, -1);
        Cave {
            memo: memo,
            memo_stride: stride,
            depth: depth,
            target: target
        }
    }

    fn geo(&mut self, pos: (i64, i64)) -> i64 {
        let ix = (pos.1 * self.memo_stride + pos.0) as usize;
        {
            let v = self.memo[ix];
            if v >= 0 {
                return v;
            }
        }

        let v = match pos {
            (0, 0) => 0,
            x if x == self.target => 0,
            (x, 0) => x * 16807_i64,
            (0, y) => y * 48271_i64,
            (x, y) => {
                let er1 = ero(self.geo((x - 1, y)), self.depth);
                let er2 = ero(self.geo((x, y - 1)), self.depth);
                er1 * er2
            }
        };
        self.memo[ix] = v;
        v
    }

    fn neighbours(&mut self, cp: CavePos, res: &mut Vec<(CavePos, i64)>) {
        res.clear();
        for (dx, dy) in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let nx = cp.pos.0 + dx;
            let ny = cp.pos.1 + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            for e in &[Equipment::ClimbingGear, Equipment::Torch, Equipment::Neither] {
                let t_old = rt(ero(self.geo((cp.pos.0, cp.pos.1)), self.depth));
                if !allowed(t_old, *e) {
                    continue;
                }
                let t_new = rt(ero(self.geo((nx, ny)), self.depth));
                if !allowed(t_new, *e) {
                    continue;
                }
                let cost = if *e == cp.equipment { 1 } else { 8 };
                res.push((CavePos { pos: (nx, ny), equipment: *e }, cost));
            }
        }
    }

    // Dijkstra's shortest path algorithm.
    fn shortest_path(&mut self, start: CavePos, goal: CavePos) -> Option<(Vec<CavePos>, i64)> {
        // dist[node] = current shortest distance from `start` to `node`
        let mut dist = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut came_from = HashMap::new();

        // We're at `start`, with a zero cost
        dist.insert(start, 0);
        heap.push(State { h_cost: manhattan(goal.pos, (0, 0)), cost: 0, position: start });

        let mut nb = vec![];

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(s) = heap.pop() {
            if s.position == goal {
                let mut path = vec![goal];
                let mut current = goal;
                while let Some(pos) = came_from.get(&current) {
                    path.insert(0, *pos);
                    current = *pos;
                }
                return Some((path, s.cost));
            }

            // Important as we may have already found a better way
            if s.cost > *dist.entry(s.position).or_insert(std::i64::MAX) {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            self.neighbours(s.position, &mut nb);
            for (nb_position, nb_cost) in &nb {
                let new_cost = s.cost + *nb_cost;
                let h = new_cost + manhattan(goal.pos, nb_position.pos) + if nb_position.equipment != goal.equipment { 7 } else { 0 };
                let next = State { h_cost: h, cost: new_cost, position: *nb_position };

                let d = *dist.entry(next.position).or_insert(std::i64::MAX);

                // If so, add it to the frontier and continue
                if next.cost < d {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist.insert(next.position, next.cost);
                    // Remember the path
                    came_from.insert(*nb_position, s.position);
                }
            }
        }

        // Goal not reachable
        None
    }
}

fn draw(cave: &mut Cave, path: &Vec<CavePos>, target: (i64, i64)) {
    let max_x = cave.memo.iter().enumerate().filter(|(_, &c)| c != -1).map(|(n, _)| (n as i64) % cave.memo_stride).max().unwrap();
    let max_y = cave.memo.iter().enumerate().filter(|(_, &c)| c != -1).map(|(n, _)| (n as i64) / cave.memo_stride).max().unwrap();
    let mut cps : HashMap<(i64, i64), CavePos> = HashMap::new();
    for p in path {
        cps.insert(p.pos, *p);
    }
    // Draw it
    println!("saving image");
    let name = format!("{}_{}_{}.ppm", cave.depth, cave.target.0, cave.target.1);
    let path = Path::new(&name);
    let mut file = File::create(&path).unwrap();
    let w = max_x + 1;
    let h = max_y + 1;
    let header = format!("P6 {} {} 255\n", w, h);
    let mut data = vec![];
    data.extend(header.as_bytes());
    println!("target: {:?}", target);
    for y in 0..h {
        print!("{:4} ", y);
        for x in 0..w {
            if x == 0 && y == 0 {
                print!("M");
                data.push(0xff);
                data.push(0xff);
                data.push(0xff);
            } else if (x as i64, y as i64) == target {
                print!("T");
                data.push(0xff);
                data.push(0xff);
                data.push(0xff);
            } else if let Some(cp) = cps.get(&(x, y)) {
                match cp.equipment {
                    Equipment::ClimbingGear => {
                        print!("x");
                        data.push(0);
                        data.push(0xff);
                        data.push(0xff);
                    },
                    Equipment::Torch => {
                        print!("+");
                        data.push(0xff);
                        data.push(0xff);
                        data.push(0);
                    },
                    Equipment::Neither => {
                        print!("o");
                        data.push(0xff);
                        data.push(0);
                        data.push(0xff);
                    }
                }
            } else if cave.memo[(y * cave.memo_stride + x) as usize] != -1 {
                let t = rt(ero(cave.memo[(y * cave.memo_stride + x) as usize], cave.depth));
                match t {
                    0 => {
                        print!(".");
                        data.push(0);
                        data.push(0x80);
                        data.push(0);
                    },
                    1 => {
                        print!("=");
                        data.push(0);
                        data.push(0);
                        data.push(0x80);
                    },
                    2 => {
                        print!("|");
                        data.push(0x80);
                        data.push(0);
                        data.push(0);
                    },
                    _ => panic!()
                }
            } else {
                data.push(0);
                data.push(0);
                data.push(0);
            }
        }
        println!("");
    }
    file.write(&data).unwrap();
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
