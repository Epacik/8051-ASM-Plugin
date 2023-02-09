use std::fmt::Display;

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum IssueType {
    Error,
    Warning,
    Info,
}
impl Display for IssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error => write!(f, "Error"),
            Warning => write!(f, "Warning"),
            Info => write!(f, "Information"),
        }
    }
}

use chumsky::Error as ChErr;
use IssueType::{Error, Info, Warning};

use crate::lexer::{
    tokens::{Token, Trivia},
    Position,
};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct IssueInfo(u32, IssueType, String);

impl ToString for IssueInfo {
    fn to_string(&self) -> String {
        format!("8051E{:04} ({}): {}", self.0, self.1, self.2)
    }
}

impl IssueInfo {
    pub fn code(&self) -> u32 {
        self.0
    }
    pub fn default_type(&self) -> &IssueType {
        &self.1
    }
    pub fn message_key(&self) -> &str {
        self.2.as_ref()
    }
}

#[allow(unused)]
pub struct Issue {
    span: Position,
    label: IssueInfo,
    expected: Vec<Option<Token>>,
    found: Option<Token>,
    invalid_characters: Vec<Option<char>>,
}

impl Issue {
    pub fn new<
        TokenIter: IntoIterator<Item = Option<Token>>,
        InvalidCharsIter: IntoIterator<Item = Option<char>>,
    >(
        position: Position,
        info: IssueInfo,
        expected: TokenIter,
        found: Option<Token>,
        invalid_characters: InvalidCharsIter,
    ) -> Issue {
        Self {
            span: position,
            label: info,
            expected: expected.into_iter().collect::<Vec<Option<Token>>>(),
            found,
            invalid_characters: invalid_characters
                .into_iter()
                .collect::<Vec<Option<char>>>(),
        }
    }
}

impl std::fmt::Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [[{}]]", self.label.to_string(), self.span)
    }
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
            label: issue_info(u32::MAX, Info, "empty-info"),
            invalid_characters: vec![],
        }
    }

    fn with_label(mut self, label: Self::Label) -> Self {
        self.label = label;
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
        //Self::expected_input_found(span, Some(Some(expected)), found).with_label(UNCLOSED_DELIMITER);
        Self {
            span,
            expected: Some(Some(expected))
                .into_iter()
                .collect::<Vec<Option<Token>>>(),
            found,
            label: unclosed_delimiter_info(),
            invalid_characters: vec![],
        }
    }
}

// I may use it if I get validate_issuecode_uniqueness to work with it
// macro_rules! new_issue {
//     ($position:expr, $info:expr) => {
//         Issue::new($position, $info, None, None)
//     };
//     ($position:expr, $info:expr, found: $found:expr) => {
//         Issue::new($position, $info, None, Some($found))
//     };
//     ($position:expr, $info:expr, expected: $expected:expr) => {
//         Issue::new($position, $info, Some($expected), None)
//     };
//     ($position:expr, $info:expr, $expected:expr, $found:expr) => {
//         Issue::new($position, $info, Some($expected), Some($found))
//     };
// }

fn issue_info(code: u32, default_type: IssueType, message_key: &str) -> IssueInfo {
    IssueInfo(code, default_type, message_key.to_string())
}

validate_issue_codes::validate_issuecode_uniqueness!(
    //#region errors

    pub(crate) fn unknown_token(position: Position, found: Token) -> Issue {
        Issue::new(
            position,
            issue_info(1000, Error, "unknown-token"),
            None,
            Some(found),
            None,
        )
    }

    fn unclosed_delimiter_info() -> IssueInfo {
        issue_info(1001, Error, "unclosed-string")
    }

    pub(crate) fn unclosed_string<S: AsRef<str>>(
        span: Position,
        expected: Token,
        actual: S,
    ) -> Issue {
        let actual = actual.as_ref().to_string();

        Issue::unclosed_delimiter(
            span.clone(),
            expected.clone(),
            span,
            expected,
            Some(Token::Trivia(Trivia::NewLine(actual))),
        )
    }

    pub(crate) fn empty_string(position: Position) -> Issue {
        Issue::new(
            position,
            issue_info(1002, Error, "empty-string"),
            None,
            None,
            None,
        )
    }

    pub(crate) fn not_an_ascii_string(
        position: Position,
        non_ascii_characters: Vec<char>,
    ) -> Issue {
        let non_ascii_characters = non_ascii_characters.into_iter().map(|x| Some(x));
        Issue::new(
            position,
            issue_info(1003, Error, "not-an-ascii-string"),
            None,
            None,
            non_ascii_characters,
        )
    }

    pub(crate) fn empty_escape_sequence(position: Position) -> Issue {
        Issue::new(
            position,
            issue_info(1004, Error, "empty-escape-sequence"),
            None,
            None,
            None,
        )
    }

    pub(crate) fn invalid_hex_escape_sequence(
        position: Position,
        err: std::num::IntErrorKind,
    ) -> Issue {
        let err = match err {
            std::num::IntErrorKind::Empty => "empty",
            std::num::IntErrorKind::InvalidDigit => "invalid-digit",
            std::num::IntErrorKind::PosOverflow => "positive-overflow",
            std::num::IntErrorKind::NegOverflow => "negative-overflow",
            std::num::IntErrorKind::Zero => "zero",
            _ => "unknown",
        };

        let msg = format!("invalid-hex-escape-sequence--{err}");
        Issue::new(
            position,
            issue_info(1005, Error, msg.as_str()),
            None,
            None,
            None,
        )
    }

    pub(crate) fn invalid_escape_sequence(position: Position, seq: char) -> Issue {

        Issue::new(
            position,
            issue_info(1006, Error, format!("invalid-escape-sequence[{seq}]").as_str()),
            None,
            None,
            None,
        )
    }

    //#endregion

    //#region warnings

    pub(crate) fn empty_comment(position: Position) -> Issue {
        Issue::new(
            position,
            issue_info(2000, Warning, "empty-comment"),
            None,
            None,
            None,
        )
    }
);


