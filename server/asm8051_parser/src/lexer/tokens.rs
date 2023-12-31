//#region Tokens
use std::{borrow::Borrow, fmt::{Display, self}};
use super::Position;
use asm8051_shared::PossibleOperand;

#[derive(PartialEq, Clone)]
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

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Keyword(arg0) => f.debug_tuple("Keyword").field(arg0).finish(),
            Self::Label(arg0) => f.debug_tuple("Label").field(arg0).finish(),
            Self::Address(arg0) => f.debug_tuple("Address").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::ControlCharacter(arg0) => f.debug_tuple("ControlCharacter").field(arg0).finish(),
            Self::Trivia(arg0) => f.debug_tuple("Trivia").field(arg0).finish(),
            Self::Other(arg0) => f.debug_tuple("Other").field(arg0).finish(),
            Self::Unknown(arg0) => f.debug_tuple("Unknown").field(arg0).finish(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(kw) => write!(f, "Token::Keyword({})", kw),
            Token::Label(lb)    => write!(f, "Token::Label({})", lb),
            Token::Address(ad)  => write!(f, "Token::Address({})", ad),
            Token::String(st)   => write!(f, "Token::String({})", st),
            Token::Number(nb)   => write!(f, "Token::Number({})", nb),
            Token::ControlCharacter(cc) => write!(f, "Token::ControlCharacter({})", cc),
            Token::Trivia(tv)  => write!(f, "Token::Trivia({})", tv),
            Token::Other(ot)   => write!(f, "Token::Other({})", ot),
            Token::Unknown(u)  => write!(f, "Token::Unknown({})", u),
        }
    }
}

