//#region Directoves

use std::collections::HashMap;

use crate::lexer::tokens::Directive;

use super::tokens::{Register, Instruction, MainRegister, SpecialRegister, HelperRegister, PortRegister};

const DIRECTIVES: [&str; 16] = [
    "BIT", "DB", "DW", "IF", "ELSEIF", "ENDIF",
    "END", "ENDM", "EQU", "$INCDIR", "$INCLUDE", 
    "MACRO", "MACEND", "ORG", "REG", "SET",
];

pub(super) fn is_directive<S: AsRef<str>>(s: S) -> bool {
    let s = s.as_ref();
    DIRECTIVES.contains(&s)
}
//#endregion

//#region Instructions

const INSTRUCTIONS: [&str; 45] = [
    "ACALL", "ADD",   "ADDC", "AJMP", "ANL",
    "CJNE",  "CLR",   "CPL",  "DA",   "DEC",
    "DIV",   "DJNZ",  "INC",  "JB",   "JBC",
    "JC",    "JMP",   "JNB",  "JNC",  "JNZ",
    "JZ",    "LCALL", "LJMP", "MOV",  "MOVC",
    "MOVX",  "MUL",   "NOP",  "ORL",  "POP",
    "PUSH",  "RET",   "RETI", "RL",   "RLC",
    "RR",    "RRC",   "SETB", "SJMP", "SUBB",
    "SWAP",  "XCH",   "XCHD", "XRL",  "CALL",
];

pub(super) fn is_instruction<S: AsRef<str>>(s: S) -> bool {
    let s = s.as_ref();
    INSTRUCTIONS.contains(&s)
}
//#endregion

//#region Registers

fn as_ref_to_vec_chars<S: AsRef<str>>(s: S) -> Vec<char> {
    let s = s.as_ref();

    s.chars().collect::<Vec<char>>()
}

mod registers {
    use super::as_ref_to_vec_chars;

    // pub fn is_addressing<S: AsRef<str>>(s: S) -> bool {
    //     let chars = as_ref_to_vec_chars(s);
        
    //     chars.len() == 3 && chars[0] == '@' && chars[1] == 'R' && (chars[2] >= '0' || chars[2] <= '1')
    // }

    pub fn is_helper<S: AsRef<str>>(s: S) -> bool {
        let chars = as_ref_to_vec_chars(s);
        
        chars.len() == 2 && chars[0] == 'R' && (chars[1] >= '0' || chars[1] <= '1')
    }

    pub fn is_port<S: AsRef<str>>(s: S) -> bool {
        let chars = as_ref_to_vec_chars(s);
        
        chars.len() == 2 && chars[0] == 'P' && (chars[1] >= '0' || chars[1] <= '3')
    }

    pub fn is_main<S: AsRef<str>>(s: S) -> bool {
        let s = s.as_ref();

        s == "A" || s == "B" || s == "AB"
    }

    pub fn is_special<S: AsRef<str>>(s: S) -> bool {
        let s = s.as_ref();
        let chars = as_ref_to_vec_chars(s);

        let is_timer = chars.len() == 2 
        && chars[0] == 'T' 
        && (chars[1_usize] == 'H' || chars[1_usize] == 'L') 
        && chars[2_usize] >= '0' && chars[2_usize] <= '1';

        s == "DPH"  || s == "DPL" || s == "DPTR" ||
        s == "IE"   || s == "IP"  || s == "PC"   ||
        s == "PCON" || s == "PSW" || s == "SBUF" ||
        s == "SCON" || s == "SP"  || is_timer
    }
}

pub(super) fn is_register<S: AsRef<str>>(s: S) -> bool {
    //registers::is_addressing(&s) ||
    registers::is_helper(&s)     ||
    registers::is_port(&s)       ||
    registers::is_special(&s)    ||
    registers::is_main(&s)
}
pub(super) fn string_to_register<S: AsRef<str>>(s: S) -> Option<Register> {

    if is_register(&s){
        let reg = if registers::is_main(&s) { Register::Main(str_to_main_register(&s)) }
        //else if registers::is_addressing(&s) { Register::Addressing(str_to_addressing_register(&s)) }
        else if registers::is_helper(&s) { Register::Helper(str_to_helper_register(&s)) }
        else if registers::is_port(&s) { Register::Port(str_to_port_register(&s)) }
        else { Register::Special(str_to_special_register(&s)) };

        Some(reg)
    }
    else {
        None
    }
}

// fn str_to_addressing_register<S: AsRef<str>>(s: &S)-> AddressingRegister {
//     let chars = as_ref_to_vec_chars(&s);

//     if chars[2] == '0' { AddressingRegister::R0 } else { AddressingRegister::R1 }
// }

fn str_to_port_register<S: AsRef<str>>(s: &S) -> PortRegister {
    let chars = as_ref_to_vec_chars(&s);

    match chars[1] {
        '0' => PortRegister::P0,
        '1' => PortRegister::P1,
        '2' => PortRegister::P2,
        _   => PortRegister::P3,
    }
}

fn str_to_helper_register<S: AsRef<str>>(s: &S) -> HelperRegister {
    let chars = as_ref_to_vec_chars(&s);

    match chars[1] {
        '0' => HelperRegister::R0,
        '1' => HelperRegister::R1,
        '2' => HelperRegister::R2,
        '3' => HelperRegister::R3,
        '4' => HelperRegister::R4,
        '5' => HelperRegister::R5,
        '6' => HelperRegister::R6,
        _   => HelperRegister::R7,
    }
}

