aoc! {
    2018: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    2022: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    2025: [1, 2, 3, 4],
}

/// Register Advent of Code solutions.
///
/// Generate module declarations and a [collect_years] function that returns a nested
/// `BTreeMap` mapping years and days to their respective `run` functions.
#[macro_export]
macro_rules! _aoc {
    ($($year:literal: [$($day:literal),* $(,)?]),* $(,)?) => {
        paste::paste! {
            $(
                mod [<year $year>] {
                    $(pub mod [<day $day>];)*
                }
            )*

            use crate::input::Input;
            use std::collections::BTreeMap;
            pub fn collect_years() -> BTreeMap<u16, BTreeMap<u8, fn(Input)>> {
                BTreeMap::from([
                    $(
                        ($year, BTreeMap::from([$(($day, [<year $year>]::[<day $day>]::run as _)),*])),
                    )*
                ])
            }
        }
    };
}
use _aoc as aoc;
