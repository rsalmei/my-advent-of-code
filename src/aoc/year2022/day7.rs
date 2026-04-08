use crate::Input;
use std::collections::HashMap;
use std::str::FromStr;

pub fn run(input: Input) {
    let data = input.parse_lines();

    // part one.
    let sh = data.iter().fold(Sh::default(), |sh, row| sh.apply(row));
    let less_100k = sh
        .fs
        .iter()
        .filter_map(|(_, &size)| (size < 100000).then_some(size));
    println!("{}", less_100k.sum::<u32>());

    // part two.
    let mut sizes = sh.fs.iter().map(|(_, &s)| s).collect::<Vec<_>>();
    sizes.sort_unstable();
    let free = 70_000_000 - sh.fs[&vec!["/"]];
    println!(
        "{}",
        sizes.into_iter().find(|&s| free + s >= 30_000_000).unwrap()
    )
}

#[derive(Debug, Default)]
struct Sh<'a> {
    fs: HashMap<Vec<&'a str>, u32>,
    cd: Vec<&'a str>,
}

#[derive(Debug)]
enum Row {
    Ls,
    Dir,
    Parent,
    Cd(String),
    File(u32),
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_ascii_whitespace().collect::<Vec<_>>()[..] {
            ["$", "ls"] => Ok(Row::Ls),
            ["$", "cd", ".."] => Ok(Row::Parent),
            ["$", "cd", path] => Ok(Row::Cd(path.to_owned())),
            ["dir", _] => Ok(Row::Dir),
            [size, _] => Ok(Row::File(size.parse::<u32>().unwrap())),
            _ => unreachable!(),
        }
    }
}

impl<'a> Sh<'a> {
    fn apply(mut self, row: &'a Row) -> Self {
        match row {
            Row::Ls => {}
            Row::Dir => {}
            Row::Parent => {
                self.cd.pop();
            }
            Row::Cd(dir) => self.cd.push(dir),
            Row::File(size) => {
                let mut curr = self.cd.clone();
                loop {
                    match self.fs.get_mut(&curr) {
                        Some(x) => *x += *size,
                        None => {
                            self.fs.insert(curr.clone(), *size);
                        }
                    }
                    if curr.pop().is_some() && curr.is_empty() {
                        break;
                    }
                }
            }
        }
        self
    }
}
