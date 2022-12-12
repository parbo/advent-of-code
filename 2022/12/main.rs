use std::{iter::*, path::PathBuf};

use aoc::Grid;

type Parsed = Vec<Vec<char>>;
type Answer = i64;

fn part1(data: &Parsed) -> Answer {
    let start = data
        .points()
        .map(|x| (x, data.get_value(x).unwrap()))
        .find(|x| x.1 == 'S')
        .unwrap()
        .0;
    let goal = data
        .points()
        .map(|x| (x, data.get_value(x).unwrap()))
        .find(|x| x.1 == 'E')
        .unwrap()
        .0;
    let p = aoc::astar_grid(
        data,
        |_p, _c| true,
        |_p1, c1, _p2, c2| {
            let h = if *c2 == 'E' { b'z' } else { *c2 as u8 };
            if *c1 as u8 + 1 >= h || *c1 == 'S' {
                Some(1)
            } else {
                None
            }
        },
        start,
        goal,
    )
    .unwrap();

    #[cfg(feature = "vis")]
    {
        let mut window = kiss3d::window::Window::new_with_size("Day 12", 1280, 720);

        window.set_light(kiss3d::light::Light::StickToCamera);

        let path = p.1.iter().copied().rev().collect::<Vec<_>>();

        let hf = |v: char| {
            (if v == 'E' {
                b'z'
            } else if v == 'S' {
                b'a'
            } else {
                v as u8
            }) - b'a'
        };

        let w = data[0].len();
        let h = data.len();

        let scale = 8;

        // Scale the heightmap with gaussian
        let mut hm = image::GrayImage::new(w as u32, h as u32);
        for [x, y] in data.points() {
            let v = data.get_value([x, y]).unwrap();
            let h = hf(v) as f32;
            let c = 255.0 * h / 26.0;
            hm.put_pixel(x as u32, y as u32, image::Luma([c as u8]));
        }
        let hm = image::imageops::resize(
            &hm,
            (w * scale) as u32,
            (h * scale) as u32,
            image::imageops::FilterType::Gaussian,
        );

        for y in 0..h {
            for x in 0..w {
                let mut vertices = vec![];
                for xx in 0..(scale + 1) {
                    for yy in 0..(scale + 1) {
                        let xxx = (x * scale + xx) as u32;
                        let yyy = (y * scale + yy) as u32;
                        let image::Luma(hp) =
                            hm.get_pixel_checked(xxx, yyy).unwrap_or(&image::Luma([0]));
                        let hp = hp[0] as f32;
                        let xp = xx as f32;
                        let yp = yy as f32;
                        vertices.push(kiss3d::nalgebra::Point3::new(
                            xp / scale as f32,
                            26.0 * hp / 255.0,
                            yp / scale as f32,
                        ));
                    }
                }
                let mut q = window.add_quad_with_vertices(&vertices, scale + 1, scale + 1);
                q.enable_backface_culling(true);
                q.append_translation(&[x as f32 - 0.5, 0.0, y as f32 - 0.5].into());
                q.recompute_normals();
                let on_path = path.contains(&[x as i64, y as i64]);
                if on_path {
                    q.set_color(1.0, 1.0, 0.0);
                } else {
                    q.set_color(0.0, 1.0, 0.0);
                }
            }
        }

        let mut i = 0;
        let eye = kiss3d::nalgebra::Point3::new(path[i][0] as f32, 0.7, path[i][1] as f32);
        let at = kiss3d::nalgebra::Point3::new(path[i + 1][0] as f32, 0.7, path[i + 1][1] as f32);
        let mut camera = kiss3d::camera::FirstPerson::new(eye, at);
        let mut frame = 0;
        let png_path = PathBuf::from("vis/12/part1");
        if let Some(parent) = png_path.parent() {
            std::fs::create_dir_all(parent).expect("could not create folder");
        }
        let speed = 5;
        while window.render_with_camera(&mut camera) {
            if i + 2 < path.len() {
                if frame % speed == 0 {
                    i += 1;
                }
                // Step camera
                let v1 = data.get_value(path[i]).unwrap();
                let h1 = hf(v1) as f32 + 0.7;
                let v2 = data.get_value(path[i + 1]).unwrap();
                let h2 = hf(v2) as f32 + 0.7;
                let f = 2;
                let v3 = data.get_value(path[(i + f).min(path.len() - 1)]).unwrap();
                let h3 = hf(v3) as f32 + 0.7;
                let v4 = data
                    .get_value(path[(i + f + 1).min(path.len() - 1)])
                    .unwrap();
                let h4 = hf(v4) as f32 + 0.7;
                let x1 = path[i][0] as f32;
                let y1 = path[i][1] as f32;
                let x2 = path[i + 1][0] as f32;
                let y2 = path[i + 1][1] as f32;
                let x3 = path[(i + f).min(path.len() - 1)][0] as f32;
                let y3 = path[(i + f).min(path.len() - 1)][1] as f32;
                let x4 = path[(i + f + 1).min(path.len() - 1)][0] as f32;
                let y4 = path[(i + f + 1).min(path.len() - 1)][1] as f32;
                let d = (frame % speed) as f32 / speed as f32;
                let eye = kiss3d::nalgebra::Point3::new(
                    x1 + d * (x2 - x1),
                    h1 + d * (h2 - h1),
                    y1 + d * (y2 - y1),
                );
                let at = kiss3d::nalgebra::Point3::new(
                    x3 + d * (x4 - x3),
                    h3 + d * (h4 - h3),
                    y3 + d * (y4 - y3),
                );
                camera.look_at(eye, at);
            }
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
    p.0
}

fn part2(data: &Parsed) -> Answer {
    let starts = data
        .points()
        .map(|x| (x, data.get_value(x).unwrap()))
        .filter_map(|x| if x.1 == 'a' { Some(x.0) } else { None })
        .collect::<Vec<_>>();
    let goal = data
        .points()
        .map(|x| (x, data.get_value(x).unwrap()))
        .find(|x| x.1 == 'E')
        .unwrap()
        .0;
    starts
        .iter()
        .filter_map(|start| {
            if let Some(p) = aoc::astar_grid(
                data,
                |_p, _c| true,
                |_p1, c1, _p2, c2| {
                    let h = if *c2 == 'E' { b'z' } else { *c2 as u8 };
                    if *c1 as u8 + 1 >= h || *c1 == 'S' {
                        Some(1)
                    } else {
                        None
                    }
                },
                *start,
                goal,
            ) {
                Some(p.0)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid(lines)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "Sabqponm".into(),
            "abcryxxl".into(),
            "accszExk".into(),
            "acctuvwj".into(),
            "abdefghi".into(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&example())), 29);
    }
}
