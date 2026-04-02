use crate::Input;
use std::str::FromStr;

pub fn run(input: Input) {
    let data = input.parse_lines();

    // part one.
    let (mut cpu, mut mem) = (Cpu::new(), data.iter());
    let (mut clock, mut signal) = (0, 0);
    while clock < 220 {
        cpu.fetch_instruction(&mut mem);
        clock += 1;
        // println!("{clock}: {:?} ({})", cpu.running, cpu.reg_x);

        if (clock - 20) % 40 == 0 {
            signal += clock * cpu.reg_x
        }

        cpu.tick();
        // println!("  --> {}", cpu.reg_x);
    }
    println!("{signal}");

    // part two.
    let (mut cpu, mut mem) = (Cpu::new(), data.iter());
    clock = 0;
    while clock < 240 {
        cpu.fetch_instruction(&mut mem);
        clock += 1;
        // println!("{clock}: {:?} ({})", cpu.running, cpu.reg_x);

        let pos = (clock - 1) % 40;
        if clock > 1 && pos == 0 {
            println!();
        }
        match pos {
            _ if (cpu.reg_x - pos).abs() <= 1 => print!("#"),
            _ => print!("."),
        }

        cpu.tick();
        // println!("  --> {}", cpu.reg_x);
    }
    println!();
}

struct Cpu<'a> {
    reg_x: i32,
    running: Option<(&'a Cmd, u8)>,
}

impl<'a> Cpu<'a> {
    pub fn new() -> Self {
        Self {
            reg_x: 1,
            running: None,
        }
    }
    pub fn tick(&mut self) {
        if let Some((cmd, mut cycles)) = self.running.take() {
            cycles -= 1;
            match cycles {
                0 => match cmd {
                    Cmd::Addx(x) => self.reg_x += x,
                    Cmd::Noop => {}
                },
                _ => self.running = Some((cmd, cycles)),
            }
        }
    }
    pub fn fetch_instruction(&mut self, mem: &mut impl Iterator<Item = &'a Cmd>) {
        if self.running.is_none() {
            if let Some(cmd) = mem.next() {
                let cycles = match cmd {
                    Cmd::Addx(_) => 2,
                    Cmd::Noop => 1,
                };
                self.running = Some((cmd, cycles))
            }
        }
    }
}

#[derive(Debug)]
enum Cmd {
    Addx(i32),
    Noop,
}

impl FromStr for Cmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();
        let c = match it.next().unwrap() {
            "addx" => Cmd::Addx(it.next().unwrap().parse().unwrap()),
            "noop" => Cmd::Noop,
            _ => unreachable!(),
        };
        Ok(c)
    }
}
