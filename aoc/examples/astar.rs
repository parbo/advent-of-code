use aoc::Grid;
use aoc::GridDrawer;

fn make_grid() -> Vec<String> {
    vec![
        "#########".into(),
        "#.......#".into(),
        "#####...#".into(),
        "#.......#".into(),
        "#..######".into(),
        "##......#".into(),
        "#####...#".into(),
        "#...#...#".into(),
        "#.......#".into(),
        "#########".into(),
    ]
}

fn make_weighted() -> Vec<String> {
    vec![
        "1163751742".into(),
        "1381373672".into(),
        "2136511328".into(),
        "3694931569".into(),
        "7463417111".into(),
        "1319128137".into(),
        "1359912421".into(),
        "3125421639".into(),
        "1293138521".into(),
        "2311944581".into(),
    ]
}

fn astar_char() {
    let mut grid = aoc::parse_grid(&make_grid());
    let graph = aoc::grid_to_undirected_graph(&grid, |_p, c| *c == '.', |_p1, _c1, _p2, _c2| Some(1), 4);
    let a: aoc::Point = [1, 1];
    let f: aoc::Point = [1, 7];
    if let Some((_cost, path)) = aoc::astar(&graph, a, f) {
        for point in &path {
            grid.set_value(*point, '*');
        }
    }
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    gd.draw(&grid);
}

fn astar_char_sparse() {
    let mut grid =
        aoc::parse_grid_to_sparse(&make_grid(), |c| if c == '.' { Some(c) } else { None });
    let graph = aoc::grid_to_undirected_graph(&grid, |_p, c| *c == '.', |_p1, _c1, _p2, _c2| Some(1), 4);
    let a: aoc::Point = [1, 1];
    let f: aoc::Point = [1, 7];
    if let Some((_cost, path)) = aoc::astar(&graph, a, f) {
        for point in &path {
            grid.set_value(*point, '*');
        }
    }
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    gd.draw(&grid);
}

fn astar_weighted() {
    let mut grid = aoc::parse_grid(&make_weighted());
    let graph = aoc::grid_to_directed_graph(
        &grid,
        |_p, _c| true,
        |_p1, _c1, _p2, c2| Some(c2.to_digit(10).unwrap() as i64),
        4,
    );
    let a: aoc::Point = [0, 0];
    let f: aoc::Point = [9, 9];
    if let Some((_cost, path)) = aoc::astar(&graph, a, f) {
        for point in &path {
            grid.set_value(*point, '*');
        }
    }
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    gd.draw(&grid);
}

fn main() {
    astar_char();
    println!();
    astar_char_sparse();
    println!();
    astar_weighted();
}
