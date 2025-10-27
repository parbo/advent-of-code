use aoc::{vec4_add, vec_add, Vec3, Vec4};
use std::collections::HashMap;
use std::iter::*;

type Parsed = HashMap<Vec3, char>;
type Answer = usize;

fn extents(g: &Parsed) -> (Vec3, Vec3) {
    let min_x = g.keys().map(|p| p[0]).min().unwrap();
    let min_y = g.keys().map(|p| p[1]).min().unwrap();
    let min_z = g.keys().map(|p| p[2]).min().unwrap();
    let max_x = g.keys().map(|p| p[0]).max().unwrap();
    let max_y = g.keys().map(|p| p[1]).max().unwrap();
    let max_z = g.keys().map(|p| p[2]).max().unwrap();
    ([min_x, min_y, min_z], [max_x, max_y, max_z])
}

fn extents4(g: &HashMap<Vec4, char>) -> (Vec4, Vec4) {
    let min_x = g.keys().map(|p| p[0]).min().unwrap();
    let min_y = g.keys().map(|p| p[1]).min().unwrap();
    let min_z = g.keys().map(|p| p[2]).min().unwrap();
    let min_w = g.keys().map(|p| p[3]).min().unwrap();
    let max_x = g.keys().map(|p| p[0]).max().unwrap();
    let max_y = g.keys().map(|p| p[1]).max().unwrap();
    let max_z = g.keys().map(|p| p[2]).max().unwrap();
    let max_w = g.keys().map(|p| p[3]).max().unwrap();
    ([min_x, min_y, min_z, min_w], [max_x, max_y, max_z, max_w])
}

fn dirs() -> Vec<Vec3> {
    let mut d = vec![];
    for z in -1..=1 {
        for y in -1..=1 {
            for x in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                d.push([x, y, z]);
            }
        }
    }
    d
}

fn dirs4() -> Vec<Vec4> {
    let mut d = vec![];
    for w in -1..=1 {
        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
                    d.push([x, y, z, w]);
                }
            }
        }
    }
    d
}

#[allow(dead_code)]
fn print_slices(g: &Parsed) {
    let ([min_x, min_y, min_z], [max_x, max_y, max_z]) = extents(g);
    for z in min_z..=max_z {
        println!("z={}", z);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!("{}", g.get(&[x, y, z]).unwrap_or(&'.'));
            }
            println!();
        }
        println!();
    }
}

fn step(g: &Parsed, d: &[Vec3]) -> Parsed {
    let mut newg = g.clone();
    let ([min_x, min_y, min_z], [max_x, max_y, max_z]) = extents(&newg);
    for z in (min_z - 1)..=(max_z + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            for x in (min_x - 1)..=(max_x + 1) {
                let p = [x, y, z];
                let mut active = 0;
                let c = g.get(&p).unwrap_or(&'.');
                for dir in d {
                    let np = vec_add(p, *dir);
                    if let Some('#') = g.get(&np) {
                        active += 1;
                    }
                }
                if *c == '#' && !(active == 2 || active == 3) {
                    newg.remove(&p);
                } else if *c == '.' && active == 3 {
                    newg.insert(p, '#');
                }
            }
        }
    }
    newg
}

#[allow(dead_code)]
fn part1(input: &Parsed) -> Answer {
    let d = dirs();
    let mut i = 0;
    let mut g = input.clone();
    print_slices(&g);
    loop {
        let newg = step(&g, &d);
        println!("After {} cycle:", i);
        print_slices(&newg);
        if i == 6 {
            break;
        }
        i += 1;
        g = newg.clone();
    }
    g.iter().filter(|(_p, v)| **v == '#').count()
}

fn step4(g: &HashMap<Vec4, char>, d: &[Vec4]) -> HashMap<Vec4, char> {
    let mut newg = g.clone();
    let ([min_x, min_y, min_z, min_w], [max_x, max_y, max_z, max_w]) = extents4(&newg);
    for w in (min_w - 1)..=(max_w + 1) {
        for z in (min_z - 1)..=(max_z + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for x in (min_x - 1)..=(max_x + 1) {
                    let p = [x, y, z, w];
                    let mut active = 0;
                    let c = g.get(&p).unwrap_or(&'.');
                    for dir in d {
                        let np = vec4_add(p, *dir);
                        if let Some('#') = g.get(&np) {
                            active += 1;
                        }
                    }
                    if *c == '#' && !(active == 2 || active == 3) {
                        newg.remove(&p);
                    } else if *c == '.' && active == 3 {
                        newg.insert(p, '#');
                    }
                }
            }
        }
    }
    newg
}

