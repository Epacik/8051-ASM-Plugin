
use crate::{lexer::*, token, position, issues};
use test_case::test_case;

#[test_case(
    "'some content'",
    vec![
        PositionedToken::new(token![Apostrophe], position!(0, 0)),
        PositionedToken::new(token![String("some content")], position!(1..12, 0)),
        PositionedToken::new(token![Apostrophe], position!(13, 0)),
    ] 
; "is a string with some ascii characters inside with apostrophes" )]
#[test_case(
    "\"some content\"",
    vec![
        PositionedToken::new(token![DoubleQuote], position!(0, 0)),
        PositionedToken::new(token![String("some content")], position!(1..12, 0)),
        PositionedToken::new(token![DoubleQuote], position!(13, 0)),
    ] 
; "is a string with some ascii characters inside with double quotes" )]
#[test_case(
    include_str!("test.asm"),
    vec![
        PositionedToken::new(token![DoubleQuote], position!(0, 0)),
        PositionedToken::new(token![String("some content\"")], position!(1..14, 0)),
        PositionedToken::new(token![DoubleQuote], position!(15, 0)),
    ] 
; "is a string with some ascii characters inside with double quotes with escaped char" )]
#[test_case(
    r#""some content\x32""#,
    vec![
        PositionedToken::new(token![DoubleQuote], position!(0, 0)),
        PositionedToken::new(token![String("some content2")], position!(1..16, 0)),
        PositionedToken::new(token![DoubleQuote], position!(17, 0)),
    ] 
; "is a string with some ascii characters inside with double quotes with escaped hex" )]
fn a_valid_string(src: &str, expected_ast: Vec<PositionedToken>) {
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


#[test_case(
    "'some content",
    vec![
        PositionedToken::new(token![Apostrophe], position!(0, 0)),
        PositionedToken::new(token![String("some content")], position!(1..12, 0)),
    ],
    vec![
        issues::unclosed_string(position!(0..12, 0), token![Apostrophe], Some(token![Other("t")]))
    ]
; "is a string with some ascii characters without a closing apostrophe" )]
fn an_invalid_string(src: &str, expected_ast: Vec<PositionedToken>, expected_issues: Vec<Issue>) {
    let (ast, actual_issues) = lexical_analysis(src);
    assert!(ast.is_some(), "ast was None");

    let actual_ast = ast.unwrap();

    let _actual_ast_strings = tokens::helpers::positioned_tokens_to_strings(&actual_ast);
    let _expected_ast_strings = tokens::helpers::positioned_tokens_to_strings(&expected_ast);

    // let's match up tokens
    assert_eq!(actual_ast.len(), expected_ast.len(), "unexpected number of tokens");

    for i in 0..(actual_ast.len()) {
        let actual_token = &actual_ast[i];
        let expected_token = &expected_ast[i];

        assert_eq!(actual_token.token, expected_token.token, 
            "tokens do not match (index {}), {} != {}", i, actual_token.token, expected_token.token);
        assert_eq!(actual_token.position, expected_token.position, 
            "positions do not match (index {}), {} != {}", i, actual_token.position, expected_token.position)
    }

    assert_eq!(actual_issues.len(), expected_issues.len(), "unexpected number of issues");

    for i in 0..(actual_issues.len()) {
        let actual_issue = &actual_issues[i];
        let expected_issue = &expected_issues[i];

        assert_eq!(actual_issue.info(), expected_issue.info(), 
            "issue info do not match [{}] != [{}]", actual_issue.info(), expected_issue.info());

        assert_eq!(actual_issue.position(), expected_issue.position(), 
            "position do not match [{}] != [{}]", actual_issue.position(), expected_issue.position());

        assert_eq!(actual_issue.expected(), expected_issue.expected(),
            "expected token do not match [{:#?}] != [{:#?}]", actual_issue.expected(), expected_issue.expected());

        assert_eq!(actual_issue.found(), expected_issue.found(),
            "found token do not match [{:#?}] != [{:#?}]", actual_issue.found(), expected_issue.found());

        let actual_inv_chars = actual_issue.invalid_characters();
        let expect_inv_chars = expected_issue.invalid_characters();

        assert_eq!(actual_inv_chars.len(), expect_inv_chars.len(),
            "not matching invalid characters length [{}] != [{}]", actual_inv_chars.len(), expect_inv_chars.len());

        for x in 0..(actual_inv_chars.len()) {
            let act = &actual_inv_chars[x];
            let exp = &expect_inv_chars[x];
            assert_eq!(act, exp,
                "found invalid char do not match [{:#?}] != [{:#?}]", act, exp);
        }
    }

}