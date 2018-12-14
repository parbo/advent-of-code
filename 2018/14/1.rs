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

fn solve(offs: usize) {
    let mut recipes = vec![3, 7];
    let mut elves = vec![0, 1];

    while recipes.len() < offs + 10 {
        let r : usize  = elves.iter().map(|e| recipes[*e]).sum();
        recipes.extend(digits(r));
        for e in &mut elves {
            *e = (*e + 1 + recipes[*e]) % recipes.len();
        }
    }
    let strs : Vec<String> = recipes[offs..(offs+10)].iter().map(|r| r.to_string()).collect();
    let s = strs.join("");
    if recipes.len() < 50 {
        println!("{:?}", recipes);
    } else {
        println!("[too long]");
    }
    println!("{} => {}", offs, s);
}

fn main() {
    assert!(digits(12345) == vec![1, 2, 3, 4, 5]);
    assert!(digits(4000) == vec![4, 0, 0, 0]);
    assert!(digits(18) == vec![1, 8]);
    assert!(digits(10) == vec![1, 0]);
    assert!(digits(0) == vec![0]);
    assert!(digits(7) == vec![7]);

    solve(5);
    solve(9);
    solve(18);
    solve(2018);
    solve(430971);
}
