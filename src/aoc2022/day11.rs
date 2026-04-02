use crate::Input;
use std::collections::{BinaryHeap, VecDeque};
use std::str::FromStr;

pub fn run(input: Input) {
    let data = input.parse_blocks::<Monkey>();

    // part one.
    let mut monkeys = data.clone().into_vec();
    let monkey_business = sim(&mut monkeys, 20, 3);
    println!("{monkey_business}");

    // part one.
    let mut monkeys = data.into_vec();
    let monkey_business = sim(&mut monkeys, 10000, 1);
    println!("{monkey_business}");
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Operation,
    test: Test,
    inspected: u32,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Test {
    div: u64,
    is: usize,
    not: usize,
}

fn sim(monkeys: &mut Vec<Monkey>, rounds: usize, relief: u64) -> u64 {
    let gcd = monkeys.iter().fold(1, |acc, m| acc * m.test.div);
    (0..rounds).for_each(|_| {
        for mi in 0..monkeys.len() {
            while let Some((item, to)) = monkeys[mi].round(relief) {
                monkeys[to].items.push_back(item % gcd);
            }
        }
        // println!("round {r}");
        // monkeys
        //     .iter()
        //     .enumerate()
        //     .for_each(|(i, m)| println!("  monkey {i}: {:?} -> {}", m.items, m.inspected))
    });
    let mut inspected = monkeys
        .iter()
        .map(|m| m.inspected)
        .collect::<BinaryHeap<_>>();
    inspected.pop().unwrap() as u64 * inspected.pop().unwrap() as u64
}

impl Monkey {
    fn round(&mut self, relief: u64) -> Option<(u64, usize)> {
        let item = self.op.apply(self.items.pop_front()?) / relief;
        let to = match item % self.test.div == 0 {
            true => self.test.is,
            false => self.test.not,
        };
        self.inspected += 1;
        Some((item, to))
    }
}

impl Operation {
    fn apply(&self, item: u64) -> u64 {
        match *self {
            Operation::Add(x) => item + x,
            Operation::Mult(x) => item * x,
            Operation::Square => item * item,
        }
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut it = s.lines();
        let _monkey = it.next();
        let items = it.next().unwrap().trim_start_matches("  Starting items: ");
        let items = items.split(", ").map(|x| x.parse().unwrap()).collect();
        let op = it.next().unwrap().trim_start_matches("  Operation: new = ");
        let op = op.parse()?;
        let div = it
            .next()
            .unwrap()
            .trim_start_matches("  Test: divisible by ");
        let div = div.parse().unwrap();
        let is = it
            .next()
            .unwrap()
            .trim_start_matches("    If true: throw to monkey ");
        let is = is.parse().unwrap();
        let not = it
            .next()
            .unwrap()
            .trim_start_matches("    If false: throw to monkey ");
        let not = not.parse().unwrap();
        it.next(); // skip line.

        Ok(Monkey {
            items,
            op,
            test: Test { div, is, not },
            inspected: 0,
        })
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();
        let (_, op, v) = (it.next(), it.next(), it.next());
        match (op.unwrap(), v.unwrap()) {
            ("*", "old") => Ok(Operation::Square),
            ("*", x) => Ok(Operation::Mult(x.parse().unwrap())),
            ("+", x) => Ok(Operation::Add(x.parse().unwrap())),
            _ => unreachable!(),
        }
    }
}
