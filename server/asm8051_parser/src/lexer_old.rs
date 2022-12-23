use std::{fmt::{Display, Debug}, borrow::Borrow};
use chumsky::{prelude::*, text::Character};

pub type Span = std::ops::Range<usize>;


#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Label(String),
    Address(Number),
    Comment(String),
    Number(Number),
    ControlCharacter(char),
    Trivia(Trivia),
    NewLine,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Instruction(String),
    Register(Register),
    Directive(String),
    FlagOrBit(String),
}

#[derive(Debug, PartialEq)]
pub enum Register {
    Main(String),
    Special(String),
    Helper(String),
    Port(String),
    Addressing(String)
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Binary(String),
    Octal(String),
    Decimal(String),
    Hexadecimal(String)
}

#[derive(Debug, PartialEq)]
pub enum Trivia {
    NewLine,
    WhiteSpace,
}

#[derive(Debug, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

impl Display for SpannedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, range: {}:{}",
            match self.token.borrow() {
                Token::Label(lb) => format!("Label: {}, range", lb.clone()),
                _ => todo!(),
            },
            self.span.start,
            self.span.end
        )
    }
}

impl SpannedToken {
    pub fn new(token: Token, span: Span) -> SpannedToken {
        SpannedToken { token, span }
    }
}



pub fn lexical_analisys(source: &str) -> (Option<Vec<SpannedToken>>, Vec<Simple<char>>) {
    let src = format!("\n{}", source);
    let (parsed_tokens, errors) = 
        lexer().parse_recovery_verbose(src.clone());

    let parsed_tokens = match parsed_tokens {
        None => return (parsed_tokens, errors),
        Some(t) => t,
    };

    let mut new_parsed_tokens = Vec::<SpannedToken>::with_capacity(parsed_tokens.len());

    for spanned_token in parsed_tokens {
        let start = if spanned_token.span.start == 0 { spanned_token.span.start } else { spanned_token.span.start - 1 };
        let end = if spanned_token.span.end == 0 { spanned_token.span.end } else { spanned_token.span.end - 1 };
        new_parsed_tokens.push(SpannedToken::new(spanned_token.token, Span { start, end }));
    }

    (Some(new_parsed_tokens), errors)
}



fn lexer() -> impl Parser<char, Vec<SpannedToken>, Error = Simple<char>> {
    

    let non_newline_whitespace = filter(|c: &char| 
        c.is_whitespace() && 
        match *c {
            '\n' | '\r' | '\x0B' | '\x0C' | '\u{0085}' | '\u{2028}' | '\u{2029}' => false,
            _ => true,
        } 
    );

    let comment = just(";")
        .ignore_then(filter(|c| *c != '\n').repeated())
        .then_ignore(just('\n').rewind())
        .collect::<String>()
        .map(Token::Comment);

    let comma = just(',')
        .map(Token::ControlCharacter);

    let newline = text::newline()
        .map(|_| Token::NewLine);

    let keywords = non_newline_whitespace.clone()
        .ignore_then(text::ident())
        .then_ignore(text::whitespace())
        .map(|ident: String| match ident.as_str() {

            "ORG" | "DB" | "DW" | "END" | "EQU" | "SET" | "BIT" | "REG" | "IF" | "ELSE" | "ENDIF" | "MACRO" | "ENDM" | "MACEND" | "EXITM" | "INCLUDE" | "INCDIR" 
                => Token::Keyword(Keyword::Directive(ident)),

            "NOP" | "AJMP" | "LJMP" | "RR" | "INC" | "JBC" | "ACALL" | "LCALL" | "RRC" | "DEC" | "JB" | "RL" | "ADD" | "JNB" | "RLC" | "ADDC" | "JC" | "ORL" | "JNC" | "ANL" | "JZ" | "XRL" | "JNZ" | "JMP" | "MOV" | "SJMP" | "MOVC" | "DIV" | "SUBB" | "MUL" | "CPL" | "CJNE" | "PUSH" | "CLR" | "SWAP" | "XCH" | "POP" | "SETB" | "DA" | "DJNZ" | "XCHD" | "MOVX" | "CALL" 
                => Token::Keyword(Keyword::Instruction(ident)),

            "A" | "B" | "AB" 
                => Token::Keyword(Keyword::Register(Register::Main(ident))),

            "R0" | "R1" | "R2" | "R3" | "R4" | "R5" | "R6" | "R7"
                => Token::Keyword(Keyword::Register(Register::Helper(ident))),

            "@R0" | "@R1" | "@DPTR" | "@A+DPTR" | "@A+PC" 
                => Token::Keyword(Keyword::Register(Register::Addressing(ident))),

            "SCON" | "SBUF" | "P0" | "P1" | "P2" | "P3" 
                => Token::Keyword(Keyword::Register(Register::Port(ident))),

            "DPH" | "DPL" | "DPTR" | "IE" | "IP" | "PC" | "PCON" | "PSW" | "SP" | "TCON" | "TH0" | "TH1" | "TL0" | "TL1" | "TMOD" 
                => Token::Keyword(Keyword::Register(Register::Special(ident))),

            "AC" | "CY" | "IE0" | "IE1" | "IT0" | "IT1" | "OV" | "P" | "TF0" | "TF1" | "TR0" | "TR1"
                => Token::Keyword(Keyword::FlagOrBit(ident)),
            
            _ => Token::Label(ident),
        }
    );

    // #region lexer-numbers

    let number_bin = just('#')
        .then(text::digits(2))
        .then(just('b').or(just('B')))
        .map(|((_, number), _)| Token::Number(Number::Binary(format!("#{}B", number))));

    let number_dec = just('#')
        .then(text::digits(10))
        .map(|(_, number)| Token::Number(Number::Decimal(format!("#{}", number))));

    let number_hex = just('#')
        .then(text::digits(16))
        .then(just('h').or(just('H')))
        .map(|((_, number), _)| Token::Number(Number::Hexadecimal(format!("#{}H", number))));

    let number = number_bin.or(number_hex).or(number_dec);

    // #endregion lexer-numbers

    // #region lexer-addresses

    let address_bin = text::digits(2)
        .then(just('b').or(just('B')))
        .map(|(number, _)| Token::Address(Number::Binary(format!("{}B", number))));

    let address_dec = text::digits(10)
        .map(|number| Token::Address(Number::Decimal(format!("{}", number))));

    let address_hex = text::digits(16)
        .then(just('h').or(just('H')))
        .map(|(number, _)| Token::Address(Number::Hexadecimal(format!("{}H", number))));

    let address = address_bin.or(address_hex).or(address_dec);

    // #endregion lexer-addresses

    let label = text::ident()
        .then_ignore(just(':').or_not())
        .map(|id| Token::Label(id));
    

    let token = comment.padded()
        .or(comma)
        .or(newline)
        .or(keywords)
        .or(number)
        .or(address)
        .or(label)
        .recover_with(skip_then_retry_until([]));

    token.map_with_span(|token, span| SpannedToken{token, span})
        .repeated()
}