impl Token {
    pub fn is_label(&self) -> bool
    {
        match self {
            Self::Label(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_label(&self) -> String 
    {
        match self {
            Self::Label(s) => s.clone(),
            _ => panic!("Token is not a label"),
        }
    }

    pub fn is_other(&self) -> bool {
        match self {
            Self::Other(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_other(&self) -> String 
    {
        match self {
            Self::Other(s) => s.clone(),
            _ => panic!("Token is not an other"),
        }
    }

    pub fn string_repr(&self) -> String {
        match self {
            Token::Keyword(kw) => kw.string_repr(),
            Token::Label(_) => todo!(),
            Token::Address(_) => todo!(),
            Token::String(_) => todo!(),
            Token::Number(_) => todo!(),
            Token::ControlCharacter(_) => todo!(),
            Token::Trivia(_) => todo!(),
            Token::Other(_) => todo!(),
            Token::Unknown(_) => todo!(),
        }
    }

    pub fn to_possible_operand(&self) -> PossibleOperand {
        match self {
            Token::Keyword(kw) => match kw {
                Keyword::Register(reg) => match reg {
                    Register::Main(m) => match m {
                        MainRegister::A => PossibleOperand::Accumulator,
                        MainRegister::B => PossibleOperand::RegisterB,
                        MainRegister::AB => PossibleOperand::AccumulatorAndB ,
                    },
                    Register::Special(s) => match s {
                        SpecialRegister::DPL => PossibleOperand::Dpl,
                        SpecialRegister::DPH => PossibleOperand::Dph,
                        SpecialRegister::DPTR => PossibleOperand::Dptr,
                        _ => PossibleOperand::CodeAddress,
                    },
                    Register::Helper(_) => PossibleOperand::HelperRegisters,
                    Register::Port(_) => PossibleOperand::CodeAddress,
                },
                Keyword::FlagOrBit(_) => PossibleOperand::BitAddress,
                _ => PossibleOperand::Any,
            },
            Token::Label(_) => PossibleOperand::Label,
            Token::Address(_) => PossibleOperand::CodeAddress,
            Token::String(_) => PossibleOperand::AsciiCharacters,
            Token::Number(n) => {
                if n.to_i32() > 255 { PossibleOperand::Data16 } else { PossibleOperand::Data }
            },
            _ => PossibleOperand::Any,
        }
    }

}


#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Instruction(Instruction),
    Register(Register),
    Directive(Directive),
    FlagOrBit(String),
}

impl Keyword {
    pub fn string_repr(&self) -> String {
        match self {
            Keyword::Instruction(instr) => instr.string_repr(),
            Keyword::Register(reg) => reg.string_repr(),
            Keyword::Directive(_) => todo!(),
            Keyword::FlagOrBit(_) => todo!(),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Instruction(inst) => write!(f, "Keyword::Instruction({})", inst),
            Keyword::Register(regi)    => write!(f, "Keyword::Register({})", regi),
            Keyword::Directive(dire)   => write!(f, "Keyword::Directive({})", dire),
            Keyword::FlagOrBit(flbt)   => write!(f, "Keyword::FlagOrBit({})", flbt),
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    CALL, ACALL, ADD, ADDC, AJMP, ANL, CJNE, CLR,
    CPL, DA, DEC, DIV, DJNZ, INC, JB, JBC, 
    JC, JMP, JNB, JNC, JNZ, JZ, LCALL, LJMP, 
    MOV, MOVC, MOVX, MUL, NOP, ORL, POP, PUSH, 
    RET, RETI, RL, RLC, RR, RRC, SETB, SJMP, 
    SUBB, SWAP, XCH, XCHD, XRL,
}

impl Instruction {
    pub fn string_repr(&self) -> String {
        match self {
            Instruction::CALL => String::from("CALL"),
            Instruction::ACALL => String::from("ACALL"),
            Instruction::ADD => String::from("ADD"),
            Instruction::ADDC => String::from("ADDC"),
            Instruction::AJMP => String::from("AJMP"),
            Instruction::ANL => String::from("ANL"),
            Instruction::CJNE => String::from("CJNE"),
            Instruction::CLR => String::from("CLR"),
            Instruction::CPL => String::from("CPL"),
            Instruction::DA => String::from("DA"),
            Instruction::DEC => String::from("DEC"),
            Instruction::DIV => String::from("DIV"),
            Instruction::DJNZ => String::from("DJNZ"),
            Instruction::INC => String::from("INC"),
            Instruction::JB => String::from("JB"),
            Instruction::JBC => String::from("JBC"),
            Instruction::JC => String::from("JC"),
            Instruction::JMP => String::from("JMP"),
            Instruction::JNB => String::from("JNB"),
            Instruction::JNC => String::from("JNC"),
            Instruction::JNZ => String::from("JNZ"),
            Instruction::JZ => String::from("JZ"),
            Instruction::LCALL => String::from("LCALL"),
            Instruction::LJMP => String::from("LJMP"),
            Instruction::MOV => String::from("MOV"),
            Instruction::MOVC => String::from("MOVC"),
            Instruction::MOVX => String::from("MOVX"),
            Instruction::MUL => String::from("MUL"),
            Instruction::NOP => String::from("NOP"),
            Instruction::ORL => String::from("ORL"),
            Instruction::POP => String::from("POP"),
            Instruction::PUSH => String::from("PUSH"),
            Instruction::RET => String::from("RET"),
            Instruction::RETI => String::from("RETI"),
            Instruction::RL => String::from("RL"),
            Instruction::RLC => String::from("RLC"),
            Instruction::RR => String::from("RR"),
            Instruction::RRC => String::from("RRC"),
            Instruction::SETB => String::from("SETB"),
            Instruction::SJMP => String::from("SJMP"),
            Instruction::SUBB => String::from("SUBB"),
            Instruction::SWAP => String::from("SWAP"),
            Instruction::XCH => String::from("XCH"),
            Instruction::XCHD => String::from("XCHD"),
            Instruction::XRL => String::from("XRL"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Instruction::CALL => "CALL",
            Instruction::ACALL => "ACALL",
            Instruction::ADD => "ADD",
            Instruction::ADDC => "ADDC",
            Instruction::AJMP => "AJMP",
            Instruction::ANL => "ANL",
            Instruction::CJNE => "CJNE",
            Instruction::CLR => "CLR",
            Instruction::CPL => "CPL",
            Instruction::DA => "DA",
            Instruction::DEC => "DEC",
            Instruction::DIV => "DIV",
            Instruction::DJNZ => "DJNZ",
            Instruction::INC => "INC",
            Instruction::JB => "JB",
            Instruction::JBC => "JBC",
            Instruction::JC => "JC",
            Instruction::JMP => "JMP",
            Instruction::JNB => "JNB",
            Instruction::JNC => "JNC",
            Instruction::JNZ => "JNZ",
            Instruction::JZ => "JZ",
            Instruction::LCALL => "LCALL",
            Instruction::LJMP => "LJMP",
            Instruction::MOV => "MOV",
            Instruction::MOVC => "MOVC",
            Instruction::MOVX => "MOVX",
            Instruction::MUL => "MUL",
            Instruction::NOP => "NOP",
            Instruction::ORL => "ORL",
            Instruction::POP => "POP",
            Instruction::PUSH => "PUSH",
            Instruction::RET => "RET",
            Instruction::RETI => "RETI",
            Instruction::RL => "RL",
            Instruction::RLC => "RLC",
            Instruction::RR => "RR",
            Instruction::RRC => "RRC",
            Instruction::SETB => "SETB",
            Instruction::SJMP => "SJMP",
            Instruction::SUBB => "SUBB",
            Instruction::SWAP => "SWAP",
            Instruction::XCH => "XCH",
            Instruction::XCHD => "XCHD",
            Instruction::XRL => "XRL",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Directive {
    BIT, DB, DW, IF, ELSEIF, ENDIF,
    END, ENDM, EQU, INCDIR, INCLUDE, 
    MACRO, MACEND, ORG, REG, SET,
}

impl Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Directive::BIT => "BIT",
            Directive::DB => "DB",
            Directive::DW => "DW",
            Directive::IF => "IF",
            Directive::ELSEIF => "ELSEIF",
            Directive::ENDIF => "ENDIF",
            Directive::END => "END",
            Directive::ENDM => "ENDM",
            Directive::EQU => "EQU",
            Directive::INCDIR => "INCDIR",
            Directive::INCLUDE => "INCLUDE",
            Directive::MACRO => "MACRO",
            Directive::MACEND => "MACEND",
            Directive::ORG => "ORG",
            Directive::REG => "REG",
            Directive::SET => "SET",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Register {
    Main(MainRegister),
    Special(SpecialRegister),
    Helper(HelperRegister),
    Port(PortRegister),
    //Addressing(AddressingRegister)
}

impl Register {
    pub fn string_repr(&self) -> String {
        match self {
            Register::Main(m) => m.string_repr(),
            Register::Special(sp) => sp.string_repr(),
            Register::Helper(_) => todo!(),
            Register::Port(_) => todo!(),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::Main(mr)    => write!(f, "Register::Main({})", mr),
            Register::Special(sr) => write!(f, "Register::Special({})", sr),
            Register::Helper(hr)  => write!(f, "Register::Helper({})", hr),
            Register::Port(pr)    => write!(f, "Register::Port({})", pr),
            //Register::Addressing(_) => write!(f, "Register::Addressing({})", name),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MainRegister {
    A, B, AB
}

impl MainRegister {
    pub fn string_repr(&self) -> String {
        match self {
            MainRegister::A => String::from("A"),
            MainRegister::B => String::from("B"),
            MainRegister::AB => String::from("AB"),
        }
    }
}
impl Display for MainRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            MainRegister::A => "A",
            MainRegister::B => "B",
            MainRegister::AB => "AB",
        };
        write!(f, "{}", name)
    }
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

impl SpecialRegister {
    pub fn string_repr(&self) -> String {
        match self {
            SpecialRegister::TL0 => String::from("TL0"),
            SpecialRegister::TH0 => String::from("TH0"),
            SpecialRegister::TL1 => String::from("TL1"),
            SpecialRegister::TH1 => String::from("TH1"),
            SpecialRegister::DPL => String::from("DPL"),
            SpecialRegister::DPH => String::from("DPH"),
            SpecialRegister::DPTR => String::from("DPTR"),
            SpecialRegister::IE => String::from("IE"),
            SpecialRegister::IP => String::from("IP"),
            SpecialRegister::PC => String::from("PC"),
            SpecialRegister::PCON => String::from("PCON"),
            SpecialRegister::PSW => String::from("PSW"),
            SpecialRegister::SBUF => String::from("SBUF"),
            SpecialRegister::SCON => String::from("SCON"),
            SpecialRegister::SP => String::from("SP"),
        }
    }
}

impl Display for SpecialRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            SpecialRegister::TL0 => "TL0",
            SpecialRegister::TH0 => "TH0",
            SpecialRegister::TL1 => "TL1",
            SpecialRegister::TH1 => "TH1",
            SpecialRegister::DPL => "DPL",
            SpecialRegister::DPH => "DPH",
            SpecialRegister::DPTR => "DPTR",
            SpecialRegister::IE => "IE",
            SpecialRegister::IP => "IP",
            SpecialRegister::PC => "PC",
            SpecialRegister::PCON => "PCON",
            SpecialRegister::PSW => "PSW",
            SpecialRegister::SBUF => "SBUF",
            SpecialRegister::SCON => "SCON",
            SpecialRegister::SP => "SP",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum HelperRegister {
    R0, R1, R2, R3, R4, R5, R6, R7
}

impl Display for HelperRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            HelperRegister::R0 => "R0",
            HelperRegister::R1 => "R1",
            HelperRegister::R2 => "R2",
            HelperRegister::R3 => "R3",
            HelperRegister::R4 => "R4",
            HelperRegister::R5 => "R5",
            HelperRegister::R6 => "R6",
            HelperRegister::R7 => "R7",
        };
        write!(f, "{}", name)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum AddressingRegister {
    R0, R1
}

#[derive(Debug, PartialEq, Clone)]
pub enum PortRegister {
    P0, P1, P2, P3
}

impl Display for PortRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            PortRegister::P0 => "P0",
            PortRegister::P1 => "P1",
            PortRegister::P2 => "P2",
            PortRegister::P3 => "P3",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Binary(String),
    Octal(String),
    Decimal(String),
    Hexadecimal(String)
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Binary(num)      => write!(f, "Number::Binary({})", num),
            Number::Octal(num)       => write!(f, "Number::Octal({})", num),
            Number::Decimal(num)     => write!(f, "Number::Decimal({})", num),
            Number::Hexadecimal(num) => write!(f, "Number::Hexadecimal({})", num),
        }
    }
}

impl Number {
    pub fn to_i32(&self) -> i32 {
        match self {
            Number::Binary(bin) => i32::from_str_radix(bin.as_str(), 2),
            Number::Octal(oct) => i32::from_str_radix(oct.as_str(), 8),
            Number::Decimal(dec) => i32::from_str_radix(dec.as_str(), 10),
            Number::Hexadecimal(hex) => i32::from_str_radix(hex.as_str(), 16),
        }
        .unwrap_or(0)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum Trivia {
    NewLine(String),
    WhiteSpace(String),
    Comment(String),
}

impl Display for Trivia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Trivia::NewLine(cnt)    => write!(f, "Trivia::NewLine(\"{}\")", cnt),
            Trivia::WhiteSpace(cnt) => write!(f, "Trivia::WhiteSpace(\"{}\")", cnt),
            Trivia::Comment(cnt)    => write!(f, "Trivia::Comment(\"{}\")", cnt),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ControlCharacter {
    Arithmetic(Arithmetic),
    AddressingModifier,
    ArgumentSeparator,
    // Colon,
    AddressingSeparator,
    ImmediateModifier,
    Parenthesis(Parenthesis),
    Delimiter(Delimiter)
}

impl Display for ControlCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlCharacter::Arithmetic(cc) => write!(f, "ControlCharacter::Arithmetic({})", cc),
            ControlCharacter::AddressingModifier => write!(f, "ControlCharacter::AddressingModifier"),
            ControlCharacter::ArgumentSeparator => write!(f, "ControlCharacter::ArgumentSeparator"),
            //ControlCharacter::Colon => write!(f, "ControlCharacter::Colon"),
            ControlCharacter::ImmediateModifier => write!(f, "ControlCharacter::ImmediateModifier"),
            ControlCharacter::AddressingSeparator => write!(f, "ControlCharacter::AddressingSeparator"),
            ControlCharacter::Parenthesis(cc) => write!(f, "ControlCharacter::Parenthesis({})", cc),
            ControlCharacter::Delimiter(cc) => write!(f, "ControlCharacter::Delimiter({})", cc),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Arithmetic {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}
impl Display for Arithmetic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Arithmetic::Add => "Add",
            Arithmetic::Subtract => "Subtract",
            Arithmetic::Multiply => "Multiply",
            Arithmetic::Divide => "Divide",
            Arithmetic::Modulo => "Modulo",
        };
        write!(f, "Arithmetic::{}", name)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum Parenthesis {
    Open,
    Close,
}

impl Display for Parenthesis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Parenthesis::Open => "Open",
            Parenthesis::Close => "Close",
        };
        write!(f, "Parenthesis::{}", name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Delimiter {
    CommentStart,
    LabelEnd,
    SingleQuote,
    DoubleQuote,
}

impl Display for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Delimiter::CommentStart => "CommentStart",
            Delimiter::LabelEnd => "LabelEnd",
            Delimiter::SingleQuote => "SingleQuote",
            Delimiter::DoubleQuote => "DoubleQuote",
        };
        write!(f, "Delimiter::{}", name)
    }
}

//#endregion

//#region positioned token

#[derive(PartialEq, Clone)]
pub struct PositionedToken {
    pub token: Token,
    pub position: Position,
}


impl fmt::Debug for PositionedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PositionedToken")
         .field("token", &self.token.to_string())
         .field("position", &self.position).finish()
    }
}

impl std::fmt::Display for PositionedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {{range: {}-{}; line: {}; columns: {}-{}}}",
            self.token.borrow(),
            self.position.range.start,
            self.position.range.end,
            self.position.line,
            self.position.columns.start,
            self.position.columns.end
        )
    }
}

