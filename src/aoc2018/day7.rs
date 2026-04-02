use crate::Input;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;

pub fn run(input: Input) {
    let data = input.parse_lines::<Step>();

    // part one.
    let mut todo = data
        .iter()
        .flat_map(|s| [s.from, s.to])
        .collect::<HashSet<_>>();
    let mut steps = data.clone().into_vec();
    while !todo.is_empty() {
        let blocked = steps.iter().map(|s| s.to).collect::<HashSet<_>>();
        let ready = todo
            .difference(&blocked)
            .map(|&s| Reverse(s))
            .collect::<BinaryHeap<_>>();
        let chosen = ready.peek().unwrap().0;
        print!("{chosen}");
        steps.retain(|s| s.from != chosen);
        todo.remove(&chosen);
    }
    println!();

    // part two.
    const NUM_WORKERS: usize = 5;
    const JOB_SIZE: u8 = 60;
    let mut t = 0;
    let mut workers = Vec::new();
    let mut todo = data
        .iter()
        .flat_map(|s| [s.from, s.to])
        .collect::<HashSet<_>>();
    let mut steps = data.into_vec();
    loop {
        workers.retain_mut(|(chosen, job)| {
            *job -= 1;
            if *job != 0 {
                true
            } else {
                steps.retain(|s| s.from != *chosen);
                false
            }
        });
        let blocked = steps.iter().map(|s| s.to).collect::<HashSet<_>>();
        let mut ready = todo
            .difference(&blocked)
            .map(|&s| Reverse(s))
            .collect::<BinaryHeap<_>>();
        while workers.len() < NUM_WORKERS && !ready.is_empty() {
            let chosen = ready.pop().unwrap().0;
            workers.push((chosen, chosen as u8 - b'A' + 1 + JOB_SIZE));
            todo.remove(&chosen);
        }
        // println!("{t}: {workers:?}");
        if todo.is_empty() && workers.is_empty() {
            break;
        }
        t += 1;
    }
    println!("{t}");
}

#[derive(Debug, Copy, Clone)]
struct Step {
    from: char,
    to: char,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            from: s.as_bytes()[5] as char,
            to: s.as_bytes()[36] as char,
        })
    }
}
