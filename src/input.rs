use std::fmt::Debug;
use std::str::FromStr;
use std::{fs, io};

/// The input for a day of Advent of Code.
///
/// The helper methods for parsing the input will panic if the code fails to parse it. That's
/// because the input is guaranteed to be valid, so any parsing errors are bugs in the code.
pub struct Input(String);

impl Input {
    pub fn read(year: u16, day: u8) -> io::Result<Self> {
        let path = format!("inputs/{year}/{day}");
        Ok(Self(fs::read_to_string(path)?))
    }

    /// Get the raw text of the input, without the trailing newline.
    pub fn text(mut self) -> Box<str> {
        let s = self.0.trim_end_matches('\n');
        self.0.truncate(s.len());
        self.0.into_boxed_str()
    }

    /// Get the raw bytes of the input, without the trailing newline.
    pub fn bytes(self) -> Box<[u8]> {
        self.text().into_boxed_bytes()
    }

    /// Map the lines of the input to an arbitrary type.
    pub fn map_lines<T>(self, f: impl FnMut(&str) -> T) -> Box<[T]> {
        // can't really avoid allocating here, since lines() returns an iterator of substrings of
        // the input, and even split_off reallocates the second half of the string.
        self.0.lines().map(f).collect()
    }

    /// Get the lines of the input.
    pub fn lines(self) -> Box<[Box<str>]> {
        self.map_lines(|s| s.into())
    }

    /// Parse the lines of the input as some context-specific type.
    pub fn parse_lines<T: FromStr<Err: Debug>>(self) -> Box<[T]> {
        self.map_lines(|s| s.parse().unwrap())
    }

    /// Map blocks of the input separated by empty lines to an arbitrary type.
    pub fn map_blocks<T>(self, f: impl FnMut(&str) -> T) -> Box<[T]> {
        self.text().split("\n\n").map(f).collect()
    }

    /// Get blocks of the input separated by empty lines.
    pub fn blocks(self) -> Box<[Box<str>]> {
        self.map_blocks(|s| s.into())
    }

    /// Parse blocks of the input separated by empty lines as some context-specific type.
    pub fn parse_blocks<T: FromStr<Err: Debug>>(self) -> Box<[T]> {
        self.map_blocks(|s| s.parse().unwrap())
    }
}
