/// Register Advent of Code solutions.
///
/// Generate module declarations and a [collect_aoc] function that returns a nested
/// `BTreeMap` mapping years and days to their respective `run` functions.
#[macro_export]
macro_rules! aoc {
    ($($year:literal: [$($day:literal),* $(,)?]),* $(,)?) => {
        paste::paste! {
            $(
                mod [<aoc $year>] {
                    $(pub mod [<day $day>];)*
                }
            )*

            fn collect_aoc() -> BTreeMap<u16, BTreeMap<u8, fn(Input)>> {
                BTreeMap::from([
                    $(
                        ($year, BTreeMap::from([$(($day, [<aoc $year>]::[<day $day>]::run as fn(Input))),*])),
                    )*
                ])
            }
        }
    };
}
