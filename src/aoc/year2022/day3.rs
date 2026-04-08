use crate::Input;
use std::collections::HashSet;

pub fn run(input: Input) {
    let data = input.lines();

    // part one.
    let mut priority = Vec::with_capacity(53);
    priority.push(b'!');
    priority.extend(b'a'..=b'z');
    priority.extend(b'A'..=b'Z');

    let items = data
        .iter()
        .map(|ks| {
            let (a, b) = ks.split_at(ks.len() / 2);
            let a = a.as_bytes().iter().copied().collect::<HashSet<_>>();
            let b = b.as_bytes().iter().copied().collect();
            *a.intersection(&b).next().unwrap()
        })
        .map(|i| priority.iter().position(|&t| i == t).unwrap());
    println!("{}", items.sum::<usize>());

    // part two.
    let items = data
        .chunks(3)
        .map(|g| {
            let a = g[0].as_bytes().iter().copied().collect::<HashSet<_>>();
            let b = g[1].as_bytes().iter().copied().collect();
            let c = g[2].as_bytes().iter().copied().collect();
            *a.intersection(&b)
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&c)
                .next()
                .unwrap()
        })
        .map(|i| priority.iter().position(|&t| i == t).unwrap());
    println!("{}", items.sum::<usize>())
}
