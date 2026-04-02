use crate::Input;
use std::str::FromStr;

pub fn run(input: Input) {
    let data = input.blocks();
    let (stack, moves) = (&data[0], &data[1]);

    // part one.
    let stack_num = stack.lines().next_back().unwrap();
    let stack_num = stack_num.split_ascii_whitespace().next_back().unwrap();
    let stack_num = stack_num.parse::<usize>().unwrap();

    let mut stacks = (0..stack_num)
        .map(|n| {
            stack
                .lines()
                .rev()
                .map(|s| s.chars().nth(1 + 4 * n).unwrap())
                .filter(|&b| b != ' ')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    {
        let mut stacks = stacks.clone();
        moves
            .lines()
            .map(|s| s.parse::<Move>().unwrap())
            .for_each(|m| m.apply_single(&mut stacks));
        let top = stacks
            .iter()
            .map(|v| *v.last().unwrap())
            .collect::<String>();
        println!("{top}");
    }

    // part two.
    moves
        .lines()
        .map(|s| s.parse::<Move>().unwrap())
        .for_each(|m| m.apply_multi(&mut stacks));
    let top = stacks
        .iter()
        .map(|v| *v.last().unwrap())
        .collect::<String>();
    println!("{top}");
}

#[derive(Debug)]
struct Move {
    qty: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn apply_single(&self, stacks: &mut [Vec<char>]) {
        (0..self.qty).for_each(|_| {
            let from = stacks[self.from].pop().unwrap();
            stacks[self.to].push(from)
        })
    }

    fn apply_multi(&self, stacks: &mut [Vec<char>]) {
        let from = &mut stacks[self.from];
        let from = from
            .drain(from.len() - self.qty..from.len())
            .collect::<Vec<_>>();
        stacks[self.to].extend(from)
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s
            .split(|c: char| !c.is_ascii_digit())
            .filter_map(|s| s.parse().ok());
        Ok(Self {
            qty: nums.next().unwrap(),
            from: nums.next().unwrap() - 1,
            to: nums.next().unwrap() - 1,
        })
    }
}
