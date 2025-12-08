use std::{cmp::Reverse, collections::BinaryHeap, iter::*};

use aoc::{FxHashMap, FxHashSet};

#[cfg(feature = "vis")]
use std::path::PathBuf;

#[cfg(feature = "vis")]
use aoc::Itertools;

#[derive(
    parse_display::Display, parse_display::FromStr, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[display("{x},{y},{z}")]
struct Junction {
    x: i64,
    y: i64,
    z: i64,
}

type ParsedItem = Junction;
type Parsed = Vec<ParsedItem>;

#[cfg(feature = "vis")]
struct Drawer {
    window: kiss3d::window::Window,
    camera: kiss3d::camera::ArcBall,
    spheres: Vec<kiss3d::scene::SceneNode>,
    data: Parsed,
}

#[cfg(feature = "vis")]
impl Drawer {
    fn new(data: &[Junction]) -> Self {
        let mut window = kiss3d::window::Window::new_with_size("Day 8", 800, 800);
        window.set_light(kiss3d::light::Light::StickToCamera);
        let eye = kiss3d::nalgebra::Point3::new(50.0, 300.0, 50.0);
        let at = kiss3d::nalgebra::Point3::new(50.0, 50.0, 50.0);
        let mut camera = kiss3d::camera::ArcBall::new(eye, at);
        camera.set_up_axis(kiss3d::nalgebra::Vector3::new(0.0, 0.0, 1.0));
        let mut spheres = vec![];
        for e in data {
            let mut c = window.add_sphere(1.0);
            c.set_color(1.0, 0.0, 0.0);
            c.append_translation(&kiss3d::nalgebra::Translation3::new(
                e.x as f32 / 1000.0,
                e.y as f32 / 1000.0,
                e.z as f32 / 1000.0,
            ));
            spheres.push(c);
        }
        Drawer {
            window,
            camera,
            spheres,
            data: data.to_vec(),
        }
    }

    fn draw(&mut self, chains: &[FxHashSet<usize>], ix: usize) {
        for c in chains.iter().take(ix) {
            for (i, j) in c.iter().tuple_windows() {
                self.window.draw_line(
                    &kiss3d::nalgebra::Point3::new(
                        self.data[*i].x as f32 / 1000.0,
                        self.data[*i].y as f32 / 1000.0,
                        self.data[*i].z as f32 / 1000.0,
                    ),
                    &kiss3d::nalgebra::Point3::new(
                        self.data[*j].x as f32 / 1000.0,
                        self.data[*j].y as f32 / 1000.0,
                        self.data[*j].z as f32 / 1000.0,
                    ),
                    &kiss3d::nalgebra::Point3::new(0.0, 1.0, 0.0),
                );
            }
        }
    }

    fn render(&mut self, chains: Vec<FxHashSet<usize>>) {
        let png_path = PathBuf::from("vis/08/part2");
        if let Some(parent) = png_path.parent() {
            std::fs::create_dir_all(parent).expect("could not create folder");
        }
        let mut frame = 0;
        while futures::executor::block_on(self.window.render_with_camera(&mut self.camera)) {
            println!("{}/{}", frame, chains.len());
            self.draw(&chains, frame);
            // Save image
            let filename = png_path.parent().unwrap().join(&format!(
                "{}_{:06}.png",
                png_path.file_name().unwrap().to_str().unwrap(),
                frame
            ));
            let img = self.window.snap_image();
            img.save(filename).unwrap();
            frame += 1;
            if frame >= chains.len() {
                break;
            }
            let curr_yaw = self.camera.yaw();
            self.camera.set_yaw(curr_yaw + 0.01);
        }
    }
}

fn dist(n: Junction, goal: Junction) -> i64 {
    (goal.x - n.x).pow(2) + (goal.y - n.y).pow(2) + (goal.z - n.z).pow(2)
}

fn distances(data: &Parsed) -> BinaryHeap<Reverse<(i64, usize, usize)>> {
    let mut heap = BinaryHeap::new();
    for i in 0..data.len() {
        for j in 0..data.len() {
            if i == j {
                continue;
            }
            let d = dist(data[i], data[j]);
            heap.push(Reverse((d, i, j)))
        }
    }
    heap
}

fn solve(data: &Parsed, num: usize) -> i64 {
    let mut connections: FxHashMap<usize, FxHashSet<usize>> = FxHashMap::default();
    let mut dd = distances(data);
    let mut n = 0;
    while n < num {
        if let Some(Reverse((_d, i, j))) = dd.pop() {
            if connections.entry(i).or_default().contains(&j) {
                continue;
            }
            n += 1;
            connections.entry(i).or_default().insert(j);
            connections.entry(j).or_default().insert(i);
        }
    }
    let mut seen: FxHashSet<usize> = FxHashSet::default();
    let mut chains = vec![];
    for i in 0..data.len() {
        let mut todo = vec![i];
        let mut chain = vec![i];
        while let Some(x) = todo.pop() {
            let e = connections.entry(x).or_default();
            seen.insert(x);
            for k in e.iter() {
                if seen.contains(k) {
                    continue;
                }
                todo.push(*k);
                chain.push(*k);
                seen.insert(*k);
            }
        }
        chains.push(chain);
    }
    chains.sort_by_key(|a| Reverse(a.len()));
    chains.iter().take(3).map(|x| x.len()).product::<usize>() as i64
}

fn part1(data: &Parsed) -> i64 {
    solve(data, 1000)
}

fn part2(data: &Parsed) -> i64 {
    let mut connections: FxHashMap<usize, FxHashSet<usize>> = FxHashMap::default();
    let mut dd = distances(data);
    #[cfg(feature = "vis")]
    let mut chains = vec![];
    loop {
        if let Some(Reverse((_d, i, j))) = dd.pop() {
            if connections.entry(i).or_default().contains(&j) {
                continue;
            }
            connections.entry(i).or_default().insert(j);
            connections.entry(j).or_default().insert(i);
            let mut seen: FxHashSet<usize> = FxHashSet::default();
            let mut todo = vec![i];
            while let Some(x) = todo.pop() {
                let e = connections.entry(x).or_default();
                seen.insert(x);
                for k in e.iter() {
                    if seen.contains(k) {
                        continue;
                    }
                    todo.push(*k);
                    seen.insert(*k);
                }
            }
            #[cfg(feature = "vis")]
            chains.push(seen.clone());

            if seen.len() == data.len() {
                #[cfg(feature = "vis")]
                {
                    let mut d = Drawer::new(data);
                    d.render(chains);
                }
                return data[i].x * data[j].x;
            }
        }
    }
}

fn parse(lines: &[String]) -> Parsed {
    lines.iter().map(|x| x.parse().unwrap()).collect()
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
        assert_eq!(solve(&parse(&example()), 10), 40);
    }
}
