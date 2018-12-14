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

fn solve(initial: &str, offs: usize, num_elves: usize) {
    let mut recipes : Vec<usize> = initial.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let mut elves : Vec<usize> = (0..num_elves).collect();

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
    println!("{}, {}, {} => {}", initial, offs, num_elves, s);
}

fn main() {
    assert!(digits(12345) == vec![1, 2, 3, 4, 5]);
    assert!(digits(4000) == vec![4, 0, 0, 0]);
    assert!(digits(18) == vec![1, 8]);
    assert!(digits(10) == vec![1, 0]);
    assert!(digits(0) == vec![0]);
    assert!(digits(7) == vec![7]);

    solve("37", 5, 2);
    solve("37", 9, 2);
    solve("37", 18, 2);
    solve("37", 2018, 2);
    for initial in &["074501", "430971"] {
        solve(initial, initial.len(), 2);
        solve(initial, initial.len(), initial.len());
    }
}
