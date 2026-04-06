use crate::input::Input;
use std::str::FromStr;

pub fn run(input: Input) {
    let data = input.parse_lines::<Turn>();

    // part one.
    let mut pos = 50;
    let mut zeros = 0;
    data.iter().for_each(|turn| {
        let sign = match turn.dir {
            'L' => -1,
            'R' => 1,
            _ => unreachable!(),
        };
        pos = (pos + sign * turn.dist).rem_euclid(100);
        if pos == 0 {
            zeros += 1;
        }
    });
    println!("{zeros}");

    // part two.
    (pos, zeros) = (50, 0);
    data.iter().for_each(|turn| {
        let (sign, overflow) = match turn.dir {
            'L' => (-1, pos),
            'R' => (1, 100 - pos),
            _ => unreachable!(),
        };
        pos = (pos + sign * turn.dist).rem_euclid(100);

        if turn.dist >= overflow && overflow > 0 {
            zeros += 1;
        }
        zeros += (turn.dist - overflow) / 100;
    });
    println!("{zeros}");
}

#[derive(Debug)]
struct Turn {
    dir: char,
    dist: i32,
}

impl FromStr for Turn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let (dir, dist) = s.split_at(1);
        Ok(Self {
            dir: dir.chars().next().unwrap(),
            dist: dist.parse().unwrap(),
        })
    }
}
