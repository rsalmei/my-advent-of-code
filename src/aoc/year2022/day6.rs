use crate::Input;

pub fn run(input: Input) {
    let data = input.bytes();

    // part one.
    let all_diff = |s: &[u8]| (0..s.len() - 1).all(|p| (p + 1..s.len()).all(|i| s[p] != s[i]));
    let find_mark = |n| data.windows(n).position(all_diff).map(|x| x + n);
    println!("{}", find_mark(4).unwrap());

    // part one.
    println!("{}", find_mark(14).unwrap());
}
