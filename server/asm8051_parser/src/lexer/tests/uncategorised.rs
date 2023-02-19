use crate::lexer::*;
//use test_case::test_case;



#[test]
fn analize()
{
    let (_ast, _errors) = 
        lexical_analisys(
r#"    LJMP #1
    LJMP  START
STR EQU 'some small text'

    ORG  100H ; we're changing the address of subsequent code
START:
    MOV A, #0
    MOV R1, #1
LOOP:
    ADD A, P1.1
    ADDC A, R2
    SUBB A, @R3
    CALL WRITE_HEX
    SJMP LOOP
    DEC R7
    RET ;🤨🤨🤨a̐éö̲\r\n"#);

    let unwrapped = _ast.unwrap();
    
    //to help with debugging
    let _strings = tokens::helpers::positioned_tokens_to_strings(&unwrapped);
    
    for tkn in unwrapped {
        println!("{}", tkn);
    }
    
    // assert_eq!(errors.len(), 0, "There were errors");
    // assert!(ast.is_some(), "there's no AST");

    // let ast = ast.unwrap();

    // assert!(ast.len() > 0);
}


