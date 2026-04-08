use crate::Input;
use Hand::*;
use Outcome::*;

pub fn run(input: Input) {
    let data = input.map_lines(|s| {
        let s = s.as_bytes();
        (Hand::from((s[0], "ABC")), Hand::from((s[2], "XYZ")))
    });

    // part one.
    let games = data.iter().map(|&(a, b)| b.score(a));
    println!("{}", games.sum::<u32>());

    // part two.
    let games = data
        .iter()
        .map(|&(a, b)| (a, Outcome::from(b)))
        .map(|(a, out)| a.reverse(out).score(a));
    println!("{}", games.sum::<u32>());
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Hand {
    Rock = 1, // number of points.
    Paper = 2,
    Scissors = 3,
}

impl From<(u8, &str)> for Hand {
    fn from((letter, map): (u8, &str)) -> Self {
        let map = map.as_bytes();
        match *map {
            [x, _, _] if x == letter => Rock,
            [_, x, _] if x == letter => Paper,
            [_, _, x] if x == letter => Scissors,
            _ => unreachable!(),
        }
    }
}

impl Hand {
    fn score(self, other: Hand) -> u32 {
        self as u32
            + match (self, other) {
                _ if self == other => 3,
                _ if self == other.reverse(Win) => 6,
                _ => 0,
            }
    }

    fn reverse(self, outcome: Outcome) -> Self {
        let (lose, win) = match (self, outcome) {
            (x, Draw) => return x,
            (Rock, _) => (Scissors, Paper),
            (Paper, _) => (Rock, Scissors),
            (Scissors, _) => (Paper, Rock),
        };
        if outcome == Win {
            win
        } else {
            lose
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl From<Hand> for Outcome {
    fn from(hand: Hand) -> Self {
        match hand {
            Rock => Lose,
            Paper => Draw,
            Scissors => Win,
        }
    }
}
