use crate::Input;
use std::collections::BTreeMap;
use std::str::FromStr;

pub fn run(input: Input) {
    let mut data = input.lines();

    // part one.
    data.sort_unstable();
    let data = data
        .iter()
        .map(|d| d.parse::<Record>().unwrap())
        .scan(Slept::default(), |acc, r| {
            match r {
                Record::BeginShift(id) => acc.guard = id,
                Record::FallAsleep(m) => acc.start = m,
                Record::WakeUp(m) => {
                    acc.duration = m - acc.start;
                    return Some(Some(*acc));
                }
            }
            Some(None)
        })
        .flatten()
        .collect::<Vec<_>>();
    fn find_max<T: Ord>(it: impl Iterator<Item = (T, u32)>) -> (T, u32) {
        it.fold(BTreeMap::new(), |mut acc, (k, c)| {
            *acc.entry(k).or_insert(0u32) += c;
            acc
        })
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap()
    }

    let guard = find_max(data.iter().map(|s| (s.guard, s.duration as u32)));
    let minute = find_max(
        data.iter()
            .filter(|s| s.guard == guard.0)
            .flat_map(|s| (s.start..s.start + s.duration).map(|m| (m, 1))),
    );
    println!("{}", guard.0 as u32 * minute.0 as u32);

    // part two.
    let guard_minute = find_max(
        data.iter()
            .flat_map(|s| (s.start..s.start + s.duration).map(|m| ((s.guard, m), 1))),
    );
    println!("{}", guard_minute.0 .0 as u32 * guard_minute.0 .1 as u32);
}

#[derive(Debug, Default, Copy, Clone)]
struct Slept {
    guard: u16,
    start: u8,
    duration: u8,
}

#[derive(Debug)]
enum Record {
    BeginShift(u16),
    FallAsleep(u8),
    WakeUp(u8),
}

impl FromStr for Record {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // [1518-11-19 23:56] Guard #73 begins shift
        // [1518-11-20 00:41] falls asleep
        // [1518-11-20 00:57] wakes up
        if let Some(pos) = s.find('#') {
            let id = s[pos + 1..].split_ascii_whitespace().next().unwrap();
            Ok(Record::BeginShift(id.parse()?))
        } else {
            let minute = s
                .split(&[':', ']'])
                .filter_map(|n| n.parse().ok())
                .next()
                .unwrap();
            if s.contains("falls") {
                Ok(Record::FallAsleep(minute))
            } else {
                Ok(Record::WakeUp(minute))
            }
        }
    }
}
