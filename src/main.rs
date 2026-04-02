mod aoc2018;
mod aoc2022;
mod input;

use clap::Parser;
use input::Input;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

type Days = [fn(Input)];
const AOC: &[(u16, &Days)] = include!(concat!(env!("OUT_DIR"), "/aoc-auto-detect.rs"));

#[derive(Debug, Parser)]
struct Args {
    /// An Advent of Code event.
    year: Option<u16>,
    /// A challenge within an event.
    day: Option<u8>,
    /// Do not print elapsed times.
    #[arg(short, long, default_value_t = false)]
    clean: bool,
}

fn main() {
    println!("My Advent of Code");
    println!("-----------------");

    let Args { year, day, clean } = Args::parse();
    let (year, day) = (year.unwrap_or_default(), day.unwrap_or_default());

    let aoc = AOC
        .iter()
        .map(|&(y, r)| (y, (1..).zip(r).collect::<BTreeMap<_, _>>()))
        .collect::<BTreeMap<_, _>>();
    match aoc.get(&year) {
        Some(runs) => match runs.get(&day) {
            Some(run) => {
                println!("year: {year}  day: {day}\n");
                runner(|| run(Input::read(year, day).unwrap()), clean);
            }
            None => {
                println!("year: {year}");
                let mut total = Duration::from_secs(0);
                for (&day, run) in runs {
                    println!("\nday: {day}");
                    total += runner(|| run(Input::read(year, day).unwrap()), clean);
                }
                println!("\ntotal -- {total:?}");
            }
        },
        None => {
            println!("\nAvailable (calling with only the year runs all days in there):");
            aoc.iter().for_each(|(y, r)| {
                print!("* {y}\n  ");
                (1..=r.len()).for_each(|d| print!(" {d}"));
                println!();
            });
        }
    }
}

fn runner(run: &dyn Fn(), clean: bool) -> Duration {
    let start = Instant::now();
    run();
    let end = start.elapsed();
    if !clean {
        println!("-- {end:?}");
    }
    end
}
