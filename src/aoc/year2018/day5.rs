use crate::Input;
use std::iter;

pub fn run(input: Input) {
    let data = input.bytes();

    // part one.
    fn react_fully<'a>(w: impl Iterator<Item = &'a u8>, out: &mut Vec<u8>) -> usize {
        out.clear();
        w.chain(iter::once(&0)).fold(None, |acc: Option<u8>, &d| {
            match acc {
                Some(a) if a.abs_diff(d) == b'a' - b'A' => return out.pop(),
                Some(a) => out.push(a),
                None => {}
            };
            Some(d)
        });
        out.len()
    }
    let mut out = Vec::with_capacity(data.len());
    println!("{}", react_fully(data.iter(), &mut out));

    // part two.
    let min_len = (b'a'..=b'z')
        .map(|unit| {
            let unit_up = unit.to_ascii_uppercase();
            let polymer = data.iter().filter(|&&c| c != unit && c != unit_up);
            react_fully(polymer, &mut out)
        })
        .min();
    println!("{}", min_len.unwrap());
}
