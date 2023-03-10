type Parsed = Vec<Vec<char>>;

fn slope(forest: &[Vec<char>], xx: usize, yy: usize) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    let w = forest[0].len();
    if forest[y][x] == '#' {
        trees += 1;
    }
    loop {
        x += xx;
        y += yy;
        if y >= forest.len() {
            return trees;
        }
        if forest[y][x % w] == '#' {
            trees += 1;
        }
    }
}

fn part1(forest: &Parsed) -> i64 {
    slope(forest, 3, 1)
}

fn part2(f: &Parsed) -> i64 {
    slope(f, 1, 1) * slope(f, 3, 1) * slope(f, 5, 1) * slope(f, 7, 1) * slope(f, 1, 2)
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid(lines)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
