//#region Tokens
use std::borrow::Borrow;
use super::Position;


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(Keyword),
    Label(String),
    Address(Number),
    String(String),
    Number(Number),
    ControlCharacter(ControlCharacter),
    Trivia(Trivia),
    Other(String),
    Unknown(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Instruction(Instruction),
    Register(Register),
    Directive(Directive),
    FlagOrBit(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    ACALL, ADD, ADDC, AJMP, ANL, CJNE, CLR, CPL,
    DA, DEC, DIV, DJNZ, INC, JB, JBC, JC, 
    JMP, JNB, JNC, JNZ, JZ, LCALL, LJMP, MOV,
    MOVC, MOVX, MUL, NOP, ORL, POP, PUSH, RET,
    RETI, RL, RLC, RR, RRC, SETB, SJMP, SUBB, SWAP,
    XCH, XCHD, XRL,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Directive {
    BIT, DB, DW, IF, ELSEIF, ENDIF,
    END, ENDM, EQU, INCDIR, INCLUDE, 
    MACRO, MACEND, ORG, REG, SET,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Register {
    Main(MainRegister),
    Special(SpecialRegister),
    Helper(HelperRegister),
    Port(PortRegister),
    Addressing(AddressingRegister)
}

#[derive(Debug, PartialEq, Clone)]
pub enum MainRegister {
    A, B, AB
}

#[derive(Debug, PartialEq, Clone)]
pub enum SpecialRegister {
    TL0,  TH0,
    TL1,  TH1,
    DPL,  DPH, DPTR,
    IE,   IP,  PC,
    PCON, PSW, 
    SBUF, SCON, SP
}

#[derive(Debug, PartialEq, Clone)]
pub enum HelperRegister {
    R0, R1, R2, R3, R4, R5, R6, R7
}

#[derive(Debug, PartialEq, Clone)]
pub enum AddressingRegister {
    R0, R1
}

#[derive(Debug, PartialEq, Clone)]
pub enum PortRegister {
    P0, P1, P2, P3
}

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Binary(String),
    Octal(String),
    Decimal(String),
    Hexadecimal(String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Trivia {
    NewLine(String),
    WhiteSpace(String),
    Comment(String),
}
#[derive(Debug, PartialEq, Clone)]
pub enum ControlCharacter {
    Arithmetic(Arithmetic),
    AddressingModifier,
    ArgumentSeparator,
    Parenthesis(Parenthesis),
    Delimiter(Delimiter)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Arithmetic {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Parenthesis {
    Open,
    Close,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Delimiter {
    CommentStart,
    LabelEnd,
    SingleQuote,
    DoubleQuote,
}

//#endregion

//#region positioned token

#[derive(Debug, PartialEq, Clone)]
pub struct PositionedToken {
    pub token: Token,
    pub position: Position,
}

impl std::fmt::Display for PositionedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, range: {}-{}; line: {}; columns: {}-{}",
            match self.token.borrow() {
                Token::Label(lb) => format!("Label: {}, range", lb.clone()),
                _ => todo!(),
            },
            self.position.range.start,
            self.position.range.end,
            self.position.line,
            self.position.columns.start,
            self.position.columns.end
        )
    }
}

impl PositionedToken {
    pub fn new(token: Token, position: Position) -> PositionedToken {
        PositionedToken { token, position }
    }
}
//#endregion
//#endregion


//#region Macro
#[macro_export]
macro_rules! Tkn {
    
    // Instructions
    [ACALL]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::ACALL)) };
    [ADD]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::ADD)) };
    [ADDC]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::ADDC)) };
    [AJMP]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::AJMP)) };
    [ANL]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::ANL)) };
    [CJNE]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::CJNE)) };
    [CLR]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::CLR)) };
    [CPL]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::CPL)) };
    [DA]      => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::DA)) };
    [DEC]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::DEC)) };
    [DIV]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::DIV)) };
    [DJNZ]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::DJNZ)) };
    [INC]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::INC)) };
    [JB]      => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::JB)) };
    [JBC]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::JBC)) };
    [JC]      => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::JC)) };
    [JMP]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::JMP)) };
    [JNB]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::JNB)) };
    [JNC]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::JNC)) };
    [JNZ]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::JNZ)) };
    [JZ]      => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::JZ)) };
    [LCALL]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::LCALL)) };
    [LJMP]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::LJMP)) };
    [MOV]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::MOV)) };
    [MOVC]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::MOVC)) };
    [MOVX]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::MOVX)) };
    [MUL]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::MUL)) };
    [NOP]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::NOP)) };
    [ORL]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::ORL)) };
    [POP]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::POP)) };
    [PUSH]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::PUSH)) };
    [RET]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::RET)) };
    [RETI]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::RETI)) };
    [RL]      => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::RL)) };
    [RLC]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::RLC)) };
    [RR]      => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::RR)) };
    [RRC]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::RRC)) };
    [SETB]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::SETB)) };
    [SJMP]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::SJMP)) };
    [SUBB]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::SUBB)) };
    [SWAP]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::SWAP)) };
    [XCH]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::XCH)) };
    [XCHD]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::XCHD)) };
    [XRL]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::XRL)) };

    // Directives
    [BIT]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::BIT)) };
    [DB]      => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::DB)) };
    [DW]      => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::DW)) };
    [IF]      => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::IF)) };
    [ELSEIF]  => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::ELSEIF)) };
    [ENDIF]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::ENDIF)) };
    [END]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::END)) };
    [ENDM]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::ENDM)) };
    [EQU]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::EQU)) };
    [INCDIR]  => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::INCDIR)) };
    [INCLUDE] => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::INCLUDE)) };
    [MACRO]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::MACRO)) };
    [MACEND]  => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::MACEND)) };
    [ORG]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::ORG)) };
    [REG]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::REG)) };
    [SET]     => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Directive(crate::lexer::tokens::Directive::SET)) };

    // Main Registers
    [A]       => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Main(crate::lexer::tokens::MainRegister::A))) };
    [B]       => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Main(crate::lexer::tokens::MainRegister::B))) };
    [AB]      => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Main(crate::lexer::tokens::MainRegister::AB))) };

    // Special Registers
    [TL0]  => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::TL0))) };
    [TH0]  => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::TH0))) };
    [TL1]  => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::TL1))) };
    [TH1]  => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::TH1))) };
    [DPL]  => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::DPL))) };
    [DPH]  => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::DPH))) };
    [DPTR] => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::DPTR))) };
    [IE]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::IE))) };
    [IP]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::IP))) };
    [PC]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::PC))) };
    [PCON] => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::PCON))) };
    [PSW]  => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::PSW))) };
    [SBUF] => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::SBUF))) };
    [SCON] => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::SCON))) };
    [SP]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Special(crate::lexer::tokens::SpecialRegister::SP))) };

    // Helper registers
    [R0]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Helper(crate::lexer::tokens::HelperRegister::R0))) };
    [R1]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Helper(crate::lexer::tokens::HelperRegister::R1))) };
    [R2]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Helper(crate::lexer::tokens::HelperRegister::R2))) };
    [R3]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Helper(crate::lexer::tokens::HelperRegister::R3))) };
    [R4]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Helper(crate::lexer::tokens::HelperRegister::R4))) };
    [R5]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Helper(crate::lexer::tokens::HelperRegister::R5))) };
    [R6]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Helper(crate::lexer::tokens::HelperRegister::R6))) };
    [R7]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Helper(crate::lexer::tokens::HelperRegister::R7))) };

    // Addressing registers
    [@R0]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Addressing(crate::lexer::tokens::AddressingRegister::R0))) };
    [@R1]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Addressing(crate::lexer::tokens::AddressingRegister::R1))) };

    // Port registers
    [P0]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Port(crate::lexer::tokens::PortRegister::P0))) };
    [P1]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Port(crate::lexer::tokens::PortRegister::P1))) };
    [P2]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Port(crate::lexer::tokens::PortRegister::P2))) };
    [P3]   => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Register(crate::lexer::tokens::Register::Port(crate::lexer::tokens::PortRegister::P3))) };



    // I'll add things as I go

    // Delimiters
    [Apostrophe]  => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Delimiter(crate::lexer::tokens::Delimiter::SingleQuote)) };
    [SingleQuote] => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Delimiter(crate::lexer::tokens::Delimiter::SingleQuote)) };
    [DoubleQuote] => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Delimiter(crate::lexer::tokens::Delimiter::DoubleQuote)) };


    // Trivia 
    [String($s:expr)] => { crate::lexer::tokens::Token::String($s) };
    [str($s:expr)] => { crate::lexer::tokens::Token::String(std::string::String::from($s)) };
}

//#endregion