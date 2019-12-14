use aoc;
// use intcode;
use serde::Deserialize;
use std::collections::HashMap;
use std::iter::*;

fn find_amount(
    p: &Product,
    reqs: &HashMap<String, (i64, Vec<Product>)>,
    pile: &mut HashMap<String, i64>,
    depth: usize,
) -> i64 {
    // println!("{:indent$}lookup: {}", "", p.material, indent = depth);
    if let Some((amount, materials)) = reqs.get(&p.material) {
        let mut m = 0;
        let mut x = 1;
        while x * amount < p.amount {
            x = x + 1;
        }
        for _ in 0..x {
            for pr in materials {
                // println!(
                //     "{:indent$}{} {} needs {} {}",
                //     "",
                //     amount,
                //     p.material,
                //     pr.amount,
                //     pr.material,
                //     indent = depth
                // );
                let pile_amount = pile.entry(pr.material.clone()).or_insert(0);
                if *pile_amount > 0 {
                    // Consume from the pile
                    // println!(
                    //     "{:indent$}=> consume {} {} from the pile",
                    //     "",
                    //     pr.amount,
                    //     pr.material,
                    //     indent = depth
                    // );
                    if *pile_amount >= pr.amount {
                        *pile_amount -= pr.amount;
                    } else {
                        let pr_left = Product {
                            amount: pr.amount - *pile_amount,
                            material: pr.material.clone(),
                        };
                        *pile_amount = 0;
                        if pr.material == "ORE" {
                            m += pr_left.amount;
                        } else {
                            m += find_amount(&pr_left, reqs, pile, depth + 1);
                        }
                    }
                } else {
                    if pr.material == "ORE" {
                        m += pr.amount;
                    } else {
                        m += find_amount(&pr, reqs, pile, depth + 1);
                    }
                }
            }
        }
        let rest = x * amount - p.amount;
        if rest > 0 {
            // println!(
            //     "{:indent$}=> put {} {} on the pile",
            //     "",
            //     rest,
            //     p.material,
            //     indent = depth
            // );
            *pile.entry(p.material.clone()).or_insert(0) += rest;
        }
        // println!(
        //     "{:indent$}=> {} {} needs {} ORE",
        //     "",
        //     p.amount,
        //     p.material,
        //     m,
        //     indent = depth
        // );
        return m;
    } else {
        panic!("NOT FOUND");
    }
}

fn part1(reqs: &HashMap<String, (i64, Vec<Product>)>) -> i64 {
    let p = Product {
        material: "FUEL".to_string(),
        amount: 1,
    };
    let mut pile = HashMap::new();
    find_amount(&p, reqs, &mut pile, 0)
}