// #[cfg(test)]
// mod tests {

//     use super::*;
//     //use chumsky::chain::Chain;
//     use test_case::test_case;

//     #[test_case(
//         "LABEL1: \nLABEL2: \nLABEL3 EQU 32h",
//         vec!(
//             Token::Label(String::from("LABEL1")),
//             Token::NewLine,

//             Token::Label(String::from("LABEL2")),
//             Token::NewLine,

//             Token::Label(String::from("LABEL3")), 
//             Token::Keyword(Keyword::Directive(String::from("EQU"))),
//             Token::Number(Number::Hexadecimal(String::from("32H")))
//         ) ; "_1")
//     ]
//     #[test_case(
//         r#"
//     LJMP  START
//     ORG  100H
// START:
//     MOV A, #0
//     MOV R1, #1
// LOOP:
//     add A, R1
//     ADDC A, R2
//     SUBB A, R3
//     CALL WRITE_HEX
//     SJMP LOOP
//     DEC R7
//     RET ;🤨🤨🤨a̐ą̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨ą̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨ąąéö̲\r\n"#,
//         vec!(
//             Token::Keyword(Keyword::Instruction(String::from("LJMP"))),
//             Token::Label(String::from("START")),
//             Token::NewLine,

//             Token::Keyword(Keyword::Directive(String::from("ORG"))),
//             Token::Address(Number::Hexadecimal(String::from("100H"))),
//             Token::NewLine,

//             Token::Label(String::from("START")),
//             Token::NewLine,

//             Token::Keyword(Keyword::Instruction(String::from("MOV"))),
//             Token::Keyword(Keyword::Register(Register::Main(String::from("A")))),
//             Token::ControlCharacter(','),
//             Token::Number(Number::Decimal(String::from("#0"))),
//             Token::NewLine,

//             Token::Keyword(Keyword::Instruction(String::from("MOV"))),
//             Token::Keyword(Keyword::Register(Register::Helper(String::from("R1")))),
//             Token::ControlCharacter(','),
//             Token::Number(Number::Decimal(String::from("#1"))),
//             Token::NewLine,

//             Token::Label(String::from("LOOP")),
//             Token::NewLine,

//             // add is not valid
//             Token::Keyword(Keyword::Register(Register::Main(String::from("A")))),
//             Token::ControlCharacter(','),
//             Token::Keyword(Keyword::Register(Register::Helper(String::from("R1")))),
//             Token::NewLine,

//             Token::Keyword(Keyword::Instruction(String::from("ADDC"))),
//             Token::Keyword(Keyword::Register(Register::Main(String::from("A")))),
//             Token::ControlCharacter(','),
//             Token::Keyword(Keyword::Register(Register::Helper(String::from("R2")))),
//             Token::NewLine,

//             Token::Keyword(Keyword::Instruction(String::from("SUBB"))),
//             Token::Keyword(Keyword::Register(Register::Main(String::from("A")))),
//             Token::ControlCharacter(','),
//             Token::Keyword(Keyword::Register(Register::Helper(String::from("R3")))),
//             Token::NewLine,

//             Token::Keyword(Keyword::Instruction(String::from("CALL"))),
//             Token::Label(String::from("WRITE_HEX")),
//             Token::NewLine,

//             Token::Keyword(Keyword::Instruction(String::from("SJMP"))),
//             Token::Label(String::from("LOOP")),
//             Token::NewLine,

//             Token::Keyword(Keyword::Instruction(String::from("DEC"))),
//             Token::Keyword(Keyword::Register(Register::Helper(String::from("R7")))),
//             Token::NewLine,

//             Token::Keyword(Keyword::Instruction(String::from("RET"))),
//             Token::Comment(String::from("🤨🤨🤨a̐ą̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨ą̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨̨ąąéö̲\\r\\n")),
//             Token::NewLine,
//         ) ; "_2")]
//     fn lexer_identifies_labels_at_the_start_of_any_line(src: &str, expected_tokens: Vec<Token>) {
//         let (parsed_tokens, _errors) = lexical_analisys(src.clone());
        
//         assert!(parsed_tokens.is_some(), "AST was not generated");

//         let parsed_tokens = parsed_tokens.unwrap();

//         assert_eq!(parsed_tokens.len(), expected_tokens.len(), "unexpected length");

//         for i in 0..(&expected_tokens).len() {
//             let parsed = &parsed_tokens[i];
//             let expected = &expected_tokens[i];
//             assert_eq!(&parsed.token, expected, "Unexpected token");
//         }
//     }
// }