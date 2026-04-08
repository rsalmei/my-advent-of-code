use crate::Input;
use std::str::FromStr;

pub fn run(input: Input) {
    let data = input.map_lines(|s| {
        let (s, t) = s.split_once(',').unwrap();
        (s.parse::<Pair>().unwrap(), t.parse().unwrap())
    });

    // part one.
    let redundant = data.iter().filter(|(p, q)| p.fully_contains(q));
    println!("{}", redundant.count());

    // part two.
    let overlap = data.iter().filter(|(p, q)| p.overlap(q));
    println!("{}", overlap.count());
}

struct Pair {
    a: u8,
    b: u8,
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('-').ok_or(())?;
        Ok(Self {
            a: a.parse().map_err(|_| ())?,
            b: b.parse().map_err(|_| ())?,
        })
    }
}

impl Pair {
    fn fully_contains(&self, other: &Self) -> bool {
        (self.a <= other.a && self.b >= other.b) || (other.a <= self.a && other.b >= self.b)
    }

    fn overlap(&self, other: &Self) -> bool {
        !(self.a > other.b || self.b < other.a)
    }
}
