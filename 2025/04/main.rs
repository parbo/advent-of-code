use aoc::{Grid, Point};

#[cfg(feature = "vis")]
use aoc::GridDrawer;

type Parsed = Vec<Vec<char>>;

fn get_removable(g: &Parsed) -> Vec<Point> {
    let mut to_remove = vec![];
    for p in g.points() {
        if g.get_value(p) != Some('@') {
            continue;
        }
        let num = aoc::neighbors_incl_diagonals(p)
            .filter(|nb| g.get_value(*nb) == Some('@'))
            .count();
        if num < 4 {
            to_remove.push(p);
        }
    }
    to_remove
}

fn part1(data: &Parsed) -> i64 {
    get_removable(data).len() as i64
}

fn part2(data: &Parsed) -> i64 {
    let mut ans = 0;
    let mut g = data.clone();
    #[cfg(feature = "vis")]
    let mut gd = aoc::make_bitmap_text_grid_drawer(
        |x| match x {
            '@' => (x, [0xff, 0xff, 0xff]),
            'x' => ('@', [0xff, 0, 0]),
            _ => (x, [0x7f, 0x7f, 0x7f]),
        },
        "vis/04/day04",
    );
    loop {
        let to_remove = get_removable(&g);
        if to_remove.is_empty() {
            break;
        }
        for p in &to_remove {
            g.set_value(*p, 'x');
            ans += 1;
        }
        #[cfg(feature = "vis")]
        {
            gd.draw(&g);
        }
        for p in &to_remove {
            g.set_value(*p, '.');
        }
    }
    ans
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
        let s = include_str!("example.txt");
        s.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&example())), 13);
    }
}