impl Default for PositionedToken {
    fn default() -> Self {
        Self { 
            token: Token::Unknown(String::new()),
            position: Position::new(0..0, 0, 0..0),
        }
    }
}

impl PositionedToken {
    pub fn new(token: Token, position: Position) -> PositionedToken {
        PositionedToken { token, position }
    }
}
//#endregion
//#endregion


pub mod helpers {
    use super::{Token, PositionedToken};

    pub fn tokens_to_strings(tokens: &Vec::<Token>) -> Vec::<String> {
        let mut result = Vec::<String>::new();


        for token in tokens {
            result.push(token.to_string());
        }

        result
    }

    pub fn positioned_tokens_to_strings(tokens: &Vec::<PositionedToken>) -> Vec::<String> {
        let mut result = Vec::<String>::new();

        for token in tokens {
            result.push(token.to_string());
        }

        result
    }
}


//#region Macro

#[macro_export]
macro_rules! position {
    ($range:literal, $line:literal) => {
        Position::new($range..$range, $line, $range..$range)
    };

    ($range:literal, $line:literal, $column:literal) => {
        Position::new($range..$range, $line, $column..$column)
    };

    ($range:expr, $line:literal) => {
        Position::new($range, $line, $range)
    };

    ($range:expr, $line:literal, $column:expr) => {
        Position::new($range, $line, $column)
    };
}

