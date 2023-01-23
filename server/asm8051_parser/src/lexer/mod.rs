use std::{ops::Range, borrow::Borrow};
use chumsky::prelude::Simple;
use ropey::Rope;

mod tests;

mod analysis;
mod initial;
mod keywords;
mod extensions;

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
}

//#endregion

//#region Tokens

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Label(String),
    Address(Number),
    String(String),
    Number(Number),
    ControlCharacter(ControlCharacter),
    Trivia(Trivia),
    Other(String),
    Unknown(String),
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Instruction(String),
    Register(Register),
    Directive(String),
    FlagOrBit(String),
}

#[derive(Debug, PartialEq)]
pub enum Register {
    Main(String),
    Special(String),
    Helper(String),
    Port(String),
    Addressing(String)
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Binary(String),
    Octal(String),
    Decimal(String),
    Hexadecimal(String)
}

#[derive(Debug, PartialEq)]
pub enum Trivia {
    NewLine(String),
    WhiteSpace(String),
    Comment(String),
}
#[derive(Debug, PartialEq)]
pub enum ControlCharacter {
    Arithmetic(Arithmetic),
    AddressingIndicator,
    ArgumentSeparator,
    Parenthesis(Parenthesis)
}

#[derive(Debug, PartialEq)]
pub enum Arithmetic {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug, PartialEq)]
pub enum Parenthesis {
    Open,
    Close
}

//#endregion

//#region positioned token

#[derive(Debug, PartialEq)]
pub struct PositionedToken {
    pub token: Token,
    pub position: Position,
}

impl std::fmt::Display for PositionedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, range: {}-{}; line: {}; columns: {}-{}",
            match self.token.borrow() {
                Token::Label(lb) => format!("Label: {}, range", lb.clone()),
                _ => todo!(),
            },
            self.position.range.start,
            self.position.range.end,
            self.position.line,
            self.position.columns.start,
            self.position.columns.end
        )
    }
}

impl PositionedToken {
    pub fn new(token: Token, position: Position) -> PositionedToken {
        PositionedToken { token, position }
    }
}
//#endregion
//#endregion

pub fn lexical_analisys(source: &str) -> (Option<Vec<PositionedToken>>, Vec<Simple<String, Position>>) {

    let rope = Rope::from_str(source);
    let spans = initial::get_spanned_strings(rope);
    let lines = initial::split_spanned_strings_into_lines(spans);
    print!("test {}", 12);
    analysis::perform_analysis(lines)
}