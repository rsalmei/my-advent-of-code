use crate::Input;
use std::str::FromStr;

pub fn run(input: Input) {
    let data = input.parse_lines::<Claim>();

    // part one.
    let mut grid = vec![0u16; 1000 * 1000];
    data.iter().for_each(|c| c.mark(&mut grid));
    let overlap = grid.into_iter().filter(|&x| x > 1).count();
    println!("{}", overlap);

    // part two.
    let unique = data.iter().find(|&c| {
        data.iter()
            .filter(|&c2| c.id != c2.id)
            .all(|c2| !c.overlap(c2))
    });
    println!("{}", unique.unwrap().id);
}

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    x2: usize,
    y2: usize,
}

impl Claim {
    fn mark(&self, grid: &mut [u16]) {
        for x in self.x..=self.x2 {
            for y in self.y..=self.y2 {
                grid[y * 1000 + x] += 1;
            }
        }
    }

    fn overlap(&self, other: &Self) -> bool {
        !(self.x2 < other.x || self.x > other.x2 || self.y2 < other.y || self.y > other.y2)
    }
}

impl FromStr for Claim {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // #1 @ 1,3: 4x4
        let mut it = s
            .split(&['#', ' ', '@', ',', ':', 'x'])
            .filter_map(|x| x.parse().ok());
        let (id, x, y) = (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());
        Ok(Claim {
            id,
            x,
            y,
            x2: x + it.next().unwrap() - 1,
            y2: y + it.next().unwrap() - 1,
        })
    }
}
