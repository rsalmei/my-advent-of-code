use crate::Input;

pub fn run(input: Input) {
    let data = input.map_lines(|s| s.as_bytes().iter().map(|&x| x - b'0').collect::<Vec<_>>());

    // part one.
    let (rows, cols) = (data.len(), data[0].len());
    let visibles = (1..rows - 1)
        .flat_map(|y| (1..cols - 1).map(move |x| (y, x)))
        .filter(|&(y, x)| {
            let pivot = data[y][x];
            data[y][..x].iter().all(|&n| n < pivot)
                || data[y][x + 1..].iter().all(|&n| n < pivot)
                || data[..y].iter().all(|v| v[x] < pivot)
                || data[y + 1..].iter().all(|v| v[x] < pivot)
        });
    println!("{}", rows * 2 + cols * 2 - 4 + visibles.count());

    // part two.
    let scenic_scores = (1..rows - 1)
        .flat_map(|y| (1..cols - 1).map(move |x| (y, x)))
        .map(|(y, x)| {
            let pivot = data[y][x];
            data[y][..x]
                .iter()
                .rev()
                .position(|&n| n >= pivot)
                .map_or(x, |x| x + 1)
                * data[y][x + 1..]
                    .iter()
                    .position(|&n| n >= pivot)
                    .map_or(cols - x - 1, |x| x + 1)
                * data[..y]
                    .iter()
                    .rev()
                    .position(|v| v[x] >= pivot)
                    .map_or(y, |y| y + 1)
                * data[y + 1..]
                    .iter()
                    .position(|v| v[x] >= pivot)
                    .map_or(rows - y - 1, |y| y + 1)
        });
    println!("{}", scenic_scores.max().unwrap());
}
