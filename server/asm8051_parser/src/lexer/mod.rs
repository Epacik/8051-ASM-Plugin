use std::ops::Range;
use ropey::Rope;

use crate::issues::Issue;

use self::tokens::PositionedToken;

mod tests;

mod analysis;
mod initial;
pub mod keywords;
mod extensions;
pub mod tokens;

//#region Types

//#region Position
#[derive(PartialEq, Clone)]
pub struct Position {
    pub range: Range<usize>,
    pub line: usize,
    pub columns: Range<usize>
}
impl std::fmt::Debug for Position {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TextPosition")
            .field("range", &self.range)
            .field("line", &self.line)
            .field("columns", &self.columns)
            .finish()
    }   
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "range: {}:{}; line: {}; columns: {}:{}", 
            self.range.start, self.range.end, 
            self.line, 
            self.columns.start, self.columns.end)
    }
}

impl chumsky::Span for Position {
    type Context = ();
    
    type Offset = usize;

    fn new(_: Self::Context, range: Range<Self::Offset>) -> Self {
        Position { range: range.clone(), line: 0, columns: range }
    }

    fn context(&self) -> Self::Context {}
    
    fn start(&self) -> Self::Offset {
        self.range.start
    }

    fn end(&self) -> Self::Offset {
        self.range.end
    }
}

impl Position {
    pub fn new(range: Range<usize>, line: usize, columns: Range<usize>) -> Position {
        Position { range, line, columns }
    }

    pub fn contains_column(&self, col: usize) -> bool {
        let range = &self.columns;
        range.start <= col && range.end >= col
    }
}

//#endregion

pub fn lexical_analysis<S: AsRef<str>>(s: S)-> (Option<Vec<PositionedToken>>, Vec<Issue>) {
    let source = s.as_ref();

    let rope = Rope::from_str(source);
    let spans = initial::get_spanned_strings(rope);
    let lines = initial::split_spanned_strings_into_lines(spans);
    analysis::perform_analysis(lines)
}

pub fn get_label_definitions(tokens: &Vec<PositionedToken>) -> Vec<String> {
    tokens
        .iter()
        .filter(|x| x.token.is_label())
        .map(|x| x.token.unwrap_label())
        .collect::<Vec<String>>()
}

pub fn get_line(tokens: &Vec<PositionedToken>, line: usize) -> Vec<PositionedToken> {
    tokens
        .iter()
        .filter(|x| x.position.line == line)
        .map(|x| x.clone())
        .collect::<Vec<PositionedToken>>()
}