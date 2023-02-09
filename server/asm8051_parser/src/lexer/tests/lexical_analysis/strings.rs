
use crate::{lexer::*, Tkn};
use test_case::test_case;
use tests::spanned_strings::pos;

// #[test_case(
//     "'some content'",
//     vec![
//         PositionedToken::new(Tkn![Apostrophe], pos(0..0, 0, 0..0)),
//         PositionedToken::new(Tkn![str("some content")], pos(1..12, 0, 1..12)),
//         PositionedToken::new(Tkn![Apostrophe], pos(13..13, 0, 13..13)),
//     ] 
// ; "is a string with some ascii characters inside with apostrophes" )]
// #[test_case(
//     "\"some content\"",
//     vec![
//         PositionedToken::new(Tkn![DoubleQuote], pos(0..0, 0, 0..0)),
//         PositionedToken::new(Tkn![str("some content")], pos(1..12, 0, 1..12)),
//         PositionedToken::new(Tkn![DoubleQuote], pos(13..13, 0, 13..13)),
//     ] 
// ; "is a string with some ascii characters inside with double quotes" )]
#[test_case(
    include_str!("test.asm"),
    vec![
        PositionedToken::new(Tkn![DoubleQuote], pos(0..0, 0, 0..0)),
        PositionedToken::new(Tkn![str("some content\"")], pos(1..14, 0, 1..14)),
        PositionedToken::new(Tkn![DoubleQuote], pos(15..15, 0, 15..15)),
    ] 
; "is a string with some ascii characters inside with double quotes with escaped char" )]
#[test_case(
    r#""some content\x32""#,
    vec![
        PositionedToken::new(Tkn![DoubleQuote], pos(0..0, 0, 0..0)),
        PositionedToken::new(Tkn![str("some content2")], pos(1..16, 0, 1..16)),
        PositionedToken::new(Tkn![DoubleQuote], pos(17..17, 0, 17..17)),
    ] 
; "is a string with some ascii characters inside with double quotes with escaped hex" )]
fn a_valid_string(src: &str, expected_ast: Vec<PositionedToken>) {
    let (ast, errors) = lexical_analisys(src);
    
    if errors.len() > 0 {
        println!("Errors occured!");
        for err in &errors[0..] {
            println!("{err}");
        }
    }
    assert!(errors.len() == 0, "errors occured!");

    assert!(ast.is_some(), "ast was None");

    let actual_ast = ast.unwrap();
    
    let actual_len = actual_ast.len();
    let expected_len = expected_ast.len();
    assert_eq!(actual_len, expected_len, "lengths of ast's are not equal, actual: {actual_len}, expected: {expected_len}");

    for i in 0..actual_len {
        let actual_ptoken = &actual_ast[i];
        let expected_ptoken = &expected_ast[i];

        let actual_token = actual_ptoken.token.clone();
        let expected_token = expected_ptoken.token.clone();

        assert_eq!(actual_token, expected_token, "tokens are different! actual: {actual_token:#?}, expected: {expected_token:#?}");

        let actual_position = actual_ptoken.position.clone();
        let expected_position = expected_ptoken.position.clone();
        assert_eq!(actual_position, expected_position, "positions are different! token {actual_token:#?}, actual position: {actual_position:#?}, expected: {expected_position:#?}");
    }
}