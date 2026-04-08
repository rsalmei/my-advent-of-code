use crate::Input;

pub fn run(input: Input) {
    let data = input.map_blocks(|block| {
        block
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    });

    // part one.
    let mut elf_cal = data
        .iter()
        .map(|group| group.iter().sum())
        .collect::<Vec<_>>();
    println!("{}", elf_cal.iter().max().unwrap());

    // part two.
    elf_cal.sort_unstable_by_key(|&c| -(c as i32));
    println!("{}", elf_cal.iter().take(3).sum::<u32>());
}
