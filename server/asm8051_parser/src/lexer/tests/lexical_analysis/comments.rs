use crate::{lexer::*, Tkn};
use test_case::test_case;
use tests::spanned_strings::pos;



#[test_case(
    ";some comment",
    vec![
        PositionedToken::new(Tkn![Semicolon], pos(0..0, 0, 0..0)),
        PositionedToken::new(Tkn![CommentStr("some comment")], pos(1..12, 0, 1..12)),
    ] 
; "is a string of characters starting with a semicolon, endinf with an end of the line of code" )]
#[test_case(
    ";some 😀😎",
    vec![
        PositionedToken::new(Tkn![Semicolon], pos(0..0, 0, 0..0)),
        PositionedToken::new(Tkn![CommentStr("some 😀😎")], pos(1..7, 0, 1..7)),
    ] 
; "can even contain some unicode characters" )]
fn a_valid_comment(src: &str, expected_ast: Vec<PositionedToken>) {
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


#[test_case::test_case(
    "    ADD A, @R1 ;some 😀😎",
    vec![
        PositionedToken::new(Tkn![WhiteSpace("    ")], pos(0..3, 0, 0..3)),
        PositionedToken::new(Tkn![ADD], pos(4..6, 0, 4..6)),
        PositionedToken::new(Tkn![WhiteSpace(" ")], pos(7..7, 0, 7..7)),
        PositionedToken::new(Tkn![A], pos(8..8, 0, 8..8)),
        PositionedToken::new(Tkn![ArgumentSeparator], pos(9..9, 0, 9..9)),
        PositionedToken::new(Tkn![WhiteSpace(" ")], pos(10..10, 0, 10..10)),
        PositionedToken::new(Tkn![AddressingModifier], pos(11..11, 0, 11..11)),
        PositionedToken::new(Tkn![R1], pos(12..13, 0, 12..13)),
        PositionedToken::new(Tkn![WhiteSpace(" ")], pos(14..14, 0, 14..14)),

        PositionedToken::new(Tkn![Semicolon], pos(15..15, 0, 15..15)),
        PositionedToken::new(Tkn![CommentStr("some 😀😎")], pos(16..22, 0, 16..22)),
    ] 
; "can it parse a simple line of code with some comment at the end?" )]
fn t1(src: &str, expected_ast: Vec<PositionedToken>) {
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