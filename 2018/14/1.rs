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
    let mut elf_1 = 0;
    let mut elf_2 = 1;
    while recipes.len() < offs + 10 {
        dump(&recipes, elf_1, elf_2);
        let elf_1_recipe = recipes[elf_1];
        let elf_2_recipe = recipes[elf_2];
        let r = elf_1_recipe + elf_2_recipe;
        if r > 9 {
            recipes.push(r / 10);
            recipes.push(r % 10);
        } else {
            recipes.push(r);
        }
        elf_1 = (elf_1 + 1 + elf_1_recipe) % recipes.len();
        elf_2 = (elf_2 + 1 + elf_2_recipe) % recipes.len();
    }
    dump(&recipes, elf_1, elf_2);
    answer(&recipes[offs..(offs+10)]);
}

fn main() {
    solve();
}
