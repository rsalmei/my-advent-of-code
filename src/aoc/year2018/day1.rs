use crate::Input;
use std::collections::HashSet;

pub fn run(input: Input) {
    let data = input.parse_lines();

    // part one.
    println!("{}", data.iter().sum::<i32>());

    // part two.
    let data = data.iter().cycle();
    let mut seen = HashSet::from([0]);
    let mut frequencies = data.scan(0, |acc, &n| {
        *acc += n;
        Some(*acc)
    });
    println!("{}", frequencies.find(|&f| !seen.insert(f)).unwrap());
}
