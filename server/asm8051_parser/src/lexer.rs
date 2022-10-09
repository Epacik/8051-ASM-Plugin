use std::fmt::Display;
use chumsky::prelude::*;

pub type Span = std::ops::Range<usize>;

pub enum Token {
    Keyword(Keyword),
    Label(String),
    Address(String),
    Comment(String),
    Number(Number),
    ControlCharacter(char)
}

pub enum Keyword {
    Instruction(String),
    Register(String),
}

pub enum Number {
    Binary(String),
    Octal(String),
    Decimal(String),
    Hexadecimal(String)
}

pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

pub fn lexer() -> impl Parser<char, Vec<SpannedToken>, Error = Simple<char>> {
    let label = text::ident().map(|id: String| Token::Label(id));

    let comment = just(";")
    .ignore_then(filter(|c| *c != '\n').repeated())
    .then_ignore(just('\n'))
    .collect::<String>()
    .map(Token::Comment);
}


#[cfg(test)]
mod tests {

    #[test]
    fn t() {
        assert!(true);
    }
}