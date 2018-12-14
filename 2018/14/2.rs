fn digits(num: usize) -> Vec<usize> {
    let mut digs = vec![];
    let mut r = num;
    loop {
        let digit = r - 10 * (r / 10);
        assert!(digit < 10);
        digs.insert(0, digit);
        r = r / 10;
        if r == 0 {
            break;
        }
    }
    digs
}

fn solve(pattern_str: &str) {
    let pattern : Vec<usize> = pattern_str.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let mut recipes = vec![3, 7];
    let mut elves : Vec<usize> = (0..2).collect();

    loop {
        let r : usize  = elves.iter().map(|e| recipes[*e]).sum();
        let digs = digits(r);
        for d in digs {
            recipes.push(d);
            let rl = recipes.len();
            let pl = pattern.len();
            if rl >= pl {
                let mut all_match = true;
                for i in 0..pl {
                    if pattern[i] != recipes[rl - pl + i] {
                        all_match = false;
                        break;
                    }
                }
                if all_match {
                    println!("{}", rl - pl);
                    return;
                }
            }
        }
        for e in &mut elves {
            *e = (*e + 1 + recipes[*e]) % recipes.len();
        }
    }
}

fn main() {
    solve("430971");
}