#[macro_export]
macro_rules! token {
    
    // Instructions
    [CALL]    => { crate::lexer::tokens::Token::Keyword(crate::lexer::tokens::Keyword::Instruction(crate::lexer::tokens::Instruction::CALL)) };
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
    [Apostrophe]   => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Delimiter(crate::lexer::tokens::Delimiter::SingleQuote)) };
    [SingleQuote]  => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Delimiter(crate::lexer::tokens::Delimiter::SingleQuote)) };
    [DoubleQuote]  => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Delimiter(crate::lexer::tokens::Delimiter::DoubleQuote)) };
    [Semicolon]    => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Delimiter(crate::lexer::tokens::Delimiter::CommentStart))};
    [CommentStart] => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Delimiter(crate::lexer::tokens::Delimiter::CommentStart))};
    [LabelEnd]     => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Delimiter(crate::lexer::tokens::Delimiter::LabelEnd))};

    // Control characters
    [ArgumentSeparator]   => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::ArgumentSeparator)};
    [AddressingModifier]  => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::AddressingModifier)};
    [AddressingSeparator] => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::AddressingSeparator)};
    [ImmediateModifier]   => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::ImmediateModifier)};

    // Arithmetics 
    [+] => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Arithmetic(crate::lexer::tokens::Arithmetic::Add))};
    [-] => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Arithmetic(crate::lexer::tokens::Arithmetic::Subtract))};
    [*] => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Arithmetic(crate::lexer::tokens::Arithmetic::Multiply))};
    [/] => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Arithmetic(crate::lexer::tokens::Arithmetic::Divide))};
    [%] => { crate::lexer::tokens::Token::ControlCharacter(crate::lexer::tokens::ControlCharacter::Arithmetic(crate::lexer::tokens::Arithmetic::Modulo))};

    // Trivia 
    [String($s:expr)]     => { crate::lexer::tokens::Token::String(std::string::String::from($s)) };
    [Comment($c:expr)]    => { crate::lexer::tokens::Token::Trivia(crate::lexer::tokens::Trivia::Comment(std::string::String::from($c))) };
    [WhiteSpace($c:expr)] => { crate::lexer::tokens::Token::Trivia(crate::lexer::tokens::Trivia::WhiteSpace(std::string::String::from($c))) };
    [NewLine($c:expr)]    => { crate::lexer::tokens::Token::Trivia(crate::lexer::tokens::Trivia::NewLine(std::string::String::from($c))) };

    // other
    [Other($s:expr)]      => { crate::lexer::tokens::Token::Other(std::string::String::from($s)) };
    [Unknown($s:expr)]    => { crate::lexer::tokens::Token::Unknown(std::string::String::from($s)) };
    [Label($s:expr)]      => { crate::lexer::tokens::Token::Label(std::string::String::from($s)) };

    // address
    [AddressB($s:expr)]   => { crate::lexer::tokens::Token::Address(crate::lexer::tokens::Number::Binary(std::string::String::from($s))) };
    [AddressO($s:expr)]   => { crate::lexer::tokens::Token::Address(crate::lexer::tokens::Number::Octal(std::string::String::from($s))) };
    [AddressD($s:expr)]   => { crate::lexer::tokens::Token::Address(crate::lexer::tokens::Number::Decimal(std::string::String::from($s))) };
    [AddressH($s:expr)]   => { crate::lexer::tokens::Token::Address(crate::lexer::tokens::Number::Hexadecimal(std::string::String::from($s))) };

    // Numbers
    [NumberB($s:expr)]    => { crate::lexer::tokens::Token::Number(crate::lexer::tokens::Number::Binary(std::string::String::from($s))) };
    [NumberO($s:expr)]    => { crate::lexer::tokens::Token::Number(crate::lexer::tokens::Number::Octal(std::string::String::from($s))) };
    [NumberD($s:expr)]    => { crate::lexer::tokens::Token::Number(crate::lexer::tokens::Number::Decimal(std::string::String::from($s))) };
    [NumberH($s:expr)]    => { crate::lexer::tokens::Token::Number(crate::lexer::tokens::Number::Hexadecimal(std::string::String::from($s))) };

}

//#endregion