use crate::Input;

pub fn run(input: Input) {
    let serial = input.lines().next().unwrap().parse::<usize>().unwrap();

    // part one.
    let mut grid = vec![vec![0; 301]; 301];
    grid.iter_mut().enumerate().for_each(|(y, r)| {
        r.iter_mut().enumerate().for_each(|(x, cell)| {
            let rack_id = x + 10;
            let tmp = ((rack_id * y + serial) * rack_id / 100) as isize;
            *cell = tmp - tmp / 10 * 10 - 5;
        })
    });
    let max_total = |size: usize| {
        (1..=300 - size + 1)
            .flat_map(|x| (1..=300 - size + 1).map(move |y| (x, y)))
            .map(|(x, y)| {
                (
                    (x, y),
                    grid[y..y + size]
                        .iter()
                        .flat_map(|r| r[x..x + size].iter())
                        .sum::<isize>(),
                )
            })
            .max_by_key(|&(_, total)| total)
    };
    let ((max_x, max_y), _total) = max_total(3).unwrap();
    println!("{max_x},{max_y}");

    // part two.
    let (size, ((max_x, max_y), _total)) = (1..=300)
        .map(|size| (size, max_total(size).unwrap()))
        .max_by_key(|&(_, (_, total))| total)
        .unwrap();
    println!("{max_x},{max_y},{size}");
}
