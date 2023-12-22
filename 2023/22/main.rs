use std::{iter::*, path::PathBuf};

use aoc::Vec3;

type ParsedItem = (Vec3, Vec3);
type Parsed = Vec<ParsedItem>;

struct Drawer {
    window: kiss3d::window::Window,
    camera: kiss3d::camera::ArcBall,
    cubes: Vec<kiss3d::scene::SceneNode>,
    data: Vec<Parsed>,
}

impl Drawer {
    fn new(data: Vec<Parsed>) -> Self {
        let mut window = kiss3d::window::Window::new_with_size("Day 22", 100, 1200);
        window.set_light(kiss3d::light::Light::StickToCamera);
        let eye = kiss3d::nalgebra::Point3::new(0.0, 400.0, 150.0);
        let at = kiss3d::nalgebra::Point3::new(0.0, 0.0, 150.0);
        // let at = kiss3d::nalgebra::Point3::origin();
        let mut camera = kiss3d::camera::ArcBall::new(eye, at);
        camera.set_up_axis(kiss3d::nalgebra::Vector3::new(0.0, 0.0, 1.0));
        let cubes = vec![];
        Drawer {
            window,
            camera,
            cubes,
            data,
        }
    }

    fn draw(&mut self, ix: usize) {
        for c in &mut self.cubes {
            self.window.remove_node(c);
        }
        self.cubes.clear();
        for (s, e) in &self.data[ix] {
            let mut c = self.window.add_cube(
                (e[1] - s[1] + 1) as f32,
                (e[0] - s[0] + 1) as f32,
                (e[2] - s[2] + 1) as f32,
            );
            c.set_color(1.0, 1.0, 0.0);
            c.append_translation(&kiss3d::nalgebra::Translation3::new(
                s[1] as f32,
                s[0] as f32,
                s[2] as f32,
            ));
            self.cubes.push(c);
        }
    }

    fn render(&mut self) {
        let mut frame = 0;
        let mut ix = 0;
        let png_path = PathBuf::from("vis/22/part1");
        if let Some(parent) = png_path.parent() {
            std::fs::create_dir_all(parent).expect("could not create folder");
        }
        while self.window.render_with_camera(&mut self.camera) {
            println!("{}/{}", frame, self.data.len());
            self.draw(ix);
            // Save image
            let filename = png_path.parent().unwrap().join(&format!(
                "{}_{:06}.png",
                png_path.file_name().unwrap().to_str().unwrap(),
                frame
            ));
            let img = self.window.snap_image();
            img.save(filename).unwrap();
            frame += 1;
            ix += 100;
            if ix > self.data.len() {
                break;
            }
            let curr_yaw = self.camera.yaw();
            self.camera.set_yaw(curr_yaw + 0.05);
            // dbg!(&data);
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn collapse(data: &Parsed, mut drawer: impl FnMut(&Parsed)) -> (Vec<(Vec3, Vec3)>, usize) {
    let mut data = data.to_vec();
    // sort on descending z
    data.sort_by(|a, b| b.0[2].cmp(&a.0[2]));
    // fall down
    let n = data.len();
    let mut num = 0;
    for i in (0..n).rev() {
        // dbg!(i);
        let d = data[i];
        let istart = [d.0[0], d.0[1]];
        let iend = [d.1[0], d.1[1]];
        let mut max_z = 0;
        // let mut candidates = vec![];
        for j in (i + 1)..n {
            let dj = data[j];
            let jstart = [dj.0[0], dj.0[1]];
            let jend = [dj.1[0], dj.1[1]];
            let min_xx = istart[0].max(jstart[0]);
            let min_yy = istart[1].max(jstart[1]);
            let max_xx = iend[0].min(jend[0]);
            let max_yy = iend[1].min(jend[1]);
            if min_xx <= max_xx && min_yy <= max_yy {
                let z = dj.1[2] + 1;
                if z >= max_z {
                    max_z = z;
                }
            }
        }

        let diff = d.0[2] - max_z;
        if diff > 0 {
            for _ in 0..diff {
                {
                    let d = &mut data[i];
                    d.0[2] -= 1;
                    d.1[2] -= 1;
                }
                drawer(&data);
            }
            num += 1;
        }
    }
    (data, num)
}

fn part1(data: &Parsed) -> i64 {
    let mut vis = vec![];
    let (data, _) = collapse(data, |x| vis.push(x.clone()));

    let mut actually_safe = vec![];
    for s in 0..data.len() {
        let dd = data
            .iter()
            .enumerate()
            .filter_map(|(i, d)| if i == s { None } else { Some(*d) })
            .collect();

        let nd = collapse(&dd, |_| {});
        if nd.0 == dd {
            actually_safe.push(s);
        }
    }

    let mut d = Drawer::new(vis);
    d.render();

    actually_safe.len() as i64
}

fn part2(data: &Parsed) -> i64 {
    let (data, _) = collapse(data, |_| {});

    let mut collapsed = 0;
    for s in 0..data.len() {
        let dd = data
            .iter()
            .enumerate()
            .filter_map(|(i, d)| if i == s { None } else { Some(*d) })
            .collect();

        let nd = collapse(&dd, |_| {});
        if nd.0 != dd {
            collapsed += nd.1
        }
    }
    collapsed as i64
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let (a, b) = x.split_once('~').unwrap();
            let a = aoc::things::<i64>(a);
            let b = aoc::things::<i64>(b);
            (a[0..3].try_into().unwrap(), b[0..3].try_into().unwrap())
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn example() -> Vec<String> {
    //     let s = include_str!("example.txt");
    //     s.lines().map(|x| x.to_string()).collect()
    // }

    // fn example() -> Vec<String> {
    // 	   vec![
    //         "0".into()
    //     ]
    // }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&parse(&example())), 0);
    // }
}
