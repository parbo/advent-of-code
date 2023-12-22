use std::iter::*;

use aoc::{FxHashMap, Vec3};

type ParsedItem = (Vec3, Vec3);
type Parsed = Vec<ParsedItem>;

fn draw(data: &Parsed) {
    let mut window = kiss3d::window::Window::new_with_size("Day 12", 1280, 720);

    for (s, e) in data {
        let mut c = window.add_cube(
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
    }

    window.set_light(kiss3d::light::Light::StickToCamera);
    let eye = kiss3d::nalgebra::Point3::new(0.0, 0.0, 80.0);
    let at = kiss3d::nalgebra::Point3::origin();
    let mut camera = kiss3d::camera::ArcBall::new(eye, at);
    // let mut camera = kiss3d::camera::FirstPerson::new(eye, at);
    // let mut frame = 0;
    // let png_path = PathBuf::from("vis/12/part1");
    // if let Some(parent) = png_path.parent() {
    //     std::fs::create_dir_all(parent).expect("could not create folder");
    // }
    while window.render_with_camera(&mut camera) {
        // rotate the arc-ball camera.
        // let curr_yaw = camera.yaw();
        // camera.set_yaw(curr_yaw + 0.01);
    }
    // // Save image
    // let filename = png_path.parent().unwrap().join(&format!(
    //     "{}_{:06}.png",
    //     png_path.file_name().unwrap().to_str().unwrap(),
    //     frame
    // ));
    // let img = window.snap_image();
    // img.save(filename).unwrap();
    // frame += 1;
    // dbg!(&data);
}

type SupportMap = FxHashMap<usize, Vec<usize>>;

fn collapse(data: &Parsed) -> (Vec<(Vec3, Vec3)>, SupportMap, SupportMap, usize) {
    let mut data: Vec<_> = data.iter().cloned().enumerate().collect();
    // draw(&data);
    // sort on descending z
    data.sort_by(|a, b| b.1 .0[2].cmp(&a.1 .0[2]));
    // draw(&data);
    // fall down
    let mut supports: SupportMap = FxHashMap::default();
    let mut supported_by: SupportMap = FxHashMap::default();
    // dbg!(data.len());
    let n = data.len();
    let mut num = 0;
    for i in (0..n).rev() {
        // dbg!(i);
        let (dix, d) = data[i];
        let istart = [d.0[0], d.0[1]];
        let iend = [d.1[0], d.1[1]];
        let mut max_z = 0;
        for j in (i + 1)..n {
            let (djix, dj) = data[j];
            let jstart = [dj.0[0], dj.0[1]];
            let jend = [dj.1[0], dj.1[1]];
            let min_xx = istart[0].max(jstart[0]);
            let min_yy = istart[1].max(jstart[1]);
            let max_xx = iend[0].min(jend[0]);
            let max_yy = iend[1].min(jend[1]);
            if min_xx <= max_xx && min_yy <= max_yy {
                // println!(
                //     "overlap {} {}, {:?} {:?}, {} {} {} {}",
                //     i, j, d, dj, min_xx, min_yy, max_xx, max_yy
                // );
                let z = dj.1[2] + 1;
                if z >= max_z {
                    supports.entry(djix).or_default().push(dix);
                    supported_by.entry(dix).or_default().push(djix);
                    max_z = z;
                }
            }
        }
        let diff = d.0[2] - max_z;
        if diff > 0 {
            let (_, d) = &mut data[i];
            d.0[2] -= diff;
            d.1[2] -= diff;
            num += 1;
        }
        // dbg!(d);
    }
    // sort back to original order
    data.sort_by(|a, b| a.0.cmp(&b.0));
    (
        data.into_iter().map(|(_, d)| d).collect(),
        supports,
        supported_by,
        num,
    )
}

fn part1(data: &Parsed) -> i64 {
    let (data, supports, supported_by, _) = collapse(data);

    dbg!(&supports);
    dbg!(&supported_by);

    let mut safe = vec![];
    for i in 0..data.len() {
        if let Some(ss) = supports.get(&i) {
            dbg!(i);
            let mut num = 0;
            for si in ss {
                if let Some(sb) = supported_by.get(si) {
                    if sb.len() > 1 {
                        num += 1;
                    }
                }
            }
            if num == ss.len() {
                safe.push(i);
            }
        } else {
            safe.push(i);
        }
    }
    dbg!(&safe);

    for s in &safe {
        let dd = data
            .iter()
            .enumerate()
            .filter_map(|(i, d)| if i == *s { None } else { Some(*d) })
            .collect();

        let nd = collapse(&dd);
        if nd.0 != dd {
            println!("OH NOES, {} was not actually safe", s);
        }
    }

    let mut actually_safe = vec![];
    for s in 0..data.len() {
        let dd = data
            .iter()
            .enumerate()
            .filter_map(|(i, d)| if i == s { None } else { Some(*d) })
            .collect();

        let nd = collapse(&dd);
        if nd.0 == dd {
            actually_safe.push(s);
        }
    }

    actually_safe.len() as i64
}

fn part2(data: &Parsed) -> i64 {
    let (data, supports, supported_by, _) = collapse(data);

    let mut collapsed = 0;
    for s in 0..data.len() {
        let dd = data
            .iter()
            .enumerate()
            .filter_map(|(i, d)| if i == s { None } else { Some(*d) })
            .collect();

        let nd = collapse(&dd);
        if nd.0 != dd {
            collapsed += nd.3
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
