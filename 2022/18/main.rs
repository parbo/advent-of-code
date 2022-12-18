use std::{collections::HashSet, iter::*};

use aoc::{vec_add, Vec3};

const NEIGHBORS: [Vec3; 6] = [
    [-1, 0, 0],
    [0, -1, 0],
    [1, 0, 0],
    [0, 1, 0],
    [0, 0, -1],
    [0, 0, 1],
];

fn area(droplet: &HashSet<Vec3>) -> i64 {
    let mut exposed = 0;
    for cube in droplet {
        for nb in NEIGHBORS {
            let p = vec_add(*cube, nb);
            if !droplet.contains(&p) {
                exposed += 1;
            }
        }
    }
    exposed
}

fn fill(
    droplet: &HashSet<Vec3>,
    pos: Vec3,
    extents: ([i64; 3], [i64; 3]),
) -> Option<HashSet<Vec3>> {
    let ([min_x, min_y, min_z], [max_x, max_y, max_z]) = extents;
    if !droplet.contains(&pos) {
        let mut ret = droplet.clone();
        let mut todo = vec![];
        todo.push(pos);
        while let Some(p) = todo.pop() {
            if p[0] < min_x
                || p[0] > max_x
                || p[1] < min_y
                || p[1] > max_y
                || p[2] < min_z
                || p[2] > max_z
            {
                return None;
            }
            if ret.insert(p) {
                for nb in NEIGHBORS {
                    todo.push(vec_add(p, nb));
                }
            }
        }
        Some(ret)
    } else {
        None
    }
}

#[cfg(feature = "vis")]
fn draw(droplet: &HashSet<Vec3>) {
    use std::path::PathBuf;

    let mut window = kiss3d::window::Window::new_with_size("Day 18", 400, 400);
    window.set_light(kiss3d::light::Light::StickToCamera);
    let eye = kiss3d::nalgebra::Point3::new(40.0f32, 0.0, 80.0);
    let at = kiss3d::nalgebra::Point3::origin();
    let mut camera = kiss3d::camera::ArcBall::new(eye, at);
    let mut frame = 0;
    let png_path = PathBuf::from("vis/18/part1");
    if let Some(parent) = png_path.parent() {
        std::fs::create_dir_all(parent).expect("could not create folder");
    }

    let maxx = droplet.iter().map(|p| p[0]).max().unwrap() as f32;
    let maxy = droplet.iter().map(|p| p[1]).max().unwrap() as f32;
    let maxz = droplet.iter().map(|p| p[2]).max().unwrap() as f32;
    for cube in droplet {
        let sc = 2.0;
        let mut c = window.add_cube(sc, sc, sc);
        c.set_color(1.0, 0.3, 0.3);
        c.append_translation(
            &[
                sc * cube[0] as f32 - maxx,
                sc * cube[1] as f32 - maxy,
                sc * cube[2] as f32 - maxz,
            ]
            .into(),
        );
    }

    let orig_curr_yaw = camera.yaw();
    while window.render_with_camera(&mut camera) {
        // rotate the arc-ball camera.
        let curr_yaw = camera.yaw();
        let new_yaw = curr_yaw + 0.01;
        if new_yaw > orig_curr_yaw + std::f32::consts::PI {
            break;
        }
        camera.set_yaw(new_yaw);
        // Save image
        let filename = png_path.parent().unwrap().join(&format!(
            "{}_{:06}.png",
            png_path.file_name().unwrap().to_str().unwrap(),
            frame
        ));
        let img = window.snap_image();
        img.save(filename).unwrap();
        frame += 1;
    }
}

fn part1(droplet: &HashSet<Vec3>) -> i64 {
    #[cfg(feature = "vis")]
    draw(droplet);
    area(droplet)
}

fn part2(droplet: &HashSet<Vec3>) -> i64 {
    let minx = droplet.iter().map(|p| p[0]).min().unwrap();
    let maxx = droplet.iter().map(|p| p[0]).max().unwrap();
    let miny = droplet.iter().map(|p| p[1]).min().unwrap();
    let maxy = droplet.iter().map(|p| p[1]).max().unwrap();
    let minz = droplet.iter().map(|p| p[2]).min().unwrap();
    let maxz = droplet.iter().map(|p| p[2]).max().unwrap();
    let mut d = droplet.clone();
    for x in minx..=maxx {
        for y in miny..=maxy {
            for z in minz..=maxz {
                if let Some(dd) = fill(&d, [x, y, z], ([minx, miny, minz], [maxx, maxy, maxz])) {
                    d = dd;
                }
            }
        }
    }
    area(&d)
}

fn parse(lines: &[String]) -> HashSet<Vec3> {
    lines
        .iter()
        .map(|x| {
            let v = aoc::things::<i64>(x);
            [v[0], v[1], v[2]]
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "2,2,2".into(),
            "1,2,2".into(),
            "3,2,2".into(),
            "2,1,2".into(),
            "2,3,2".into(),
            "2,2,1".into(),
            "2,2,3".into(),
            "2,2,4".into(),
            "2,2,6".into(),
            "1,2,5".into(),
            "3,2,5".into(),
            "2,1,5".into(),
            "2,3,5".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 58);
    }
}
