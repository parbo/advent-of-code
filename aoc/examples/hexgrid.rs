use aoc::HexGrid;
use aoc::HexGridDrawer;
use std::collections::HashMap;

fn print_char() {
    let mut grid = HashMap::new();
    let mut col = 0;
    let mut row = 0;
    for c in 'A'..='Z' {
        grid.insert(aoc::oddr_to_cube([col, row]), c);
        if col + 1 < 5 {
            col += 1;
        } else {
            col = 0;
            row += 1;
        }
    }
    let mut gd = aoc::PrintHexGridDrawer::new(|x| x);
    let mut gdb = aoc::BitmapHexGridDrawer::new(
        |x| match x {
            'A' => [0xFF, 0x0, 0x0],
            'B' => [0xFF, 0x40, 0x0],
            'C' => [0xFF, 0x80, 0x0],
            'D' => [0xFF, 0xBF, 0x0],
            'E' => [0xFF, 0xFF, 0x0],
            'F' => [0xBF, 0xFF, 0x0],
            'G' => [0x80, 0xFF, 0x0],
            'H' => [0x40, 0xFF, 0x0],
            'I' => [0x0, 0xFF, 0x0],
            'J' => [0x0, 0xFF, 0x40],
            'K' => [0x0, 0xFF, 0x80],
            'L' => [0x0, 0xFF, 0xBF],
            'M' => [0x0, 0xFF, 0xFF],
            'N' => [0x0, 0xBF, 0xFF],
            'O' => [0x0, 0x80, 0xFF],
            'P' => [0x0, 0x40, 0xFF],
            'Q' => [0x0, 0x0, 0xFF],
            'R' => [0x40, 0x0, 0xFF],
            'S' => [0x80, 0x0, 0xFF],
            'T' => [0xBF, 0x0, 0xFF],
            'U' => [0xFF, 0x0, 0xFF],
            'V' => [0xFF, 0x0, 0x40],
            'W' => [0x80, 0x80, 0x80],
            'X' => [0x40, 0x40, 0x40],
            'Y' => [0x20, 0x20, 0x20],
            _ => [0x0, 0x0, 0x0],
        },
        "ppm/examples/hexgrid",
    );

    gd.draw(&grid);
    gdb.draw(&grid);
    println!();

    let mut gg = grid.clone();
    gg.flip_horizontal();
    gd.draw(&gg);
    gdb.draw(&gg);
    println!();

    let mut gg = grid.clone();
    gg.flip_vertical();
    gd.draw(&gg);
    gdb.draw(&gg);
    println!();

    let mut gg = grid.clone();
    gg.flip_x();
    gd.draw(&gg);
    gdb.draw(&gg);
    println!();

    let mut gg = grid.clone();
    gg.flip_y();
    gd.draw(&gg);
    gdb.draw(&gg);
    println!();

    let mut gg = grid.clone();
    gg.flip_z();
    gd.draw(&gg);
    gdb.draw(&gg);
    println!();

    let mut gg = grid.clone();
    gg.rotate_60_cw();
    gd.draw(&gg);
    gdb.draw(&gg);
    println!();

    let mut gg = grid.clone();
    gg.rotate_120_cw();
    gd.draw(&gg);
    gdb.draw(&gg);
    println!();

    let mut gg = grid.clone();
    gg.rotate_180_cw();
    gd.draw(&gg);
    gdb.draw(&gg);
    println!();

    let mut gg = grid.clone();
    gg.rotate_240_cw();
    gd.draw(&gg);
    gdb.draw(&gg);
    println!();

    let mut gg = grid.clone();
    gg.rotate_300_cw();
    gd.draw(&gg);
    gdb.draw(&gg);
    println!();
}

fn main() {
    print_char();
}
