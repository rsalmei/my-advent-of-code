use crate::Input;
use std::collections::HashSet;

pub fn run(input: Input) {
    let data = input.map_lines(|s| {
        let (direction, steps) = s.split_once(' ').unwrap();
        (direction.as_bytes()[0], steps.parse().unwrap())
    });

    // part one.
    fn sim(data: &[(u8, usize)], tail_size: usize) -> usize {
        let mut head = (0i32, 0i32);
        let mut rope = vec![(0, 0); tail_size];
        let mut visited = HashSet::new();
        data.iter().for_each(|&(s, steps)| {
            for _ in 0..steps {
                match s {
                    b'U' => head.1 += 1,
                    b'D' => head.1 -= 1,
                    b'L' => head.0 -= 1,
                    b'R' => head.0 += 1,
                    _ => unreachable!(),
                }
                rope.iter_mut().fold(head, |head, tail| {
                    if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                        tail.0 += (head.0 - tail.0).clamp(-1, 1);
                        tail.1 += (head.1 - tail.1).clamp(-1, 1);
                    }
                    *tail
                });
                visited.insert(rope[tail_size - 1]);
                // println!("{s} -> H {head:?}  T {tail:?} ({})", visited.len());
            }
        });
        visited.len()
    }
    println!("{}", sim(&data, 1));

    // part two.
    println!("{}", sim(&data, 9));
}
