use crate::{lexer::*, token, position};
use test_case::test_case;


#[test_case(
r#"    LJMP  START
    ORG  100H
START:
    CLR  P1.7
; infinite loop that does nothing
STOP:
    SJMP  STOP"#,
    vec![
        // line 0
        PositionedToken::new(token![WhiteSpace("    ")],   position!(0..3, 0)),
        PositionedToken::new(token![LJMP],                 position!(4..7, 0)),
        PositionedToken::new(token![WhiteSpace("  ")],     position!(8..9, 0)),
        PositionedToken::new(token![Other("START")],       position!(10..14, 0)),
        PositionedToken::new(token![NewLine("\n")],        position!(15, 0)),
        
        // line 1
        PositionedToken::new(token![WhiteSpace("    ")],   position!(16..19, 1, 0..3)),
        PositionedToken::new(token![ORG],                  position!(20..22, 1, 4..6)),
        PositionedToken::new(token![WhiteSpace("  ")],     position!(23..24, 1, 7..8)),
        PositionedToken::new(token![AddressH("100")],      position!(25..28, 1, 9..12)),
        PositionedToken::new(token![NewLine("\n")],        position!(29, 1, 13)),
        
        // line 2
        PositionedToken::new(token![Label("START")],       position!(30..34, 2, 0..4)),
        PositionedToken::new(token![LabelEnd],             position!(35, 2, 5)),
        PositionedToken::new(token![NewLine("\n")],        position!(36, 2, 6)),
        
        // line 3
        PositionedToken::new(token![WhiteSpace("    ")],   position!(37..40, 3, 0..3)),
        PositionedToken::new(token![CLR],                  position!(41..43, 3, 4..6)),
        PositionedToken::new(token![WhiteSpace("  ")],     position!(44..45, 3, 7..8)),
        PositionedToken::new(token![P1],                   position!(46..47, 3, 9..10)),
        PositionedToken::new(token![AddressingSeparator],  position!(48, 3, 11)),
        PositionedToken::new(token![AddressD("7")],        position!(49, 3, 12)),
        PositionedToken::new(token![NewLine("\n")],        position!(50, 3, 13)),
        
        // line 4
        PositionedToken::new(token![CommentStart],         position!(51, 4, 0)),
        PositionedToken::new(token![Comment(" infinite loop that does nothing")], position!(52..83, 4, 1..32)),
        PositionedToken::new(token![NewLine("\n")],        position!(84, 4, 33)),

        // line 5
        PositionedToken::new(token![Label("STOP")],        position!(85..88, 5, 0..3)),
        PositionedToken::new(token![LabelEnd],             position!(89, 5, 4)),
        PositionedToken::new(token![NewLine("\n")],        position!(90, 5, 5)),

        // line 6
        PositionedToken::new(token![WhiteSpace("    ")],   position!(91..94, 6, 0..3)),
        PositionedToken::new(token![SJMP],                 position!(95..98, 6, 4..7)),
        PositionedToken::new(token![WhiteSpace("  ")],     position!(99..100, 6, 8..9)),
        PositionedToken::new(token![Other("STOP")],        position!(101..104, 6, 10..13)),
    ]
; "t1")]
#[test_case(
    r#"TAB	EQU	30H
	LJMP  START
	ORG  100H
START:
	MOV  TAB,#2
	MOV  TAB+1,#3
	MOV  TAB+2,#5
    MOV  TAB+3,#7
 	MOV  R0,#TAB
 	CALL  LCD_CLR
ABCD:
 	MOV  A,@R0
 	CALL  WRITE_HEX
 	MOV  A,#5
 	CALL  DELAY_100MS
 	CALL  LCD_CLR
 	INC  R0
 	CJNE  R0,#TAB+3,ABCD
	MOV  R0,#TAB
 	SJMP  ABCD
