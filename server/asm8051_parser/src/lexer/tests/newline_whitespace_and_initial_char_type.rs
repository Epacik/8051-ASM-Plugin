use test_case::test_case;

use crate::lexer::{
    initial::{InitialTokenType, self}
};

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
    assert_eq!(initial::is_newline(c), is);
}

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
    assert_eq!(initial::is_whitespace(c, match_newline) , whitespace)
}


#[test_case('\n', InitialTokenType::NewLine; "NEWLINE is InitialCharType::NewLine")]
#[test_case('\r', InitialTokenType::NewLine; "CARRIAGE RETURN is InitialCharType::NewLine")]
#[test_case('\x0B', InitialTokenType::NewLine; "VERTICAL TABULATION is InitialCharType::NewLine")]
#[test_case('\x0C', InitialTokenType::NewLine; "FORM FEED is InitialCharType::NewLine")]
#[test_case('\u{0085}', InitialTokenType::NewLine; "NEXT LINE is InitialCharType::NewLine")]
#[test_case('\u{2028}', InitialTokenType::NewLine; "LINE SEPARATOR is InitialCharType::NewLine")]
#[test_case('\u{2029}', InitialTokenType::NewLine; "PARAGRAPH SEPARATOR is InitialCharType::NewLine")]

#[test_case('(', InitialTokenType::Control ; "PAREN_OPEN is InitialCharType::Control")]
#[test_case(')', InitialTokenType::Control ; "PAREN_CLOSE is InitialCharType::Control")]
#[test_case(';', InitialTokenType::Control ; "SEMICOLON is InitialCharType::Control")]
#[test_case(':', InitialTokenType::Control ; "COLON is InitialCharType::Control")]
#[test_case('"', InitialTokenType::Control ; "QUOTATION MARK is InitialCharType::Control")]
#[test_case('\'', InitialTokenType::Control ; "APOSTROPHE is InitialCharType::Control")]
#[test_case(',', InitialTokenType::Control ; "COMMA is InitialCharType::Control")]
#[test_case('#', InitialTokenType::Control ; "HASH is InitialCharType::Control")]
#[test_case('@', InitialTokenType::Control ; "AT is InitialCharType::Control")]
#[test_case('+', InitialTokenType::Control ; "PLUS is InitialCharType::Control")]
#[test_case('-', InitialTokenType::Control ; "MINUS is InitialCharType::Control")]
#[test_case('*', InitialTokenType::Control ; "MULTIPLY is InitialCharType::Control")]
#[test_case('/', InitialTokenType::Control ; "DIVIDE is InitialCharType::Control")]
#[test_case('%', InitialTokenType::Control ; "MODULO is InitialCharType::Control")]

#[test_case(' ', InitialTokenType::WhiteSpace ; "SPACE is InitialCharType::WhiteSpace")]
#[test_case('\t', InitialTokenType::WhiteSpace ; "HORIZONTAL TABULATION is InitialCharType::WhiteSpace")]

#[test_case('&', InitialTokenType::Other ; "AMPERSANT is InitialCharType::Other")]
#[test_case('a', InitialTokenType::Other ; "A is InitialCharType::Other")]
#[test_case('V', InitialTokenType::Other ; "V is InitialCharType::Other")]
#[test_case('4', InitialTokenType::Other ; "4 is InitialCharType::Other")]
#[test_case('Z', InitialTokenType::Other ; "Z is InitialCharType::Other")]
fn initial_char_type_for(ch: char, t: InitialTokenType) {
    assert_eq!(initial::get_initial_char_type(ch), t);
}