use std::iter::*;

type Parsed = Vec<(String, Vec<(usize, String)>)>;

fn part1(rules: &Parsed) -> i64 {
    let mut graph = aoc::GraphMap::<&String, i32, aoc::Directed>::new();
    for (node, neighbors) in rules {
        let gp = graph.add_node(node);
        for (_c, n) in neighbors {
            let gnp = graph.add_node(n);
            graph.add_edge(gp, gnp, 1);
        }
    }
    let mut with_path = 0;
    for (node, _) in rules {
        if node != "shiny gold"
            && aoc::algo::has_path_connecting(&graph, node, &"shiny gold".to_string(), None)
        {
            with_path += 1;
        }
    }
    with_path
}

fn sum_bags(bag: &str, rules: &Parsed) -> usize {
    for (node, neighbors) in rules {
        if node == bag {
            let mut tot = 1;
            for (c, n) in neighbors {
                tot += c * sum_bags(n, rules);
            }
            return tot;
        }
    }
    0
}

fn part2(rules: &Parsed) -> i64 {
    // - 1 as we're not counting the "shiny gold" bag
    sum_bags("shiny gold", rules) as i64 - 1
}

fn parse(lines: &[String]) -> Parsed {
    let rx1 = regex::Regex::new(r"^(.*) bags contain").unwrap();
    let rx2 = regex::Regex::new(r"(\d+) (.+?) bags?").unwrap();
    lines
        .iter()
        .map(|x| {
            (
                rx1.captures(x)
                    .and_then(|x| x.get(1))
                    .map_or("", |m| m.as_str())
                    .to_string(),
                rx2.captures_iter(x)
                    .map(|caps| {
                        (
                            caps.get(1)
                                .map_or(0, |m| m.as_str().parse::<usize>().unwrap()),
                            caps.get(2).map_or("", |m| m.as_str()).to_string(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn main() {
    aoc::run_main(parse, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_bag() {
        let lines = vec![
            "light; red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];
        let rules = parse(&lines);
        assert_eq!(part2(&rules), 32);
    }

    #[test]
    fn test_sum_bag2() {
        let lines = vec![
            "shiny gold bags contain 2 dark red bags.".to_string(),
            "dark red bags contain 2 dark orange bags.".to_string(),
            "dark orange bags contain 2 dark yellow bags.".to_string(),
            "dark yellow bags contain 2 dark green bags.".to_string(),
            "dark green bags contain 2 dark blue bags.".to_string(),
            "dark blue bags contain 2 dark violet bags.".to_string(),
            "dark violet bags contain no other bags.".to_string(),
        ];
        let rules = parse(&lines);
        assert_eq!(part2(&rules), 126);
    }
}
