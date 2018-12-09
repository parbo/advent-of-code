extern crate linked_list;

use linked_list::LinkedList;
use std::env;

fn solve(players: i64, marbles: i64) -> i64 {
    let mut circle = LinkedList::new();
    circle.push_front(0);
    let mut cursor = circle.cursor();
    let mut score = vec![];
    score.resize(players as usize, 0);
    for marble in 1..(marbles+1) {
        if marble % 23 == 0 {
            for _ in 0..7 {
                if let None = cursor.prev() {
                    cursor.prev();
                }
            }
            let removed = cursor.remove().unwrap();
            score[(marble % players) as usize] += removed + marble;
        } else {
            for _ in 0..2 {
                if let None = cursor.next() {
                    cursor.next();
                }
            }
            cursor.insert(marble);
        }
    }
    return *score.iter().max().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let players = args[1].parse::<i64>().unwrap();
    let marbles = args[2].parse::<i64>().unwrap();

    let result = solve(players, marbles);
    println!("{}", result);
}
