use std::ops::Range;
use chumsky::prelude::*;
use ropey::Rope;
use crate::lexer_old::{self, SpannedToken};

#[derive(Debug)]
#[derive(PartialEq)]
struct TextPosition {
    pub range: Range<usize>,
    pub line: usize,
    pub columns: Range<usize>
}

impl TextPosition {
    pub fn new(range: Range<usize>, line: usize, columns: Range<usize>) -> TextPosition {
        TextPosition { range, line, columns }
    }
}

struct SpannedString {
    pub string: String,
    pub position: TextPosition,
}

impl SpannedString {
    pub fn new(string: String, position: TextPosition) -> SpannedString {
        SpannedString { string, position }
    }

    pub fn from_str(string: &str, position: TextPosition) -> SpannedString {
        SpannedString { string: String::from(string), position }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum InitialCharType {
    None,
    NewLine,
    WhiteSpace,
    Control,
    Other,
}

fn is_control_char(c: char) -> bool {
    match c {
        ';' | ':' => true,
        '"' | '\'' => true,
        '(' | ')' => true,
        ',' => true,
        '#' => true,
        '@' => true,
        _ => false,
    }
}

fn is_newline(c: char) -> bool {
    match c {
        '\n' | '\r' | '\x0B' | '\x0C' | '\u{0085}' | '\u{2028}' | '\u{2029}' => true,
        _ => false
    }
}

fn is_whitespace(c: char, match_newline: bool) -> bool {
    if match_newline {
        c.is_whitespace() || is_newline(c)
    }
    else {
        c.is_whitespace() && !is_newline(c)
    }
}

fn get_initial_char_type(c: char) -> InitialCharType {
    if is_newline(c) {
        InitialCharType::NewLine
    }
    else if is_control_char(c) {
        InitialCharType::Control
    }
    else if is_whitespace(c, false){
        InitialCharType::WhiteSpace
    }
    else {
        InitialCharType::Other
    }
}

fn get_spanned_strings(source: Rope) -> Vec<SpannedString> {

    let mut spans = Vec::<SpannedString>::new();

    // let's  go trough all of the lines
    for line_idx in 0..(source.len_lines()) {
        
        //if, for some reason, we ventured outside of the bounds of provided source, let's just break out 
        let line = match source.get_line(line_idx) {
            Some(l) => l,
            None => break,
        };

        // index of a first char of a current line in a source text
        let line_char_idx = source.line_to_char(line_idx);

        let mut buf = Vec::<(char, InitialCharType, usize)>::new();

        let push_buf_to_spans = |buf: &Vec<(char, InitialCharType, usize)>, spans: &mut  Vec<SpannedString>| {
            let start_column = buf.first().unwrap().2;
            let end_column = buf.last().unwrap().2;

            let out_str = buf.iter().map(|x| x.0).collect::<String>();

            let position = TextPosition::new(
                (line_char_idx+start_column)..(line_char_idx+end_column), 
                line_idx, 
                start_column..end_column);

            spans.push(SpannedString::new(out_str, position));
        };

        //we're going trough the line
        let chars = line.chars().collect::<Vec<char>>();
        for char_idx in 0..(line.len_chars()) {
            let ch = chars[char_idx];
            
            let buf_len = buf.len();
    
            let char_type = get_initial_char_type(ch);
            let last_type = if buf_len > 0 { &buf.last().unwrap().1 } else { &InitialCharType::None };

            // if type has changed (or it is a Control character) push it to spans and reset buffer
            if (char_type != *last_type || char_type == InitialCharType::Control) && buf_len > 0 {
                push_buf_to_spans(&buf, &mut spans);
                buf = Vec::new();
            }

            buf.push((ch, char_type, char_idx));
        }

        if buf.len() > 0 {
            push_buf_to_spans(&buf, &mut spans);
            buf = Vec::new();
        }

    }

    spans
}

pub fn lexical_analisys(source: &str) -> (Option<Vec<SpannedToken>>, Vec<Simple<char>>) {

    let rope = Rope::from_str(source);
    let spans = get_spanned_strings(rope);

    let mut tokens = Vec::<SpannedToken>::new();

    (Some(tokens), Vec::<Simple<char>>::new())
}


#[cfg(test)]
mod tests {

    use crate::lexer::*;
    use test_case::test_case;

//#region newline
    #[test_case('\n', true; "NEWLINE as a newline character")]
    #[test_case('\r', true; "CARRIAGE RETURN as a newline character")]
    #[test_case('\x0B', true; "VERTICAL TABULATION as a newline character")]
    #[test_case('\x0C', true; "FORM FEED as a newline character")]
    #[test_case('\u{0085}', true; "NEXT LINE as a newline character")]
    #[test_case('\u{2028}', true; "LINE SEPARATOR as a newline character")]
    #[test_case('\u{2029}', true; "PARAGRAPH SEPARATOR as a newline character")]
    #[test_case('%', false ; "PERCENT as not a newline character")]
    #[test_case('&', false ; "AMPERSANT as not a newline character")]
    #[test_case('(', false ; "PAREN_OPEN as not a newline character")]
    #[test_case(')', false ; "PAREN_CLOSE as not a newline character")]
    #[test_case('*', false ; "ASTERISK as not a newline character")]
    #[test_case('+', false ; "PLUS as not a newline character")]
    #[test_case(',', false ; "COMMA as not a newline character")]
    #[test_case('-', false ; "MINUS as not a newline character")]
    #[test_case('.', false ; "DOT as not a newline character")]
    #[test_case('/', false ; "SLASH as not a newline character")]
    #[test_case('0', false ; "0 as not a newline character")]
    #[test_case('1', false ; "1 as not a newline character")]
    #[test_case('2', false ; "2 as not a newline character")]
    #[test_case('3', false ; "3 as not a newline character")]
    #[test_case('4', false ; "4 as not a newline character")]
    #[test_case('5', false ; "5 as not a newline character")]
    #[test_case('6', false ; "6 as not a newline character")]
    #[test_case('7', false ; "7 as not a newline character")]
    #[test_case('8', false ; "8 as not a newline character")]
    #[test_case('9', false ; "9 as not a newline character")]
    #[test_case(':', false ; "COLON as not a newline character")]
    #[test_case(';', false ; "SEMICOLON as not a newline character")]
    #[test_case('<', false ; "LESS THAN as not a newline character")]
    #[test_case('=', false ; "EQUAL as not a newline character")]
    #[test_case('>', false ; "GREATER THAN as not a newline character")]
    #[test_case('?', false ; "QUESTION MARK as not a newline character")]
    #[test_case('@', false ; "AT as not a newline character")]
    #[test_case('A', false ; "A as not a newline character")]
    #[test_case('B', false ; "B as not a newline character")]
    #[test_case('C', false ; "C as not a newline character")]
    #[test_case('D', false ; "D as not a newline character")]
    #[test_case('E', false ; "E as not a newline character")]
    #[test_case('F', false ; "F as not a newline character")]
    #[test_case('G', false ; "G as not a newline character")]
    #[test_case('H', false ; "H as not a newline character")]
    #[test_case('I', false ; "I as not a newline character")]
    #[test_case('J', false ; "J as not a newline character")]
    #[test_case('K', false ; "K as not a newline character")]
    #[test_case('L', false ; "L as not a newline character")]
    #[test_case('M', false ; "M as not a newline character")]
    #[test_case('N', false ; "N as not a newline character")]
    #[test_case('O', false ; "O as not a newline character")]
    #[test_case('P', false ; "P as not a newline character")]
    #[test_case('Q', false ; "Q as not a newline character")]
    #[test_case('R', false ; "R as not a newline character")]
    #[test_case('S', false ; "S as not a newline character")]
    #[test_case('T', false ; "T as not a newline character")]
    #[test_case('U', false ; "U as not a newline character")]
    #[test_case('V', false ; "V as not a newline character")]
    #[test_case('W', false ; "W as not a newline character")]
    #[test_case('X', false ; "X as not a newline character")]
    #[test_case('Y', false ; "Y as not a newline character")]
    #[test_case('Z', false ; "Z as not a newline character")]
    fn is_newline_identifies(c: char, is: bool) {
        assert_eq!(is_newline(c), is);
    }
//#endregion

//#region whitespace
    #[test_case('\n', true, true; "NEWLINE as a whitespace when matching newline")]
    #[test_case('\r', true, true; "CARRIAGE RETURN as a whitespace when matching newline")]
    #[test_case('\x0B', true, true; "VERTICAL TABULATION as a whitespace when matching newline")]
    #[test_case('\x0C', true, true; "FORM FEED as a whitespace when matching newline")]
    #[test_case('\u{0085}', true, true; "NEXT LINE as a whitespace when matching newline")]
    #[test_case('\u{2028}', true, true; "LINE SEPARATOR as a whitespace when matching newline")]
    #[test_case('\u{2029}', true, true; "PARAGRAPH SEPARATOR as a whitespace when matching newline")]
    #[test_case('\n', false, false; "NEWLINE as not a whitespace when not matching newline")]
    #[test_case('\r', false, false; "CARRIAGE RETURN as not a whitespace not when matching newline")]
    #[test_case('\x0B', false, false; "VERTICAL TABULATION as not a whitespace not when matching newline")]
    #[test_case('\x0C', false, false; "FORM FEED as not a whitespace not when matching newline")]
    #[test_case('\u{0085}', false, false; "NEXT LINE as not a whitespace not when matching newline")]
    #[test_case('\u{2028}', false, false; "LINE SEPARATOR as not a whitespace not when matching newline")]
    #[test_case('\u{2029}', false, false; "PARAGRAPH SEPARATOR as not a whitespace not when matching newline")]
    #[test_case(' ', true, false ; "SPACE as whitespace when not matching newline")]
    #[test_case(' ', true, true ; "SPACE as whitespace when matching newline")]
    #[test_case('\t', true, false ; "TAB as whitespace when not matching newline")]
    #[test_case('\t', true, true ; "TAB as whitespace when matching newline")]
    #[test_case('\u{0009}', true, true ; "CHARACTER TABULATION as whitespace when matching newline")]
    #[test_case('\u{0009}', true, false ; "CHARACTER TABULATION as whitespace when not matching newline")]
    #[test_case('\u{00A0}', true, true ; "NO-BREAK SPACE as whitespace when matching newline")]
    #[test_case('\u{00A0}', true, false ; "NO-BREAK SPACE as whitespace when not matching newline")]
    #[test_case('\u{1680}', true, true ; "OGHAM SPACE MARK as whitespace when matching newline")]
    #[test_case('\u{1680}', true, false ; "OGHAM SPACE MARK as whitespace when not matching newline")]
    #[test_case('\u{2000}', true, true ; "EN QUAD as whitespace when matching newline")]
    #[test_case('\u{2000}', true, false ; "EN QUAD as whitespace when not matching newline")]
    #[test_case('\u{2001}', true, true ; "EM QUAD as whitespace when matching newline")]
    #[test_case('\u{2001}', true, false ; "EM QUAD as whitespace when not matching newline")]
    #[test_case('\u{2002}', true, true ; "EN SPACE as whitespace when matching newline")]
    #[test_case('\u{2002}', true, false ; "EN SPACE as whitespace when not matching newline")]
    #[test_case('\u{2003}', true, true ; "EM SPACE as whitespace when matching newline")]
    #[test_case('\u{2003}', true, false ; "EM SPACE as whitespace when not matching newline")]
    #[test_case('\u{2004}', true, true ; "THREE-PER-EM SPACE as whitespace when matching newline")]
    #[test_case('\u{2004}', true, false ; "THREE-PER-EM SPACE as whitespace when not matching newline")]
    #[test_case('\u{2005}', true, true ; "FOUR-PER-EM SPACE as whitespace when matching newline")]
    #[test_case('\u{2005}', true, false ; "FOUR-PER-EM SPACE as whitespace when not matching newline")]
    #[test_case('\u{2006}', true, true ; "SIX-PER-EM SPACE as whitespace when matching newline")]
    #[test_case('\u{2006}', true, false ; "SIX-PER-EM SPACE as whitespace when not matching newline")]
    #[test_case('\u{2007}', true, true ; "FIGURE SPACE as whitespace when matching newline")]
    #[test_case('\u{2007}', true, false ; "FIGURE SPACE as whitespace when not matching newline")]
    #[test_case('\u{2008}', true, true ; "PUNCTUATION SPACE as whitespace when matching newline")]
    #[test_case('\u{2008}', true, false ; "PUNCTUATION SPACE as whitespace when not matching newline")]
    #[test_case('\u{2009}', true, true ; "THIN SPACE as whitespace when matching newline")]
    #[test_case('\u{2009}', true, false ; "THIN SPACE as whitespace when not matching newline")]
    #[test_case('\u{200A}', true, true ; "HAIR SPACE as whitespace when matching newline")]
    #[test_case('\u{200A}', true, false ; "HAIR SPACE as whitespace when not matching newline")]
    #[test_case('\u{202F}', true, true ; "NARROW NO-BREAK SPACE as whitespace when matching newline")]
    #[test_case('\u{202F}', true, false ; "NARROW NO-BREAK SPACE as whitespace when not matching newline")]
    #[test_case('\u{205F}', true, true ; "MEDIUM MATHEMATICAL SPACE as whitespace when matching newline")]
    #[test_case('\u{205F}', true, false ; "MEDIUM MATHEMATICAL SPACE as whitespace when not matching newline")]
    #[test_case('\u{3000}', true, true ; "IDEOGRAPHIC SPACE as whitespace when matching newline")]
    #[test_case('\u{3000}', true, false ; "IDEOGRAPHIC SPACE as whitespace when not matching newline")]
    #[test_case('\u{180E}', false, true ; "MONGOLIAN VOWEL SEPARATOR as not whitespace when matching newline")]
    #[test_case('\u{180E}', false, false ; "MONGOLIAN VOWEL SEPARATOR as not whitespace when not matching newline")]
    #[test_case('\u{200B}', false, true ; "ZERO WIDTH SPACE as not whitespace when matching newline")]
    #[test_case('\u{200B}', false, false ; "ZERO WIDTH SPACE as not whitespace when not matching newline")]
    #[test_case('\u{200C}', false, true ; "ZERO WIDTH NON-JOINER as not whitespace when matching newline")]
    #[test_case('\u{200C}', false, false ; "ZERO WIDTH NON-JOINER as not whitespace when not matching newline")]
    #[test_case('\u{200D}', false, true ; "ZERO WIDTH JOINER as not whitespace when matching newline")]
    #[test_case('\u{200D}', false, false ; "ZERO WIDTH JOINER as not whitespace when not matching newline")]
    #[test_case('\u{2060}', false, true ; "WORD JOINER as not whitespace when matching newline")]
    #[test_case('\u{2060}', false, false ; "WORD JOINER as not whitespace when not matching newline")]
    #[test_case('\u{FEFF}', false, true ; "ZERO WIDTH NON-BREAKING SPACE as not whitespace when matching newline")]
    #[test_case('\u{FEFF}', false, false ; "ZERO WIDTH NON-BREAKING SPACE as not whitespace when not matching newline")]
    fn is_whitespace_identifies(c: char, whitespace: bool, match_newline: bool) {
        assert_eq!(is_whitespace(c, match_newline) , whitespace)
    }

//#endregion

//#region get_initial_char_type

//';' | ':' => true,
// '"' | '\'' => true,
// '(' | ')' => true,
// ',' => true,
// '#' => true,
// '@' => true,
    #[test_case('\n', InitialCharType::NewLine; "NEWLINE is InitialCharType::NewLine")]
    #[test_case('\r', InitialCharType::NewLine; "CARRIAGE RETURN is InitialCharType::NewLine")]
    #[test_case('\x0B', InitialCharType::NewLine; "VERTICAL TABULATION is InitialCharType::NewLine")]
    #[test_case('\x0C', InitialCharType::NewLine; "FORM FEED is InitialCharType::NewLine")]
    #[test_case('\u{0085}', InitialCharType::NewLine; "NEXT LINE is InitialCharType::NewLine")]
    #[test_case('\u{2028}', InitialCharType::NewLine; "LINE SEPARATOR is InitialCharType::NewLine")]
    #[test_case('\u{2029}', InitialCharType::NewLine; "PARAGRAPH SEPARATOR is InitialCharType::NewLine")]
    
    #[test_case('(', InitialCharType::Control ; "PAREN_OPEN is InitialCharType::Control")]
    #[test_case(')', InitialCharType::Control ; "PAREN_CLOSE is InitialCharType::Control")]
    #[test_case(';', InitialCharType::Control ; "SEMICOLON is InitialCharType::Control")]
    #[test_case(':', InitialCharType::Control ; "COLON is InitialCharType::Control")]
    #[test_case('"', InitialCharType::Control ; "QUOTATION MARK is InitialCharType::Control")]
    #[test_case('\'', InitialCharType::Control ; "APOSTROPHE is InitialCharType::Control")]
    #[test_case(',', InitialCharType::Control ; "COMMA is InitialCharType::Control")]
    #[test_case('#', InitialCharType::Control ; "HASH is InitialCharType::Control")]
    #[test_case('@', InitialCharType::Control ; "AT is InitialCharType::Control")]

    #[test_case(' ', InitialCharType::WhiteSpace ; "SPACE is InitialCharType::WhiteSpace")]
    #[test_case('\t', InitialCharType::WhiteSpace ; "HORIZONTAL TABULATION is InitialCharType::WhiteSpace")]

    #[test_case('%', InitialCharType::Other ; "PERCENT is InitialCharType::Other")]
    #[test_case('&', InitialCharType::Other ; "AMPERSANT is InitialCharType::Other")]
    fn initial_char_type_for(ch: char, t: InitialCharType) {
        assert_eq!(get_initial_char_type(ch), t);
    }
//#endregion

//#region spanned strings

fn spanned_string(s: &str, range: Range<usize>, line: usize, columns: Range<usize>) -> SpannedString {
    SpannedString::from_str(s, TextPosition::new(range, line, columns))
}

    #[test_case(
        "abc cde   dcr xyz",
        vec![ 
            SpannedString::from_str("abc", TextPosition::new(0..2, 0, 0..2)),
            SpannedString::from_str(" ",   TextPosition::new(3..3, 0, 3..3)),
            SpannedString::from_str("cde", TextPosition::new(4..6, 0, 4..6)),
            SpannedString::from_str("   ", TextPosition::new(7..9, 0, 7..9)),
            SpannedString::from_str("dcr", TextPosition::new(10..12, 0, 10..12)),
            SpannedString::from_str(" ",   TextPosition::new(13..13, 0, 13..13)),
            SpannedString::from_str("xyz", TextPosition::new(14..16, 0, 14..16)),
        ]
        ; "simple one line source" )]
    #[test_case(
        "abc cde\n dcr xyz",
        vec![ 
            SpannedString::from_str("abc", TextPosition::new(0..2, 0, 0..2)),
            SpannedString::from_str(" ",   TextPosition::new(3..3, 0, 3..3)),
            SpannedString::from_str("cde", TextPosition::new(4..6, 0, 4..6)),
            SpannedString::from_str("\n",  TextPosition::new(7..7, 0, 7..7)),

            SpannedString::from_str(" ",   TextPosition::new(8..8, 1, 0..0)),
            SpannedString::from_str("dcr", TextPosition::new(9..11, 1, 1..3)),
            SpannedString::from_str(" ",   TextPosition::new(12..12, 1, 4..4)),
            SpannedString::from_str("xyz", TextPosition::new(13..15, 1, 5..7)),
        ]
        ; "simple two line source" )]
    #[test_case(
        "abc cde ;xyz",
        vec![
            SpannedString::from_str("abc", TextPosition::new(0..2, 0, 0..2)),
            SpannedString::from_str(" ",   TextPosition::new(3..3, 0, 3..3)),
            SpannedString::from_str("cde", TextPosition::new(4..6, 0, 4..6)),
            SpannedString::from_str(" ",   TextPosition::new(7..7, 0, 7..7)),
            SpannedString::from_str(";",   TextPosition::new(8..8, 0, 8..8)),
            SpannedString::from_str("xyz", TextPosition::new(9..11, 0, 9..11)),
        ]
        ; "simple one line source with a comment" )]
    #[test_case(
        "abc ;cde\n dcr ; xyz",
        vec![ 
            SpannedString::from_str("abc", TextPosition::new(0..2, 0, 0..2)),
            SpannedString::from_str(" ",   TextPosition::new(3..3, 0, 3..3)),
            SpannedString::from_str(";",   TextPosition::new(4..4, 0, 4..4)),
            SpannedString::from_str("cde", TextPosition::new(5..7, 0, 5..7)),
            SpannedString::from_str("\n",  TextPosition::new(8..8, 0, 8..8)),
            
            SpannedString::from_str(" ",   TextPosition::new(9..9, 1, 0..0)),
            SpannedString::from_str("dcr", TextPosition::new(10..12, 1, 1..3)),
            SpannedString::from_str(" ",   TextPosition::new(13..13, 1, 4..4)),
            SpannedString::from_str(";",   TextPosition::new(14..14, 1, 5..5)),
            SpannedString::from_str(" ",   TextPosition::new(15..15, 1, 6..6)),
            SpannedString::from_str("xyz", TextPosition::new(16..18, 1, 7..9)),
        ]
        ; "simple two line source with comments" )]
    #[test_case(
        "    LJMP       START             \n    ORG       100H\nSTART:\n    MOV    R0,#30H             ;address",
        vec![
            //line 0
            spanned_string("    ", 0..3, 0, 0..3),
            spanned_string("LJMP", 4..7, 0, 4..7),
            spanned_string("       ", 8..14, 0, 8..14),
            spanned_string("START", 15..19, 0, 15..19),
            spanned_string("             ", 20..32, 0, 20..32),
            spanned_string("\n", 33..33, 0, 33..33),

            //line 1
            spanned_string("    ", 34..37, 1, 0..3),
            spanned_string("ORG", 38..40, 1, 4..6),
            spanned_string("       ", 41..47, 1, 7..13),
            spanned_string("100H", 48..51, 1, 14..17),
            spanned_string("\n", 52..52, 1, 18..18),

            //line 2
            spanned_string("START", 53..57, 2, 0..4),
            spanned_string(":", 58..58, 2, 5..5),
            spanned_string("\n", 59..59, 2, 6..6),

            //line 3
            spanned_string("    ", 60..63, 3, 0..3),
            spanned_string("MOV", 64..66, 3, 4..6),
            spanned_string("    ", 67..70, 3, 7..10),
            spanned_string("R0", 71..72, 3, 11..12),
            spanned_string(",", 73..73, 3, 13..13),
            spanned_string("#", 74..74, 3, 14..14),
            spanned_string("30H", 75..77, 3, 15..17),
            spanned_string("             ", 78..90, 3, 18..30),
            spanned_string(";", 91..91, 3, 31..31),
            spanned_string("address", 92..98, 3, 32..38),
        ]
        ; "some more complex code")]
    fn get_spanned_strings_for(source: &str, expected_strings: Vec<SpannedString>) {
        
        let rope = Rope::from_str(source);
        let actual_strings = get_spanned_strings(rope);
        for i in 0..(actual_strings.len()) {
            let expected = &expected_strings[i];
            let actual = &actual_strings[i];

            assert_eq!(expected.string, actual.string);
            assert_eq!(expected.position, actual.position);
        }
    }
//#endregion

}