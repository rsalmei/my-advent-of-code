use crate::Input;

pub fn run(input: Input) {
    let serial = input.text().parse::<usize>().unwrap();
    const SIZE: usize = 300;

    // part one.
    let mut grid = vec![vec![0; SIZE + 1]; SIZE + 1];
    grid.iter_mut().enumerate().for_each(|(y, r)| {
        r.iter_mut().enumerate().for_each(|(x, cell)| {
            let rack_id = x + 10;
            let tmp = ((rack_id * y + serial) * rack_id / 100) as isize;
            *cell = tmp - tmp / 10 * 10 - 5;
        })
    });
    // let max_total = |size| {
    //     let rblock = grid
    //         .iter()
    //         .map(|r| r.iter().take(size).sum::<i32>())
    //         .collect::<Vec<_>>();
    //     (1..=SIZE - size + 1).flat_map(|x| {})
    // };
    let max_total = |size: usize| {
        (1..=SIZE - size + 1)
            .flat_map(|x| (1..=SIZE - size + 1).map(move |y| (x, y)))
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
    let (size, ((max_x, max_y), _total)) = (1..=SIZE)
        .map(|size| (size, max_total(size).unwrap()))
        .max_by_key(|&(_, (_, total))| total)
        .unwrap();
    println!("{max_x},{max_y},{size}");
}

// fn solve_part2(grid_size: usize, sn: usize) -> (usize, usize, usize, i32) {
//     let mut best = (0, 0, 0, -10000);
//     for bs in 1..=300 {
//         let (x, y, p) = solve(grid_size, sn, bs);
//         if p > best.3 {
//             best = (x, y, bs, p);
//         }
//     }
//     best
// }
//
// fn solve(grid_size: usize, sn: usize, bs: usize) -> (usize, usize, i32) {
//     let mut best = (0, 0, -10000);
//
//     // Create a list of sums of vertical slices of the power levels starting at y=0 and height of
//     // the block size. One for every x coordinate.
//     let mut chunks: Vec<_> = (0..grid_size)
//         .map(|x| (0..bs).map(|y| power_level(x, y, sn)).sum::<i32>())
//         .collect();
//
//     for y in 0..grid_size - (bs - 1) {
//         // Calculate the total power of the first block at (x=0, y)
//         let mut total = chunks.iter().take(bs).sum::<i32>();
//         // For every x, update the total power by removing the leftmost chunk and adding one to the
//         // right.
//         for x in 0..grid_size - (bs - 1) {
//             // First update the best score
//             if total > best.2 {
//                 best = (x, y, total);
//             }
//             if x < grid_size - bs {
//                 total += chunks[x + bs] - chunks[x];
//             }
//         }
//
//         // After a horizontal scan, move all the chunks one position down by subtracting the topmost
//         // row of power levels and adding one to the bottom.
//         if y < grid_size - bs {
//             for x in 0..grid_size - (bs - 1) {
//                 chunks[x] += power_level(x, y + bs, sn) - power_level(x, y, sn);
//             }
//         }
//     }
//     best
// }
//
// fn power_level(x: usize, y: usize, sn: usize) -> i32 {
//     let rack = x + 10;
//     let power = rack * y;
//     let power = power + sn;
//     let power = power * rack;
//     let power = (power % 1000) / 100;
//     power as i32 - 5
// }
