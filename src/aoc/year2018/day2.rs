use crate::Input;
use std::collections::HashMap;

pub fn run(input: Input) {
    let data = input.lines();

    // part one.
    let mut tmp = HashMap::new();
    let two_three = data
        .iter()
        .map(|w| {
            tmp.clear();
            w.chars().for_each(|c| {
                tmp.entry(c).and_modify(|e| *e += 1).or_insert(1);
            });
            (tmp.values().any(|&n| n == 2), tmp.values().any(|&n| n == 3))
        })
        .fold([0, 0], |acc, e| [acc[0] + e.0 as i32, acc[1] + e.1 as i32]);
    println!("{}", two_three[0] * two_three[1]);

    // part two.
    let similar = |w1: &str, w2: &str| {
        // the fastest way I could come up with: it short-circuits while detecting differences!
        w1.chars().zip(w2.chars()).try_fold(true, |acc, (a, b)| {
            (a == b).then_some(acc).or_else(|| acc.then_some(false))
        })
    };
    let (w1, w2) = data
        .iter()
        .flat_map(|w1| {
            data.iter()
                .filter(move |&w2| w2 != w1)
                .map(move |w2| (w1, w2))
        })
        .find(|(w1, w2)| similar(w1, w2).is_some())
        .unwrap();
    let response = w1
        .chars()
        .zip(w2.chars())
        .filter_map(|(c1, c2)| (c1 == c2).then_some(c1))
        .collect::<String>();
    println!("{}", response);
}
