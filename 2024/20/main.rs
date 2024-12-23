use aoc::Grid;

type Parsed = Vec<Vec<char>>;

fn get_neighbors(data: &Parsed, p: aoc::Point, d: i64) -> aoc::FxHashSet<(aoc::Point, i64)> {
    let mut res = aoc::FxHashSet::default();
    let mut todo = vec![(p, d)];
    let mut seen = aoc::FxHashSet::default();
    while let Some((pp, d)) = todo.pop() {
        for nb in aoc::neighbors(pp) {
            if nb == p {
                continue;
            }
            match data.get_value(nb) {
                Some('.') | Some('E') => {
                    res.insert((nb, d));
                }
                Some('#') => {
                    if d > 0 && seen.insert((nb, d - 1)) {
                        todo.push((nb, d - 1));
                    }
                }
                _ => {}
            }
        }
    }
    // if d > 0 {
    //     dbg!(p, d, &res);
    // }
    res
}

fn solve(
    data: &Parsed,
    path: Vec<aoc::Point>,
    goal: aoc::Point,
    score: i64,
    threshold: i64,
    rem: i64,
    seen: &mut aoc::FxHashMap<(aoc::Point, i64), i64>,
) -> i64 {
    let pos = *path.last().unwrap();
    if let Some(x) = seen.get(&(pos, rem)) {
        // if *x != 0 {
        //     dbg!(pos, rem, x);
        // }
        return *x;
    }
    if pos == goal {
        println!("{}, {}, {}", score, path.len(), rem);
        return 1;
    }
    let mut num = 0;
    for (nb, r) in get_neighbors(data, pos, rem) {
        if path.contains(&nb) {
            continue;
        }
        let new_score = score + aoc::manhattan(pos, nb);
        if new_score > threshold {
            dbg!(new_score, threshold);
            continue;
        }
        let mut new_path = path.clone();
        new_path.push(nb);
        num += solve(data, new_path, goal, new_score, threshold, r, seen);
    }

    seen.insert((pos, rem), num);
    num
}

fn part1(data: &Parsed) -> i64 {
    let mut seen = aoc::FxHashMap::default();
    let s = data
        .points()
        .find(|p| data.get_value(*p) == Some('S'))
        .unwrap();
    let e = data
        .points()
        .find(|p| data.get_value(*p) == Some('E'))
        .unwrap();
    let uncheated_res =
        aoc::dijkstra_grid(data, |_p, c| *c != '#', |_p1, _c1, _p2, _c2| Some(1), s, e)
            .unwrap()
            .0;
    dbg!(uncheated_res);
    let res = solve(data, vec![s], e, 0, uncheated_res - 100, 1, &mut seen);
    dbg!(seen.len());
    res
}

fn part2(_data: &Parsed) -> i64 {
    // solve(data, 100, 19)
    0
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
        let mut seen = aoc::FxHashMap::default();
        assert_eq!(solve(&parse(&example()), 1, 1), 44);
    }
}
