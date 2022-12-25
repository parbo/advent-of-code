use aoc::Grid;

type ParsedItem = Vec<char>;
type Parsed = Vec<ParsedItem>;
type Answer = i64;

fn part1(grid: &Parsed) -> Answer {
    let mut g = grid.to_owned();
    // let mut gd = aoc::PrintGridDrawer::new(|c| c);
    let mut step = 0;
    let ([minx, miny], [maxx, maxy]) = g.extents();
    loop {
        step += 1;
        let mut new_g = g.clone();
        for (c, d) in [('>', aoc::EAST), ('v', aoc::SOUTH)] {
            let mut nn_g = new_g.clone();
            for p in new_g.points() {
                if let Some(v) = new_g.get_value(p) {
                    if v == c {
                        let mut dp = aoc::point_add(p, d);
                        if dp[0] > maxx {
                            dp[0] = minx;
                        }
                        if dp[1] > maxy {
                            dp[1] = miny;
                        }
                        if let Some('.') = new_g.get_value(dp) {
                            nn_g.set_value(p, '.');
                            nn_g.set_value(dp, c);
                        }
                    }
                }
            }
            new_g = nn_g;
        }
        // gd.draw(&new_g);
        // println!();
        if new_g == g {
            break;
        }
        g = new_g;
    }
    step
}

fn part2(_: &Parsed) -> Answer {
    0
}

fn parse(lines: &[String]) -> Parsed {
    aoc::parse_grid(lines)
}

fn main() {
    aoc::run_main(parse, part1, part2);
}
