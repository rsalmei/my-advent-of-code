use crate::input::Input;

pub fn run(input: Input) {
    let data = input.lines();

    // part one.
    let sum = data
        .iter()
        .map(|s| {
            let s = s.as_bytes();
            let a = s[..s.len() - 1].iter().max().unwrap();
            let i = s.iter().position(|&b| b == *a).unwrap();
            let b = s[i + 1..].iter().max().unwrap();
            (a - b'0') as u32 * 10 + (b - b'0') as u32
        })
        .sum::<u32>();
    println!("{sum}");

    // part two.
    const N: usize = 12;
    let sum = data
        .iter()
        .map(|s| {
            println!("{s}");
            let s = s.as_bytes();
            (1..=N)
                .fold((0, 0), |(acc, p), i| {
                    let &a = s[p..s.len() - N + i].iter().max().unwrap();
                    // this is a debug print, but it's too beautiful to remove.
                    println!(
                        "{}{} ({p}..{}) {}",
                        " ".repeat(p),
                        "#".repeat(s.len() - N + i - p),
                        s.len() - N + i,
                        a as char
                    );
                    let p = 1 + p + s[p..].iter().position(|&b| b == a).unwrap();
                    (acc * 10 + (a - b'0') as u64, p)
                })
                .0
        })
        .sum::<u64>();
    println!("{sum}");
}
