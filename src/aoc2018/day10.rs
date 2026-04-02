use crate::Input;
use std::str::FromStr;

pub fn run(input: Input) {
    let data = input.parse_lines::<Point>();

    // part one.
    let mut sky = Sky {
        points: data.into_vec(),
    };
    let mut previous = i64::MAX;
    let mut frame = 0;
    loop {
        sky.advance(1);
        frame += 1;
        let (x, y, xx, yy) = sky.bounding_box();
        let new = (xx - x) as i64 * (yy - y) as i64;
        match new < previous {
            true => previous = new,
            false => {
                sky.advance(-1);
                sky.render();
                break;
            }
        }
    }

    // part two.
    println!("{}", frame - 1);
}

struct Sky {
    points: Vec<Point>,
}

impl Sky {
    fn advance(&mut self, dir: i32) {
        self.points.iter_mut().for_each(|p| p.advance(dir))
    }
    fn bounding_box(&self) -> (i32, i32, i32, i32) {
        let (mut x, mut y, mut xx, mut yy) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        self.points.iter().for_each(|p| {
            x = x.min(p.pos.0);
            y = y.min(p.pos.1);
            xx = xx.max(p.pos.0);
            yy = yy.max(p.pos.1);
        });
        (x, y, xx, yy)
    }
    fn render(&self) {
        let (x, y, xx, yy) = self.bounding_box();
        let mut board = vec![vec!['.'; (xx - x + 1) as usize]; (yy - y + 1) as usize];
        self.points.iter().for_each(|p| {
            if let Some(r) = board.get_mut((p.pos.1 - y) as usize) {
                if let Some(c) = r.get_mut((p.pos.0 - x) as usize) {
                    *c = '#'
                }
            }
        });
        board.iter().for_each(|r| {
            r.iter().for_each(|c| print!("{c}"));
            println!();
        });
    }
}

struct Point {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Point {
    fn advance(&mut self, dir: i32) {
        self.pos.0 += dir * self.vel.0;
        self.pos.1 += dir * self.vel.1;
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(|c| "<>, ".contains(c)).filter(|&s| !s.is_empty());
        let mut get = |n| it.nth(n).unwrap().parse().unwrap();
        Ok(Self {
            pos: (get(1), get(0)),
            vel: (get(1), get(0)),
        })
    }
}
