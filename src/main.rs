mod aoc;
mod input;

use clap::Parser;
use input::Input;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

#[derive(Debug, Parser)]
struct Args {
    /// An Advent of Code event year.
    year: Option<u16>,
    /// A day within the specified year.
    day: Option<u8>,
    /// Do not print elapsed times.
    #[arg(short, long, default_value_t = false)]
    clean: bool,
}

aoc! {
    2018: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    2022: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    2025: [1, 2],
}

fn main() {
    println!("My Advent of Code");
    println!("-----------------");

    let Args { year, day, clean } = Args::parse();
    let year = year.unwrap_or_default();

    let aoc = collect_aoc(); // collect the implemented puzzles.
    match (aoc.get(&year), day) {
        (Some(runs), Some(d)) if runs.contains_key(&d) => {
            println!("year: {year}  day: {d}\n");
            // SAFETY: day was just confirmed to exist.
            runner(year, d, &runs[&d], clean);
        }
        (Some(runs), Some(d)) => {
            println!("day {d} not found in year {year}; available:");
            runs.keys().for_each(|d| print!(" {d}"));
            println!();
        }
        (Some(runs), None) => {
            println!("year: {year}");
            let total = runs
                .iter()
                .map(|(&d, run)| {
                    println!("\nday: {d}");
                    runner(year, d, run, clean)
                })
                .sum::<Duration>();
            println!("\ntotal -- {total:?}");
        }
        (None, _) if year > 0 => {
            println!("year {year} not found; available:");
            aoc.keys().for_each(|y| print!(" {y}"));
            println!();
        }
        _ => {
            println!("Available: (calling with only the year runs all days in there)");
            aoc.iter().for_each(|(y, runs)| {
                print!("* {y}\n  ");
                runs.keys().for_each(|d| print!(" {d}"));
                println!();
            });
        }
    }
}

fn runner(year: u16, day: u8, run: &dyn Fn(Input), clean: bool) -> Duration {
    let input = Input::read(year, day).unwrap();
    let start = Instant::now();
    run(input);
    let end = start.elapsed();
    if !clean {
        println!("-- {end:?}");
    }
    end
}
