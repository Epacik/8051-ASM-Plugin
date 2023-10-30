use std::fmt::Display;

use chumsky::Error as ChErr;
use IssueType::{Error, Info, Warning, Hint};

use crate::lexer::{
    tokens::Token,
    Position,
};

#[derive(Hash, Eq, PartialEq, Clone, Debug, Copy)]
pub enum IssueType {
    Error,
    Warning,
    Info,
    Hint,
}
impl Display for IssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error => write!(f, "Error"),
            Warning => write!(f, "Warning"),
            Info => write!(f, "Information"),
            Hint => write!(f, "Hint"),
        }
    }
}


#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct IssueInfo(u32, IssueType, String);

impl Display for IssueInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "asm8051({:04}) ({}): {}", self.0, self.1, self.2 )
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

    pub fn position(&self) -> &Position {
        &self.span
    }

    pub fn info(&self) -> &IssueInfo {
        &self.label
    }

    pub fn expected(&self) -> &[Option<Token>] {
        self.expected.as_ref()
    }

    pub fn found(&self) -> Option<&Token> {
        self.found.as_ref()
    }

    pub fn invalid_characters(&self) -> &[Option<char>] {
        self.invalid_characters.as_ref()
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
            label: issue_info(u32::MAX, Info, String::from("")), //t!("parser.error.empty_info")),
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

fn issue_info(code: u32, default_type: IssueType, message_key: String) -> IssueInfo {
    IssueInfo(code, default_type, message_key)
}

validate_issue_codes::validate_issuecode_uniqueness!(
    //#region errors

    pub(crate) fn unknown_token(position: Position, found: Token) -> Issue {
        Issue::new(
            position,
            issue_info(1000, Error, t!("parser.error.unknown_token")),
            None,
            Some(found),
            None,
        )
    }

    fn unclosed_delimiter_info() -> IssueInfo {
        issue_info(1001, Error, t!("parser.error.unclosed_string"))
    }

    pub(crate) fn unclosed_string(
        span: Position,
        expected: Token,
        actual: Option<Token>,
    ) -> Issue {

        Issue::unclosed_delimiter(
            span.clone(),
            expected.clone(),
            span,
            expected,
            actual,
        )
    }

    pub(crate) fn empty_string(position: Position) -> Issue {
        Issue::new(
            position,
            issue_info(1002, Error, t!("parser.error.empty_string")),
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
            issue_info(1003, Error, t!("parser.error.non-ascii_string")),
            None,
            None,
            non_ascii_characters,
        )
    }

    pub(crate) fn empty_escape_sequence(position: Position) -> Issue {
        Issue::new(
            position,
            issue_info(1004, Error, t!("parser.error.empty-escape_sequence")),
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
            std::num::IntErrorKind::Empty => t!("parser.error.empty-escape_sequence"),
            std::num::IntErrorKind::InvalidDigit => t!("parser.error.invalid-digit-hex-escape_sequence"),
            std::num::IntErrorKind::PosOverflow => t!("parser.error.positive-overflow-hex-escape_sequence"),
            std::num::IntErrorKind::NegOverflow => t!("parser.error.negative-overflow-hex-escape_sequence"),
            //std::num::IntErrorKind::Zero => t!("parser.error.zero-hex-escape_sequence"),
            _ => t!("parser.error.unknown_error"),
        };

        Issue::new(
            position,
            issue_info(1005, Error, err),
            None,
            None,
            None,
        )
    }

    pub(crate) fn invalid_escape_sequence(position: Position, seq: char) -> Issue {
        let seq = seq.to_string();
        Issue::new(
            position,
            issue_info(1006, Error, t!("parser.error.invalid-escape-sequence", sequence = seq)),
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
            issue_info(2000, Warning, t!("parser.warning.empty_comment")),
            None,
            None,
            None,
        )
    }
);


