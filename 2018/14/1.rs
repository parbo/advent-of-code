fn dump(recipes: &[usize], elf_1: usize, elf_2: usize) {
    for (n, recipe) in recipes.iter().enumerate() {
        if n == elf_1 {
            print!("({})", recipe);
        } else if n == elf_2 {
            print!("[{}]", recipe);
        } else {
            print!(" {} ", recipe);
        }
    }
    println!("");
}

fn answer(recipes: &[usize]) {
    for recipe in recipes {
        print!("{}", recipe);
    }
    println!("");
}

fn solve() {
    let mut recipes = vec![4, 3, 0, 9, 7, 1];
    let offs = recipes.len();
    let mut elves : Vec<usize> = (0..offs).collect();

    while recipes.len() < offs + 10 {
//        dump(&recipes, elf_1, elf_2);
        let mut r : usize  = elves.iter().map(|e| recipes[*e]).sum();
        let mut i = 1;
        while r > 0 {
            recipes.push(r % (10 * i));
            r = r / 10;
            i += 1;
        }

        elves = elves.iter().map(|e| (e + 1 + recipes[*e]) % recipes.len()).collect();
    }
//    dump(&recipes, elf_1, elf_2);
    answer(&recipes[offs..(offs+10)]);
}

fn main() {
    solve();
}
