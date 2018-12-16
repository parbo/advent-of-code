use std::env;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
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
    position: (usize, usize)  // y, x
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
            .then_with(|| other.position.cmp(&self.position))
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
fn shortest_path(n: &Neighbours, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<Vec<(usize, usize)>>> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut came_from = HashMap::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start });

    let mut shortest = std::usize::MAX;
    let mut paths : Vec<Vec<(usize, usize)>> = vec![];

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            let mut path = vec![goal];
            let mut current = goal;
            while let Some(pos) = came_from.get(&current) {
                path.insert(0, *pos);
                current = *pos;
            }
            if path.len() > shortest {
                paths.sort();
                return Some(paths);
            } else if path.len() == shortest {
                paths.push(path);
            } else {
                shortest = path.len();
                paths = vec![path];
            }
        }

        // Important as we may have already found a better way
        if cost > *dist.entry(position).or_insert(std::usize::MAX) {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        let neighbours = n.neighbours(position);
//        println!("neigh: {:?} => {:?}", position, neighbours);
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

    if paths.len() > 0 {
        return Some(paths);
    }

    // Goal not reachable
    None
}

fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()) as usize
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum FighterKind {
    Elf,
    Goblin
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Fighter {
    kind: FighterKind,
    attack: i64,
    hp: i64,
    id: (usize, usize)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Entity {
    Being(Fighter),
    Wall,
    Floor
}

struct Map {
    map: Vec<Vec<Entity>>
}

impl Neighbours for Map {
    fn neighbours(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        self.neighbours(pos, |p| self.map[p.0][p.1] == Entity::Floor)
    }
}

impl Map {
    fn neighbours(&self, pos: (usize, usize), filter: impl Fn((usize, usize)) -> bool) -> Vec<(usize, usize)> {
        let mut n = vec![];
        let y = pos.0 as i64;
        let x = pos.1 as i64;
        let w = self.map[0].len() as i64;
        let h = self.map.len() as i64;
        for (ny, nx) in &[(y - 1, x),
                          (y, x - 1),
                          (y, x + 1),
                          (y + 1, x)] {
            if *nx > w || *ny > h || *ny < 0 || *nx < 0 {
                continue;
            }
            let p = (*ny as usize, *nx as usize);
            if !filter(p) {
                continue
            }
            n.push((*ny as usize, *nx as usize));
        }
        n
    }

    fn entity(&self, pos: (usize, usize)) -> Entity {
        self.map[pos.0][pos.1]
    }

    fn enemies_in_range(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut enemies = vec![];
        for npos in self.neighbours(pos, |_| true) {
            if let Entity::Being(a) = self.entity(pos) {
                if let Entity::Being(b) = self.entity(npos) {
                    if a.kind != b.kind {
                        enemies.push(npos);
                    }
                }
            }
        }
        enemies
    }

    fn enemies(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut enemies = vec![];
        for (y, row) in self.map.iter().enumerate() {
            for (x, entity) in row.iter().enumerate() {
                if pos != (y, x) {
                    if let Entity::Being(a) = self.entity(pos) {
                        if let Entity::Being(b) = entity {
                            if a.kind != b.kind {
                                enemies.push((y, x));
                            }
                        }
                    }
                }
            }
        }
        enemies
    }

    fn fighters(&self) -> Vec<(usize, usize)> {
        let mut fighters = vec![];
        for (y, row) in self.map.iter().enumerate() {
            for (x, entity) in row.iter().enumerate() {
                if let Entity::Being(_) = entity {
                    fighters.push((y, x));
                }
            }
        }
        fighters
    }

    fn fighter(&self, id: (usize, usize)) -> (usize, usize) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, entity) in row.iter().enumerate() {
                if let Entity::Being(b) = entity {
                    if b.id == id {
                        return (y, x);
                    }
                }
            }
        }
        panic!();
    }

    fn round(&mut self) -> (bool, bool) {
        let fighter_ids : Vec<(usize, usize)> = self.fighters().iter().map(|f| if let Entity::Being(x) = self.entity(*f) { x.id } else { panic!() }).collect();
        let mut elf_died = false;
        let mut already_dead = HashSet::new();
        for id in &fighter_ids {
            if already_dead.contains(id) {
                // Already dead
//                println!("{:?} already dead", fa);
                continue;
            }
            let mut fighter = self.fighter(*id);
            // Do i have an enemy in range?
            let mut enemies_to_fight = self.enemies_in_range(fighter);
            if enemies_to_fight.len() == 0 {
                // Find my enemies
                let mut enemies = self.enemies(fighter);
                if enemies.len() == 0 {
                    // Combat is over
                    return (true, elf_died);
                }
                // Find the closest open square adjacent to an enemy
                let mut shortest = std::usize::MAX;
                enemies.sort_by(|a, b| manhattan(*a, fighter).cmp(&manhattan(*b, fighter)));
                let mut all_paths = vec![];
                for fb in &enemies {
                    // Find paths to all free spaces next to the enemy
                    for adj in self.neighbours(*fb, |c| self.map[c.0][c.1] == Entity::Floor) {
                        if manhattan(fighter, adj) > shortest {
                            continue;
                        }
                        if let Some(paths) = shortest_path(self, fighter, adj) {
                            all_paths.extend(paths);
                        }
                    }
                }
                if all_paths.len() > 0 {
                    all_paths.sort_by(|a, b| a.len().cmp(&b.len()).then_with(|| a[a.len()-1].cmp(&b[b.len()-1])).then_with(|| a[1].cmp(&b[1])));
                    // for p in &all_paths {
                    //     println!("path: {:?}", p);
                    // }
                    // Move
                    let (y, x) = fighter;
                    let (ny, nx) = all_paths[0][1];
                    assert!(self.map[ny][nx] == Entity::Floor);
                    self.map[ny][nx] = self.map[y][x];
                    self.map[y][x] = Entity::Floor;
                    fighter = (ny, nx);
                    //println!("moving {:?} > {:?}", (y, x), (ny, nx));
                    // After moving, we might have some in range enemies
                    enemies_to_fight = self.enemies_in_range(fighter);
                }
            }
            // Fight!
            if let Entity::Being(_) = self.entity(fighter) {
                // Find the weakest enemy
                let mut close_enemies = vec![];
                for close_enemy in enemies_to_fight {
                    if let Entity::Being(x) = self.entity(close_enemy) {
                        close_enemies.push((x.hp, close_enemy));
                    } else {
                        panic!();
                    }
                }
                if close_enemies.len() > 0 {
                    close_enemies.sort();
                    let enemy = close_enemies[0].1;
                    let (y, x) = fighter;
                    if let Entity::Being(attacker) = self.map[y][x] {
                        let (yy, xx) = enemy;
                        let mut dead = false;
                        if let Entity::Being(attackee) = &mut self.map[yy][xx] {
                            //println!("attack: {:?} -> {:?}", attacker, attackee);
                            if attackee.hp > attacker.attack {
                                attackee.hp -= attacker.attack;
                            } else {
                                dead = true;
                                already_dead.insert(attackee.id);
                                if attackee.kind == FighterKind::Elf {
                                    elf_died = true;
                                }
                            }
                        }
                        if dead {
                            // die!
                            //println!("die: {:?}, {:?}", (yy, xx), self.entity((yy, xx)));
                            self.map[yy][xx] = Entity::Floor;
                        }
                    }
                }
            }
        }
        (false, elf_died)
    }

    fn draw(&self) {
        for _ in 0..self.map[0].len() {
            print!("-");
        }
        println!("");
        for row in &self.map {
            let mut entities : Vec<Entity> = vec![];
            for col in row {
                let s = match col {
                    Entity::Being(x) => {
                        entities.push(*col);
                        match x.kind {
                            FighterKind::Elf => "E",
                            FighterKind::Goblin => "G"
                        }
                    },
                    Entity::Wall => "#",
                    Entity::Floor => "."
                };
                print!("{}", s);
            }
            if entities.len() > 0 {
                for e in entities {
                    match e {
                        Entity::Being(b) => {
                            match b.kind {
                                FighterKind::Elf => {
                                    print!(" E({})", b.hp)
                                },
                                FighterKind::Goblin => {
                                    print!(" G({})", b.hp)
                                }
                            }
                        },
                        _ => {}
                    }
                }
                println!("");
            } else {
                println!("");
            }
        }
    }

    fn outcome(&self) -> i64 {
        let mut sum = 0;
        for row in &self.map {
            for col in row {
                if let Entity::Being(x) = col {
                    println!("{:?}", x);
                    sum += x.hp;
                }
            }
        }
        sum
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
    let mut elf_power = 3;
    loop {
        println!("elf_power: {}", elf_power);
        let mut m = vec![];
        for (y, row) in grid.iter().enumerate() {
            let mut map_row = vec![];
            for (x, col) in row.iter().enumerate() {
                let entity = match col {
                    '#' => Entity::Wall,
                    '.' => Entity::Floor,
                    'E' => Entity::Being(Fighter { kind: FighterKind::Elf, attack: elf_power, hp: 200, id: (y, x) }),
                    'G' => Entity::Being(Fighter { kind: FighterKind::Goblin, attack: 3, hp: 200, id: (y, x) }),
                    _ => panic!()
                };
                map_row.push(entity);
            }
            m.push(map_row);
        }
        let mut map = Map { map: m };

        let mut rounds = 0;
        loop {
            let (done, elf_died) = map.round();
            if !done {
                rounds += 1;
                if elf_power == 20 || elf_power == 3 {
                    println!("After round {}", rounds);
                    map.draw();
                }
            }
            if elf_died {
                map.draw();
                let sum = map.outcome();
                println!("{}, {}, {}, {}", elf_power, rounds, sum, rounds * sum);
                break;
            }
            if done {
                map.draw();
                let sum = map.outcome();
                println!("{}, {}, {}, {}", elf_power, rounds, sum, rounds * sum);
                return;
            }
        }
        elf_power += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    solve(Path::new(&filename));
}
