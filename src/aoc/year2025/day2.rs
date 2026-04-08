use crate::input::Input;

pub fn run(input: Input) {
    let data = input
        .text()
        .split(',')
        .map(|s| {
            let (a, b) = s.split_once('-').unwrap();
            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    // part one.
    let sum = data
        .iter()
        .flat_map(|&(a, b)| a..=b)
        .filter(|&n| {
            let s = n.to_string();
            if s.len() % 2 == 1 {
                return false;
            }
            s.bytes().take(s.len() / 2).eq(s.bytes().skip(s.len() / 2))
        })
        .sum::<u64>();
    println!("{sum}");

    // part two.
    let sum = data
        .iter()
        .flat_map(|&(a, b)| a..=b)
        .filter(|&n| {
            let s = n.to_string();
            let m = s.len() / 2;
            (1..m + 1).any(|i| {
                let first = &s.as_bytes()[..i];
                // println!("  {i}: {}", str::from_utf8(first).unwrap());
                s.as_bytes()[i..].chunks(i).all(|chunk| chunk == first)
            })
        })
        .sum::<u64>();
    println!("{sum}");
}
