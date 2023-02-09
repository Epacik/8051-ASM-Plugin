#![cfg(test)]

use std::ops::Range;
use super::{initial::{SpannedString, InitialTokenType}, Position};

mod newline_whitespace_and_initial_char_type;
mod spanned_strings;
mod keywords;
mod uncategorised;
mod lexical_analysis;


pub(self) fn spanned_string(s: &str, range: Range<usize>, line: usize, columns: Range<usize>, initial_type: InitialTokenType) -> SpannedString {
    SpannedString::from_str(s, Position::new(range, line, columns), initial_type)
}