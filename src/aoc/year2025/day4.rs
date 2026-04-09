use crate::input::Input;

pub fn run(input: Input) {
    let mut data = input
        .lines()
        .into_vec()
        .into_iter()
        .map(|s| s.into_boxed_bytes())
        .collect::<Vec<_>>();

    // part one.
    let count = find_accessible(&data).count();
    println!("{count}");

    // part two.
    let mut count = 0;
    loop {
        let accessible = find_accessible(&data).collect::<Vec<_>>();
        if accessible.is_empty() {
            break;
        }
        count += accessible.len();
        accessible.into_iter().for_each(|(y, x)| {
            data[y][x] = b'.';
        });
    }
    println!("{count}");
}

fn find_accessible(data: &[Box<[u8]>]) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..data.len())
        .flat_map(|y| (0..data[0].len()).map(move |x| (y, x)))
        .filter(|&(y, x)| data[y][x] == b'@')
        .filter(|&(y, x)| {
            let neighbors = (-1isize..=1)
                .flat_map(|dy| (-1isize..=1).map(move |dx| (dy, dx)))
                .filter(|&(dy, dx)| dy != 0 || dx != 0)
                .filter_map(|(dy, dx)| {
                    let ey = y.checked_add_signed(dy)?;
                    let ex = x.checked_add_signed(dx)?;
                    data.get(ey)?.get(ex)
                })
                .filter(|&&b| b == b'@')
                .count();
            neighbors < 4
        })
}