"#,
vec![
    // line 0
    PositionedToken::new(token![Label("TAB")],      position!(0..2, 0)),
    PositionedToken::new(token![WhiteSpace("\t")],  position!(3, 0)),
    PositionedToken::new(token![EQU],               position!(4..6, 0)),
    PositionedToken::new(token![WhiteSpace("\t")],  position!(7, 0)),
    PositionedToken::new(token![AddressH("30")],    position!(8..10, 0)),
    PositionedToken::new(token![NewLine("\n")],     position!(11, 0)),
     
    // line 1 
    PositionedToken::new(token![WhiteSpace("\t")],  position!(12, 1, 0)),
    PositionedToken::new(token![LJMP],              position!(13..16, 1, 1..4)),
    PositionedToken::new(token![WhiteSpace("  ")],  position!(17..18, 1, 5..6)),
    PositionedToken::new(token![Other("START")],    position!(19..23, 1, 7..11)),
    PositionedToken::new(token![NewLine("\n")],     position!(24, 1, 12)),
     
    // line 2 
    PositionedToken::new(token![WhiteSpace("\t")],  position!(25, 2, 0)),
    PositionedToken::new(token![ORG],               position!(26..28, 2, 1..3)),
    PositionedToken::new(token![WhiteSpace("  ")],  position!(29..30, 2, 4..5)),
    PositionedToken::new(token![AddressH("100")],   position!(31..34, 2, 6..9)),
    PositionedToken::new(token![NewLine("\n")],     position!(35, 2, 10)),
     
    // line 3 
    PositionedToken::new(token![Label("START")],    position!(36..40, 3, 0..4)),
    PositionedToken::new(token![LabelEnd],          position!(41, 3, 5)),
    PositionedToken::new(token![NewLine("\n")],     position!(42, 3, 6)),
    
    // line 4 
    PositionedToken::new(token![WhiteSpace("\t")],  position!(43, 4, 0)),
    PositionedToken::new(token![MOV],               position!(44..46, 4, 1..3)),
    PositionedToken::new(token![WhiteSpace("  ")],  position!(47..48, 4, 4..5)),
    PositionedToken::new(token![Other("TAB")],      position!(49..51, 4, 6..8)),
    PositionedToken::new(token![ArgumentSeparator], position!(52, 4, 9)),
    PositionedToken::new(token![NumberD("2")],      position!(53..54, 4, 10..11)),
    PositionedToken::new(token![NewLine("\n")],     position!(55, 4, 12)),
    
    // line 5
    PositionedToken::new(token![WhiteSpace("\t")],  position!(56, 5, 0)),
    PositionedToken::new(token![MOV],               position!(57..59, 5, 1..3)),
    PositionedToken::new(token![WhiteSpace("  ")],  position!(60..61, 5, 4..5)),
    PositionedToken::new(token![Other("TAB")],      position!(62..64, 5, 6..8)),
    PositionedToken::new(token![+],                 position!(65, 5, 9)),
    PositionedToken::new(token![AddressD("1")],     position!(66, 5, 10)),
    PositionedToken::new(token![ArgumentSeparator], position!(67, 5, 11)),
    PositionedToken::new(token![NumberD("3")],      position!(68..69, 5, 12..13)),
    PositionedToken::new(token![NewLine("\n")],     position!(70, 5, 14)),

    // line 6
    PositionedToken::new(token![WhiteSpace("\t")],  position!(71, 6, 0)),
    PositionedToken::new(token![MOV],               position!(72..74, 6, 1..3)),
    PositionedToken::new(token![WhiteSpace("  ")],  position!(75..76, 6, 4..5)),
    PositionedToken::new(token![Other("TAB")],      position!(77..79, 6, 6..8)),
    PositionedToken::new(token![+],                 position!(80, 6, 9)),
    PositionedToken::new(token![AddressD("2")],     position!(81, 6, 10)),
    PositionedToken::new(token![ArgumentSeparator], position!(82, 6, 11)),
    PositionedToken::new(token![NumberD("5")],      position!(83..84, 6, 12..13)),
    PositionedToken::new(token![NewLine("\n")],     position!(85, 6, 14)),

    // line 7 
    PositionedToken::new(token![WhiteSpace("    ")], position!(86..89, 7, 0..3)),
    PositionedToken::new(token![MOV],                position!(90..92, 7, 4..6)),
    PositionedToken::new(token![WhiteSpace("  ")],   position!(93..94, 7, 7..8)),
    PositionedToken::new(token![Other("TAB")],       position!(95..97, 7, 9..11)),
    PositionedToken::new(token![+],                  position!(98, 7, 12)),
    PositionedToken::new(token![AddressD("3")],      position!(99, 7, 13)),
    PositionedToken::new(token![ArgumentSeparator],  position!(100, 7, 14)),
    PositionedToken::new(token![NumberD("7")],       position!(101..102, 7, 15..16)),
    PositionedToken::new(token![NewLine("\n")],      position!(103, 7, 17)),
    
    // line 8
    PositionedToken::new(token![WhiteSpace(" \t")],  position!(104..105, 8, 0..1)),
    PositionedToken::new(token![MOV],                position!(106..108, 8, 2..4)),
    PositionedToken::new(token![WhiteSpace("  ")],   position!(109..110, 8, 5..6)),
    PositionedToken::new(token![R0],                 position!(111..112, 8, 7..8)),
    PositionedToken::new(token![ArgumentSeparator],  position!(113, 8, 9)),
    PositionedToken::new(token![ImmediateModifier],  position!(114, 8, 10)),
    PositionedToken::new(token![Other("TAB")],       position!(115..117, 8, 11..13)),
    PositionedToken::new(token![NewLine("\n")],      position!(118, 8, 14)),
    
    // line 9
    PositionedToken::new(token![WhiteSpace(" \t")],  position!(119..120, 9, 0..1)),
    PositionedToken::new(token![CALL],               position!(121..124, 9, 2..5)),
    PositionedToken::new(token![WhiteSpace("  ")],   position!(125..126, 9, 6..7)),
    PositionedToken::new(token![Other("LCD_CLR")],   position!(127..133, 9, 8..14)),
    PositionedToken::new(token![NewLine("\n")],      position!(134, 9, 15)),

    // line 10
    PositionedToken::new(token![Label("ABCD")],      position!(135..138, 10, 0..3)),
    PositionedToken::new(token![LabelEnd],           position!(139, 10, 4)),
    PositionedToken::new(token![NewLine("\n")],      position!(140, 10, 5)),
    
    // line 11
    PositionedToken::new(token![WhiteSpace(" \t")],  position!(141..142, 11, 0..1)),
    PositionedToken::new(token![MOV],                position!(143..145, 11, 2..4)),
    PositionedToken::new(token![WhiteSpace("  ")],   position!(146..147, 11, 5..6)),
    PositionedToken::new(token![A],                  position!(148, 11, 7)),
    PositionedToken::new(token![ArgumentSeparator],  position!(149, 11, 8)),
    PositionedToken::new(token![AddressingModifier], position!(150, 11, 9)),
    PositionedToken::new(token![R0],                 position!(151..152, 11, 10..11)),
    PositionedToken::new(token![NewLine("\n")],      position!(153, 11, 12)),

    // line 12
    PositionedToken::new(token![WhiteSpace(" \t")],  position!(154..155, 12, 0..1)),
    PositionedToken::new(token![CALL],               position!(156..159, 12, 2..5)),
    PositionedToken::new(token![WhiteSpace("  ")],   position!(160..161, 12, 6..7)),
    PositionedToken::new(token![Other("WRITE_HEX")], position!(162..170, 12, 8..16)),
    PositionedToken::new(token![NewLine("\n")],      position!(171, 12, 17)),

    // line 13
    PositionedToken::new(token![WhiteSpace(" \t")],  position!(172..173, 13, 0..1)),
    PositionedToken::new(token![MOV],                position!(174..176, 13, 2..4)),
    PositionedToken::new(token![WhiteSpace("  ")],   position!(177..178, 13, 5..6)),
    PositionedToken::new(token![A],                  position!(179, 13, 7)),
    PositionedToken::new(token![ArgumentSeparator],  position!(180, 13, 8)),
    PositionedToken::new(token![NumberD("5")],       position!(181..182, 13, 9..10)),
    PositionedToken::new(token![NewLine("\n")],      position!(183, 13, 11)),

    // line 14
    PositionedToken::new(token![WhiteSpace(" \t")],    position!(184..185, 14, 0..1)),
    PositionedToken::new(token![CALL],                 position!(186..189, 14, 2..5)),
    PositionedToken::new(token![WhiteSpace("  ")],     position!(190..191, 14, 6..7)),
    PositionedToken::new(token![Other("DELAY_100MS")], position!(192..202, 14, 8..18)),
    PositionedToken::new(token![NewLine("\n")],        position!(203, 14, 19)),

    // line 15
    PositionedToken::new(token![WhiteSpace(" \t")],    position!(204..205, 15, 0..1)),
    PositionedToken::new(token![CALL],                 position!(206..209, 15, 2..5)),
    PositionedToken::new(token![WhiteSpace("  ")],     position!(210..211, 15, 6..7)),
    PositionedToken::new(token![Other("LCD_CLR")],     position!(212..218, 15, 8..14)),
    PositionedToken::new(token![NewLine("\n")],        position!(219, 15, 15)),
    
    // line 16
    PositionedToken::new(token![WhiteSpace(" \t")],    position!(220..221, 16, 0..1)),
    PositionedToken::new(token![INC],                  position!(222..224, 16, 2..4)),
    PositionedToken::new(token![WhiteSpace("  ")],     position!(225..226, 16, 5..6)),
    PositionedToken::new(token![R0],                   position!(227..228, 16, 7..8)),
    PositionedToken::new(token![NewLine("\n")],        position!(229, 16, 9)),

    // line 17
    PositionedToken::new(token![WhiteSpace(" \t")],    position!(230..231, 17, 0..1)),
    PositionedToken::new(token![CJNE],                 position!(232..235, 17, 2..5)),
    PositionedToken::new(token![WhiteSpace("  ")],     position!(236..237, 17, 6..7)),
    PositionedToken::new(token![R0],                   position!(238..239, 17, 8..9)),
    PositionedToken::new(token![ArgumentSeparator],    position!(240, 17, 10)),
    PositionedToken::new(token![ImmediateModifier],    position!(241, 17, 11)),
    PositionedToken::new(token![Other("TAB")],         position!(242..244, 17, 12..14)),
    PositionedToken::new(token![+],                    position!(245, 17, 15)),
    PositionedToken::new(token![AddressD("3")],        position!(246, 17, 16)),
    PositionedToken::new(token![ArgumentSeparator],    position!(247, 17, 17)),
    PositionedToken::new(token![Other("ABCD")],        position!(248..251, 17, 18..21)),
    PositionedToken::new(token![NewLine("\n")],        position!(252, 17, 22)),

    // line 18
    PositionedToken::new(token![WhiteSpace("\t")],     position!(253, 18, 0)),
    PositionedToken::new(token![MOV],                  position!(254..256, 18, 1..3)),
    PositionedToken::new(token![WhiteSpace("  ")],     position!(257..258, 18, 4..5)),
    PositionedToken::new(token![R0],                   position!(259..260, 18, 6..7)),
    PositionedToken::new(token![ArgumentSeparator],    position!(261, 18, 8)),
    PositionedToken::new(token![ImmediateModifier],    position!(262, 18, 9)),
    PositionedToken::new(token![Other("TAB")],         position!(263..265, 18, 10..12)),
    PositionedToken::new(token![NewLine("\n")],        position!(266, 18, 13)),
    
    // line 19
    PositionedToken::new(token![WhiteSpace(" \t")],    position!(267..268, 19, 0..1)),
    PositionedToken::new(token![SJMP],                 position!(269..272, 19, 2..5)),
    PositionedToken::new(token![WhiteSpace("  ")],     position!(273..274, 19, 6..7)),
    PositionedToken::new(token![Other("ABCD")],        position!(275..278, 19, 8..11)),
    PositionedToken::new(token![NewLine("\n")],        position!(279, 19, 12)),
]
; "t2")]
fn a_valid_code(src: &str, expected_ast: Vec<PositionedToken>) {
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