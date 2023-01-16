
use test_case::test_case;
use crate::lexer::keywords;


#[test_case("ACALL"; "instruction ACALL")]
#[test_case("ADD"; "instruction ADD")]
#[test_case("ADDC"; "Ainstruction DDC")]
#[test_case("AJMP"; "instruction AJMP")]
#[test_case("ANL"; "instruction ANL")]
#[test_case("CJNE"; "instruction CJNE")]
#[test_case("CLR"; "instruction CLR")]
#[test_case("CPL"; "instruction CPL")]
#[test_case("DA"; "instruction DA")]
#[test_case("DEC"; "instruction DEC")]
#[test_case("DIV"; "instruction DIV")]
#[test_case("DJNZ"; "instruction DJNZ")]
#[test_case("INC"; "instruction INC")]
#[test_case("JB"; "instruction JB")]
#[test_case("JBC"; "instruction JBC")]
#[test_case("JC"; "instruction JC")]
#[test_case("JMP"; "instruction JMP")]
#[test_case("JNB"; "instruction JNB")]
#[test_case("JNC"; "instruction JNC")]
#[test_case("JNZ"; "instruction JNZ")]
#[test_case("JZ"; "instruction JZ")]
#[test_case("LCALL"; "instruction LCALL")]
#[test_case("LJMP"; "instruction LJMP")]
#[test_case("MOV"; "instruction MOV")]
#[test_case("MOVC"; "instruction MOVC")]
#[test_case("MOVX"; "instruction MOVX")]
#[test_case("MUL"; "instruction MUL")]
#[test_case("NOP"; "instruction NOP")]
#[test_case("ORL"; "instruction ORL")]
#[test_case("POP"; "instruction POP")]
#[test_case("PUSH"; "instruction PUSH")]
#[test_case("RET"; "instruction RET")]
#[test_case("RETI"; "instruction RETI")]
#[test_case("RL"; "instruction RL")]
#[test_case("RLC"; "instruction RLC")]
#[test_case("RR"; "instruction RR")]
#[test_case("RRC"; "instruction RRC")]
#[test_case("SETB"; "instruction SETB")]
#[test_case("SJMP"; "instruction SJMP")]
#[test_case("SUBB"; "instruction SUBB")]
#[test_case("SWAP"; "instruction SWAP")]
#[test_case("XCH"; "instruction XCH")]
#[test_case("XCHD"; "instruction XCHD")]
#[test_case("XRL"; "instruction XRL")]
#[test_case("BIT"; "directive BIT")]
#[test_case("DB"; "directive DB")]
#[test_case("DW"; "directive DW")]
#[test_case("IF"; "directive _IF")]
#[test_case("ELSEIF"; "directive ELSEIF")]
#[test_case("ENDIF"; "directive ENDIF")]
#[test_case("END"; "directive END")]
#[test_case("ENDM"; "directive ENDM")]
#[test_case("EQU"; "directive EQU")]
#[test_case("$INCDIR"; "directive $INCDIR")]
#[test_case("$INCLUDE"; "directive $INCLUDE")]
#[test_case("MACRO"; "directive _MACRO")]
#[test_case("MACEND"; "directive MACEND")]
#[test_case("ORG"; "directive ORG")]
#[test_case("REG"; "directive REG")]
#[test_case("SET"; "directive SET")]
#[test_case("@R1"; "register addressing")]
fn is_keyword_returns_true_for_any_known_instruction_directive_register_or_flag(s: &str) {
    assert!(keywords::is_keyword(s), "Invalid keyword: {} (using a &str)", s);
    let string = String::from(s);
    assert!(keywords::is_keyword(&string), "Invalid keyword: {} (using a &String)", s);
    assert!(keywords::is_keyword(string), "Invalid keyword: {} (using a String)", s);
}

#[test_case("AAAA"; "AAAA")]
#[test_case("🤓🤓🤓🤓"; "emojis")]
#[test_case("1234"; "1234")]
fn is_keyword_returns_false_for_any_unknown_instruction_directive_register_or_flag(s: &str) {
    assert!(!keywords::is_keyword(s), "Valid keyword: {} (using a &str)", s);
    let string = String::from(s);
    assert!(!keywords::is_keyword(&string), "Valid keyword: {} (using a &String)", s);
    assert!(!keywords::is_keyword(string), "Valid keyword: {} (using a String)", s);
}

