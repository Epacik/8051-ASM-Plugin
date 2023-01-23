use std::fmt::Display;


#[derive(Hash, Eq, PartialEq)]
pub enum IssueType { Error, Warning, Info }
impl Display for IssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error   => write!(f, "Error"),
            Warning => write!(f, "Warning"),
            Info    => write!(f, "Information"),
        }
    }
}


use IssueType::{ Error, Warning, Info };

use crate::lexer::{Position, Token};

#[derive(Hash, Eq, PartialEq)]
pub struct IssueInfo(i32, IssueType, String);

impl ToString for IssueInfo {
    fn to_string(&self) -> String {
        format!("8051E{:04} ({}): {}", self.0, self.1, self.2)
    }
}

impl IssueInfo {
    pub fn code(&self) -> i32 { self.0 }
    pub fn default_type(&self) -> &IssueType { &self.1 }
    pub fn message_key(&self) -> &str { self.2.as_ref() }
}

pub struct Issue {
    span: Position,
    label: Option<IssueInfo>,
    expected: Vec<Option<Token>>,
    found: Option<Token>
}

impl chumsky::Error<Token> for Issue {
    type Span = Position;

    type Label = IssueInfo;
    
    fn expected_input_found<Iter: IntoIterator<Item = Option<Token>>>(
        span: Self::Span,
        expected: Iter,
        found: Option<Token>,
    ) -> Self {
        Self { 
            span,
            expected: expected.into_iter().collect::<Vec<Option<Token>>>(),
            found,
            label: None,
         }
    }

    fn with_label(mut self, label: Self::Label) -> Self {
        self.label.get_or_insert(label);
        self
    }

    fn merge(mut self, other: Self) -> Self {
        for expected in other.expected {
            self.expected.push(expected);
        }
        self
    }

    fn unclosed_delimiter(
        unclosed_span: Self::Span,
        unclosed: Token,
        span: Self::Span,
        expected: Token,
        found: Option<Token>,
    ) -> Self {
        #![allow(unused_variables)]
        Self::expected_input_found(span, Some(Some(expected)), found).with_label(UNCLOSED_DELIMITER)
    }
}


fn issue(code: i32, default_type: IssueType, message_key: &str) -> IssueInfo{
    IssueInfo(code, default_type, message_key.to_string())
}



pub(crate) static UNKNOWN_TOKEN: IssueInfo = issue(0, Error, "unknown-token");
pub(crate) static UNCLOSED_DELIMITER: IssueInfo = issue(1, Error, "unclosed-delimiter");