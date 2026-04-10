use crate::input::Input;

pub fn run(input: Input) {
    let data = input.blocks();
    let ranges = data[0]
        .lines()
        .map(|s| {
            let (start, end) = s.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start, end)
        })
        .collect::<Vec<_>>();
    let items = data[1]
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // part one.
    let count = items
        .iter()
        .filter(|&&item| {
            ranges
                .iter()
                .any(|&(start, end)| item >= start && item <= end)
        })
        .count();
    println!("{count}");

    // part two.
    // |-------|              |-------|      |-------|
    //      |-------|      |-------|           |---|
    // since the ranges overlap, we need to merge them, then sum the lengths of the resulting ones.
    let mut ranges = ranges;
    ranges.sort_unstable_by_key(|&(start, _)| start);
    let merged = ranges
        .into_iter()
        .fold(Vec::new(), |mut acc, (start, end)| {
            match acc.last_mut() {
                Some((_, prev_end)) if start <= *prev_end => {
                    *prev_end = (*prev_end).max(end);
                }
                _ => acc.push((start, end)),
            }
            acc
        });
    let count = merged
        .into_iter()
        .map(|(start, end)| end - start + 1)
        .sum::<u64>();
    println!("{count}");
}