fn str_to_special_register<S: AsRef<str>>(s: &S) -> SpecialRegister {
    match s.as_ref() {
        "TL0"  => SpecialRegister::TL0,
        "TH0"  => SpecialRegister::TH0,
        "TL1"  => SpecialRegister::TL1,
        "TH1"  => SpecialRegister::TH1,
        "DPL"  => SpecialRegister::DPL,
        "DPH"  => SpecialRegister::DPH,
        "DPTR" => SpecialRegister::DPTR,
        "IE"   => SpecialRegister::IE,
        "IP"   => SpecialRegister::IP,
        "PC"   => SpecialRegister::PC,
        "PCON" => SpecialRegister::PCON,
        "PSW"  => SpecialRegister::PSW,
        "SBUF" => SpecialRegister::SBUF,
        "SCON" => SpecialRegister::SCON,
        _      => SpecialRegister::SP,
    }
}

fn str_to_main_register<S: AsRef<str>>(s: &S) -> MainRegister {
    match s.as_ref() {
        "A" => MainRegister::A,
        "B" => MainRegister::B,
        _   => MainRegister::AB,
    }
}
//#endregion

const PSW_FLAGS: [&str; 15] = [
    "P", "PSW.0",
    "PSW.1",
    "OV", "PSW.2",
    "RS0", "PSW.3",
    "RS1", "PSW.4",
    "F0", "PSW.5",
    "AC", "PSW.6",
    "CY", "PSW.7",
];

const ADDRESSABLE_BITS: [&str; 8] = [
    "TF0", "TF1",
    "TR0", "TR1",
    "IE0", "IE1",
    "IT0", "IT1",
];

pub(super) fn is_flag_or_bit<S: AsRef<str>>(s: S) -> bool { 
    let s = s.as_ref();

    PSW_FLAGS.contains(&s) || ADDRESSABLE_BITS.contains(&s)
}

pub(super) fn is_keyword<S: AsRef<str>>(s: S) -> bool {
    is_register(&s) || is_instruction(&s) || is_directive(&s) || is_flag_or_bit(&s)
}

lazy_static::lazy_static! {
    static ref INSTRUCTION_MAP: HashMap<&'static str, Instruction> = HashMap::from([
        ("CALL",  Instruction::CALL),
        ("ACALL", Instruction::ACALL),
        ("ADD",   Instruction::ADD),
        ("ADDC",  Instruction::ADDC),
        ("AJMP",  Instruction::AJMP),
        ("ANL",   Instruction::ANL),
        ("CJNE",  Instruction::CJNE),
        ("CLR",   Instruction::CLR),
        ("CPL",   Instruction::CPL),
        ("DA",    Instruction::DA),
        ("DEC",   Instruction::DEC),
        ("DIV",   Instruction::DIV),
        ("DJNZ",  Instruction::DJNZ),
        ("INC",   Instruction::INC),
        ("JB",    Instruction::JB),
        ("JBC",   Instruction::JBC),
        ("JC",    Instruction::JC),
        ("JMP",   Instruction::JMP),
        ("JNB",   Instruction::JNB),
        ("JNC",   Instruction::JNC),
        ("JNZ",   Instruction::JNZ),
        ("JZ",    Instruction::JZ),
        ("LCALL", Instruction::LCALL),
        ("LJMP",  Instruction::LJMP),
        ("MOV",   Instruction::MOV),
        ("MOVC",  Instruction::MOVC),
        ("MOVX",  Instruction::MOVX),
        ("MUL",   Instruction::MUL),
        ("NOP",   Instruction::NOP),
        ("ORL",   Instruction::ORL),
        ("POP",   Instruction::POP),
        ("PUSH",  Instruction::PUSH),
        ("RET",   Instruction::RET),
        ("RETI",  Instruction::RETI),
        ("RL",    Instruction::RL),
        ("RLC",   Instruction::RLC),
        ("RR",    Instruction::RR),
        ("RRC",   Instruction::RRC),
        ("SETB",  Instruction::SETB),
        ("SJMP",  Instruction::SJMP),
        ("SUBB",  Instruction::SUBB),
        ("SWAP",  Instruction::SWAP),
        ("XCH",   Instruction::XCH),
        ("XCHD",  Instruction::XCHD),
        ("XRL",   Instruction::XRL),
    ]);

    static ref DIRECTIVE_MAP: HashMap<&'static str, Directive> = HashMap::from([
        ("BIT",     Directive::BIT),
        ("DB",      Directive::DB),
        ("DW",      Directive::DW),
        ("IF",      Directive::IF),
        ("ELSEIF",  Directive::ELSEIF),
        ("ENDIF",   Directive::ENDIF),
        ("END",     Directive::END),
        ("ENDM",    Directive::ENDM),
        ("EQU",     Directive::EQU),
        ("INCDIR",  Directive::INCDIR),
        ("INCLUDE", Directive::INCLUDE),
        ("MACRO",   Directive::MACRO),
        ("MACEND",  Directive::MACEND),
        ("ORG",     Directive::ORG),
        ("REG",     Directive::REG),
        ("SET",     Directive::SET),
    ]);
}
pub(crate) fn string_to_instruction<S: AsRef<str>>(s: S) -> Option<super::tokens::Instruction> {
    let string = s.as_ref();
    if INSTRUCTION_MAP.contains_key(&string) {
        Some(INSTRUCTION_MAP.get(&string).unwrap().clone())
    }
    else {
        None
    }
}

pub(crate) fn string_to_directive<S: AsRef<str>>(s: S) -> Option<super::tokens::Directive> {
    let string = s.as_ref();
    if DIRECTIVE_MAP.contains_key(&string) {
        Some(DIRECTIVE_MAP.get(&string).unwrap().clone())
    }
    else {
        None
    }
}