fn part2(reqs: &HashMap<String, (i64, Vec<Product>)>) -> i64 {
    let p = Product {
        material: "FUEL".to_string(),
        amount: 1,
    };
    let mut pile = HashMap::new();
    pile.insert("ORE".to_string(), 1000000000000);
    let mut f = 0;
    loop {
        find_amount(&p, reqs, &mut pile, 0);
        let non_zero = pile
            .iter()
            .filter(|x| x.0 != "ORE")
            .any(|x| *x.1 != 0);
        f += 1;
        if !non_zero {
            println!("cycle found! f: {}", f);
            println!("f: {}, pile: {:?}", f, pile);
            break;
        }
        if f % 10000 == 0 {
            println!("f: {}, pile: {:?}", f, pile);
        }
    }
    let rem = *pile.get(&"ORE".to_string()).unwrap();
    let consumed = 1000000000000 - rem;
    let cycles = 1000000000000 / consumed;
    let ore = 1000000000000 - cycles * consumed;
    println!("consumed: {}, rem: {}, cycles: {}, ore: {}", consumed, rem, cycles, ore);
    let mut pile2 = HashMap::new();
    pile2.insert("ORE".to_string(), ore);
    let mut f2 = 0;
    loop {
        let o2 = find_amount(&p, reqs, &mut pile2, 0);
        if o2 > 0 {
            // stockpile depleted
            break;
        }
        f2 += 1;
    }
    println!("cycles: {}, f: {}, f2: {}", cycles, f, f2);
    cycles * f + f2
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
struct Product {
    amount: i64,
    material: String,
}

fn parse(lines: &Vec<String>) -> HashMap<String, (i64, Vec<Product>)> {
    let mut requirements = HashMap::new();
    for line in lines {
        let sides: Vec<_> = line.split("=>").collect();
        let left: Vec<Product> = sides[0]
            .split(",")
            .map(|x| x.trim())
            .map(|x| aoc::from_str(x).unwrap())
            .collect();
        let rstring = sides[1].trim();
        let right: Product = aoc::from_str(rstring).unwrap();
        requirements.insert(right.material, (right.amount, left));
    }
    requirements
}

fn main() {
    let (part, lines) = aoc::read_lines();
    //let parsed = aoc::parse_intcode(&lines);
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::{parse, part1, part2};

    #[test]
    fn test_part1_1() {
        let input = vec![
            "10 ORE => 10 A".to_string(),
            "1 ORE => 1 B".to_string(),
            "7 A, 1 B => 1 C".to_string(),
            "7 A, 1 C => 1 D".to_string(),
            "7 A, 1 D => 1 E".to_string(),
            "7 A, 1 E => 1 FUEL".to_string(),
        ];
        let reqs = parse(&input);
        assert_eq!(part1(&reqs), 31);
    }

    #[test]
    fn test_part1_2() {
        let input = vec![
            "9 ORE => 2 A".to_string(),
            "8 ORE => 3 B".to_string(),
            "7 ORE => 5 C".to_string(),
            "3 A, 4 B => 1 AB".to_string(),
            "5 B, 7 C => 1 BC".to_string(),
            "4 C, 1 A => 1 CA".to_string(),
            "2 AB, 3 BC, 4 CA => 1 FUEL".to_string(),
        ];
        let reqs = parse(&input);
        assert_eq!(part1(&reqs), 165);
    }

    #[test]
    fn test_part1_3() {
        let input = vec![
            "157 ORE => 5 NZVS".to_string(),
            "165 ORE => 6 DCFZ".to_string(),
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL".to_string(),
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ".to_string(),
            "179 ORE => 7 PSHF".to_string(),
            "177 ORE => 5 HKGWZ".to_string(),
            "7 DCFZ, 7 PSHF => 2 XJWVT".to_string(),
            "165 ORE => 2 GPVTF".to_string(),
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT".to_string(),
        ];
        let reqs = parse(&input);
        assert_eq!(part1(&reqs), 13312);
        assert_eq!(part2(&reqs), 82892753);
    }

    #[test]
    fn test_part1_4() {
        let input = vec![
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG".to_string(),
            "17 NVRVD, 3 JNWZP => 8 VPVL".to_string(),
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL".to_string(),
            "22 VJHF, 37 MNCFX => 5 FWMGM".to_string(),
            "139 ORE => 4 NVRVD".to_string(),
            "144 ORE => 7 JNWZP".to_string(),
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC".to_string(),
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV".to_string(),
            "145 ORE => 6 MNCFX".to_string(),
            "1 NVRVD => 8 CXFTF".to_string(),
            "1 VJHF, 6 MNCFX => 4 RFSQX".to_string(),
            "176 ORE => 6 VJHF".to_string(),
        ];
        let reqs = parse(&input);
        assert_eq!(part1(&reqs), 180697);
        assert_eq!(part2(&reqs), 5586022);
    }

    #[test]
    fn test_part1_5() {
        let input = vec![
            "171 ORE => 8 CNZTR".to_string(),
            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL".to_string(),
            "114 ORE => 4 BHXH".to_string(),
            "14 VRPVC => 6 BMBT".to_string(),
            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL".to_string(),
            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT".to_string(),
            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW".to_string(),
            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW".to_string(),
            "5 BMBT => 4 WPTQ".to_string(),
            "189 ORE => 9 KTJDG".to_string(),
            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP".to_string(),
            "12 VRPVC, 27 CNZTR => 2 XDBXC".to_string(),
            "15 KTJDG, 12 BHXH => 5 XCVML".to_string(),
            "3 BHXH, 2 VRPVC => 7 MZWV".to_string(),
            "121 ORE => 7 VRPVC".to_string(),
            "7 XCVML => 6 RJRHP".to_string(),
            "5 BHXH, 4 VRPVC => 5 LTCX".to_string(),
        ];
        let reqs = parse(&input);
        assert_eq!(part1(&reqs), 2210736);
        assert_eq!(part2(&reqs), 460664);
    }
}
