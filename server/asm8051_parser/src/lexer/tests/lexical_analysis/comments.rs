use crate::{lexer::*, token, position};
use test_case::test_case;

#[test_case(
    ";some comment",
    vec![
        PositionedToken::new(token![Semicolon], position!(0, 0)),
        PositionedToken::new(token![Comment("some comment")], position!(1..12, 0)),
    ] 
; "is a string of characters starting with a semicolon, ending with an end of the line of code" )]
#[test_case(
    ";some 😀😎",
    vec![
        PositionedToken::new(token![Semicolon], position!(0, 0)),
        PositionedToken::new(token![Comment("some 😀😎")], position!(1..7, 0)),
    ] 
; "can even contain some unicode characters" )]
fn a_valid_comment(src: &str, expected_ast: Vec<PositionedToken>) {
    let (ast, errors) = lexical_analysis(src);
    
    if errors.len() > 0 {
        println!("Errors occurred!");
        for err in &errors[0..] {
            println!("{err}");
        }
    }
    assert!(errors.len() == 0, "errors occurred!");

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
        PositionedToken::new(token![WhiteSpace("    ")], position!(0..3, 0)),
        PositionedToken::new(token![ADD], position!(4..6, 0, 4..6)),
        PositionedToken::new(token![WhiteSpace(" ")], position!(7, 0)),
        PositionedToken::new(token![A], position!(8, 0)),
        PositionedToken::new(token![ArgumentSeparator], position!(9, 0)),
        PositionedToken::new(token![WhiteSpace(" ")], position!(10, 0)),
        PositionedToken::new(token![AddressingModifier], position!(11, 0)),
        PositionedToken::new(token![R1], position!(12..13, 0)),
        PositionedToken::new(token![WhiteSpace(" ")], position!(14, 0)),

        PositionedToken::new(token![Semicolon], position!(15, 0)),
        PositionedToken::new(token![Comment("some 😀😎")], position!(16..22, 0)),
    ] 
; "can it parse a simple line of code with some comment at the end?" )]
fn t1(src: &str, expected_ast: Vec<PositionedToken>) {
    let (ast, errors) = lexical_analysis(src);
    
    if errors.len() > 0 {
        println!("Errors occurred!");
        for err in &errors[0..] {
            println!("{err}");
        }
    }
    assert!(errors.len() == 0, "errors occurred!");

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