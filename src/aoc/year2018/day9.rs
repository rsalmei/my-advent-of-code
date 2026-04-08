use crate::Input;
use std::collections::VecDeque;

pub fn run(input: Input) {
    let data = input.text();

    // part one.
    let mut it = data.split_ascii_whitespace();
    let mut get = |n| it.nth(n).unwrap().parse().unwrap();
    let (players, marbles) = (get(0), get(5));
    let scores = game(players, marbles);
    println!("{}", scores.into_iter().max().unwrap());

    // part two.
    let scores = game(players, marbles * 100);
    println!("{}", scores.into_iter().max().unwrap())
}

fn game(players: u32, marbles: u32) -> Vec<u32> {
    let mut scores = vec![0; players as _];
    let mut board = VecDeque::with_capacity(marbles as _);
    board.push_back(0);
    for m in 1..marbles {
        // println!("{board:?}");
        if m % 23 > 0 {
            board.rotate_left(1);
            board.rotate_left(1);
            board.insert(0, m);
        } else {
            board.rotate_right(7);
            scores[(m % players) as usize] += m + board.remove(0).unwrap_or_default();
        }
    }
    scores
}