#[test_case("ACALL"; "ACALL")]
#[test_case("ADD"; "ADD")]
#[test_case("ADDC"; "ADDC")]
#[test_case("AJMP"; "AJMP")]
#[test_case("ANL"; "ANL")]
#[test_case("CJNE"; "CJNE")]
#[test_case("CLR"; "CLR")]
#[test_case("CPL"; "CPL")]
#[test_case("DA"; "DA")]
#[test_case("DEC"; "DEC")]
#[test_case("DIV"; "DIV")]
#[test_case("DJNZ"; "DJNZ")]
#[test_case("INC"; "INC")]
#[test_case("JB"; "JB")]
#[test_case("JBC"; "JBC")]
#[test_case("JC"; "JC")]
#[test_case("JMP"; "JMP")]
#[test_case("JNB"; "JNB")]
#[test_case("JNC"; "JNC")]
#[test_case("JNZ"; "JNZ")]
#[test_case("JZ"; "JZ")]
#[test_case("LCALL"; "LCALL")]
#[test_case("LJMP"; "LJMP")]
#[test_case("MOV"; "MOV")]
#[test_case("MOVC"; "MOVC")]
#[test_case("MOVX"; "MOVX")]
#[test_case("MUL"; "MUL")]
#[test_case("NOP"; "NOP")]
#[test_case("ORL"; "ORL")]
#[test_case("POP"; "POP")]
#[test_case("PUSH"; "PUSH")]
#[test_case("RET"; "RET")]
#[test_case("RETI"; "RETI")]
#[test_case("RL"; "RL")]
#[test_case("RLC"; "RLC")]
#[test_case("RR"; "RR")]
#[test_case("RRC"; "RRC")]
#[test_case("SETB"; "SETB")]
#[test_case("SJMP"; "SJMP")]
#[test_case("SUBB"; "SUBB")]
#[test_case("SWAP"; "SWAP")]
#[test_case("XCH"; "XCH")]
#[test_case("XCHD"; "XCHD")]
#[test_case("XRL"; "XRL")]
fn is_instruction_returns_true_for_a_known_instruction(s: &str) {
    assert!(keywords::is_instruction(s), "Invalid instruction: {} (using a &str)", s);
    let string = String::from(s);
    assert!(keywords::is_instruction(&string), "Invalid instruction: {} (using a &String)", s);
    assert!(keywords::is_instruction(string), "Invalid instruction: {} (using a String)", s);
}

#[test_case("AAAA"; "AAAA")]
#[test_case("🤓🤓🤓🤓"; "emojis")]
#[test_case("1234"; "1234")]
fn is_instruction_returns_false_for_a_unknown_instruction(s: &str) {
    assert!(!keywords::is_instruction(s), "Valid instruction: {} (using a &str)", s);
    let string = String::from(s);
    assert!(!keywords::is_instruction(&string), "Valid instruction: {} (using a &String)", s);
    assert!(!keywords::is_instruction(string), "Valid instruction: {} (using a String)", s);
}

#[test_case("BIT"; "BIT")]
#[test_case("DB"; "DB")]
#[test_case("DW"; "DW")]
#[test_case("IF"; "_IF")]
#[test_case("ELSEIF"; "ELSEIF")]
#[test_case("ENDIF"; "ENDIF")]
#[test_case("END"; "END")]
#[test_case("ENDM"; "ENDM")]
#[test_case("EQU"; "EQU")]
#[test_case("$INCDIR"; "$INCDIR")]
#[test_case("$INCLUDE"; "$INCLUDE")]
#[test_case("MACRO"; "_MACRO")]
#[test_case("MACEND"; "MACEND")]
#[test_case("ORG"; "ORG")]
#[test_case("REG"; "REG")]
#[test_case("SET"; "SET")]
fn is_directive_returns_true_for_a_known_directive(s: &str) {
    assert!(keywords::is_directive(s), "Invalid directive: {} (using a &str)", s);
    let string = String::from(s);
    assert!(keywords::is_directive(&string), "Invalid directive: {} (using a &String)", s);
    assert!(keywords::is_directive(string), "Invalid directive: {} (using a String)", s);
}

#[test_case("AAAA"; "AAAA")]
#[test_case("🤓🤓🤓🤓"; "emojis")]
#[test_case("1234"; "1234")]
fn is_directive_returns_false_for_a_unknown_directive(s: &str) {
    assert!(!keywords::is_directive(s), "Valid directive: {} (using a &str)", s);
    let string = String::from(s);
    assert!(!keywords::is_directive(&string), "Invalid directive: {} (using a &String)", s);
    assert!(!keywords::is_directive(string), "Invalid directive: {} (using a String)", s);
}