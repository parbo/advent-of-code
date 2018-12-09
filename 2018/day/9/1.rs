use std::collections::HashMap;
use std::env;

fn solve(players: i64, marbles: i64) -> i64 {
    let mut circle = vec![];
    circle.reserve(marbles as usize);
    circle.push(0);
    let mut player = 1;
    let mut pos : i64 = 0;
    let mut score = HashMap::new();
    for marble in 1..(marbles+1) {
        if marble % 23 == 0 {
            if pos > 7 {
                pos -= 7;
            } else {
                pos = (circle.len() as i64) + pos - 7;
            }
            let removed = circle.remove(pos as usize);
            *score.entry(player).or_insert(0) += removed + marble;
        } else {
            pos = (pos + 2) % (circle.len() as i64);
            circle.insert(pos as usize, marble);
        }
        player = (player + 1) % players;
    }
    return *score.iter().map(|a| a.1).max().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let players = args[1].parse::<i64>().unwrap();
    let marbles = args[2].parse::<i64>().unwrap();

    let result = solve(players, marbles);
    println!("{}", result);
}
