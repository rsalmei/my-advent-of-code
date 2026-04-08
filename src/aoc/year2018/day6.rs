use crate::Input;
use std::collections::HashSet;
use std::ops::{Add, Rem};
use Location::*;

pub fn run(input: Input) {
    let data = input.map_lines(|s| {
        let (x, y) = s.split_once(", ").unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    });

    // part one.
    let (xm, ym) = (
        data.iter().map(|&(x, _)| x).max().unwrap(),
        data.iter().map(|&(_, y)| y).max().unwrap(),
    );
    let fill = |grid: &mut Vec<Vec<Location>>, op: fn(Location, Location) -> Location| {
        (0..=xm)
            .flat_map(|x| (0..=ym).map(move |y| (x, y)))
            .for_each(|(x, y)| {
                grid[y][x] = data
                    .iter()
                    .enumerate()
                    .map(|(who, &o)| Dist {
                        who,
                        dist: dist(x, y, o),
                    })
                    .fold(Clear, op)
            })
    };

    let mut grid = vec![vec![Clear; xm + 1]; ym + 1];
    fill(&mut grid, |acc, x| acc % x);
    let infinite = grid[0]
        .iter()
        .chain(&grid[ym])
        .chain(grid[1..ym].iter().flat_map(|r| [&r[0], &r[xm]]))
        .filter_map(|loc| match loc {
            Dist { who, .. } => Some(*who),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let areas = (0..data.len()).filter(|i| !infinite.contains(i)).map(|i| {
        grid[1..ym]
            .iter()
            .flat_map(|r| &r[1..xm])
            .filter(|loc| matches!(loc, Dist { who, .. } if i == *who))
            .count()
    });
    println!("{}", areas.max().unwrap());

    // part two.
    fill(&mut grid, |acc, x| acc + x);
    let size = grid[1..ym]
        .iter()
        .flat_map(|r| &r[1..xm])
        .filter(|loc| matches!(loc, Tie { dist } if *dist < 10_000));
    println!("{}", size.count());
}

#[derive(Debug, Copy, Clone)]
enum Location {
    Dist { dist: usize, who: usize },
    Tie { dist: usize },
    Clear,
}

impl Rem for Location {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Dist { dist: da, .. }, Dist { dist: db, .. }) if da == db => Tie { dist: da },
            (Dist { dist: da, .. }, Dist { dist: db, .. }) if da < db => self,
            (Dist { .. }, Dist { .. }) => rhs,
            (Tie { dist: da, .. }, Dist { dist: db, .. }) if da == db => self,
            (Tie { dist: da, .. }, Dist { dist: db, .. }) if da < db => self,
            (Tie { .. }, Dist { .. }) => rhs,
            (Clear, rhs) => rhs,
            _ => unreachable!(),
        }
    }
}

impl Add for Location {
    type Output = Location;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Dist { dist: da, .. } | Tie { dist: da }, Dist { dist: db, .. }) => {
                Tie { dist: da + db }
            }
            (Clear, rhs) => rhs,
            _ => unreachable!(),
        }
    }
}

fn dist(x: usize, y: usize, (ox, oy): (usize, usize)) -> usize {
    x.abs_diff(ox) + y.abs_diff(oy)
}