#[allow(dead_code)]
fn part2(input: &Parsed) -> Answer {
    let d = dirs4();
    let mut i = 0;
    let mut g = HashMap::new();
    for ([x, y, z], v) in input {
        g.insert([*x, *y, *z, 0], *v);
    }
    loop {
        let newg = step4(&g, &d);
        if i == 6 {
            break;
        }
        i += 1;
        g = newg.clone();
    }
    g.iter().filter(|(_p, v)| **v == '#').count()
}

fn parse(lines: &[String]) -> Parsed {
    let mut g = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            g.insert([x as i64, y as i64, 0i64], c);
        }
    }
    g
}

#[cfg(feature = "vis")]
fn draw1(parsed: &Parsed) -> Answer {
    let mut window = kiss3d::window::Window::new_with_size("Day 17", 1280, 720);

    window.set_light(kiss3d::light::Light::StickToCamera);

    let d = dirs();
    let mut g = parsed.clone();
    let mut cubes = vec![];
    let eye = kiss3d::nalgebra::Point3::new(40.0f32, 15.0, 20.0);
    let at = kiss3d::nalgebra::Point3::origin();
    let mut camera = kiss3d::camera::ArcBall::new(eye, at);
    let mut frame = 0;
    while window.render_with_camera(&mut camera) {
        if frame % 20 == 0 {
            for mut c in cubes {
                window.remove_node(&mut c);
            }
            let mut new_cubes = vec![];
            for (p, v) in &g {
                if *v == '#' {
                    let mut c = window.add_cube(1.0, 1.0, 1.0);
                    c.append_translation(&kiss3d::nalgebra::Translation3::new(
                        p[0] as f32,
                        p[1] as f32,
                        p[2] as f32,
                    ));
                    c.set_color(0.0, 1.0, 0.0);
                    new_cubes.push(c);
                }
            }
            g = step(&g, &d);
            cubes = new_cubes;
        }
        // rotate the arc-ball camera.
        let curr_yaw = camera.yaw();
        camera.set_yaw(curr_yaw + 0.05);
        frame += 1;
    }
    0
}

#[cfg(feature = "vis")]
fn draw2(parsed: &Parsed) -> Answer {
    let mut window = kiss3d::window::Window::new_with_size("Day 17", 1280, 720);

    window.set_light(kiss3d::light::Light::StickToCamera);

    let d = dirs4();
    let mut g = HashMap::new();
    for ([x, y, z], v) in parsed {
        g.insert([*x, *y, *z, 0], *v);
    }
    let mut cubes = vec![];
    let eye = kiss3d::nalgebra::Point3::new(40.0f32, 15.0, 20.0);
    let at = kiss3d::nalgebra::Point3::origin();
    let mut camera = kiss3d::camera::ArcBall::new(eye, at);
    let mut frame = 0;
    while window.render_with_camera(&mut camera) {
        if frame % 20 == 0 {
            for mut c in cubes {
                window.remove_node(&mut c);
            }
            let mut new_cubes = vec![];
            for (p, v) in &g {
                if *v == '#' {
                    let mut c = window.add_cube(1.0, 1.0, 1.0);
                    c.append_translation(&kiss3d::nalgebra::Translation3::new(
                        p[0] as f32,
                        p[1] as f32,
                        p[2] as f32,
                    ));
                    let col = p[3] as f32 / (frame / 20 + 1) as f32;
                    c.set_color(0.0, col, 0.0);
                    new_cubes.push(c);
                }
            }
            g = step4(&g, &d);
            cubes = new_cubes;
        }
        // rotate the arc-ball camera.
        let curr_yaw = camera.yaw();
        camera.set_yaw(curr_yaw + 0.05);
        frame += 1;
    }
    0
}

#[cfg(feature = "vis")]
fn main() {
    aoc::run_main(parse, draw1, draw2);
}

#[cfg(not(feature = "vis"))]
fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![".#.".to_string(), "..#".to_string(), "###".to_string()];
        let parsed = parse(&input);
        assert_eq!(part1(&parsed), 112);
    }
}
