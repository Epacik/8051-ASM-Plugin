//#region Directoves

use super::Register;

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

const INSTRUCTIONS: [&str; 44] = [
    "ACALL", "ADD",  "ADDC", "AJMP", "ANL", "CJNE", "CLR", "CPL", "DA", "DEC", "DIV", "DJNZ", 
    "INC", "JB", "JBC", "JC", "JMP", "JNB", "JNC", "JNZ", "JZ", "LCALL", "LJMP", "MOV", "MOVC", 
    "MOVX", "MUL", "NOP", "ORL", "POP", "PUSH", "RET", "RETI", "RL", "RLC", "RR", "RRC", "SETB", 
    "SJMP", "SUBB", "SWAP", "XCH", "XCHD", "XRL", 
];

pub(super) fn is_instruction<S: AsRef<str>>(s: S) -> bool {
    let s = s.as_ref();
    INSTRUCTIONS.contains(&s)
}
//#endregion

//#region Registers

mod registers {

    fn as_ref_to_vec_chars<S: AsRef<str>>(s: S) -> Vec<char> {
        let s = s.as_ref();
    
        s.chars().collect::<Vec<char>>()
    }

    pub fn is_addressing<S: AsRef<str>>(s: S) -> bool {
        let chars = as_ref_to_vec_chars(s);
        
        chars.len() == 3 && chars[0] == '@' && chars[1] == 'R' && (chars[2] >= '0' || chars[2] <= '1')
    }

    pub fn is_helper<S: AsRef<str>>(s: S) -> bool {
        let chars = as_ref_to_vec_chars(s);
        
        chars.len() == 2 && chars[0] == 'R' && (chars[1] >= '0' || chars[1] <= '1')
    }

    pub fn is_port<S: AsRef<str>>(s: S) -> bool {
        let chars = as_ref_to_vec_chars(s);
        
        chars.len() == 2 && chars[0] == 'P' && (chars[1] >= '0' || chars[1] <= '1')
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
    registers::is_addressing(&s) || 
    registers::is_helper(&s)     || 
    registers::is_port(&s)       || 
    registers::is_special(&s)    ||
    registers::is_main(&s)
}
pub(super) fn string_to_register<S: AsRef<str>>(s: S) -> Register {
    let string = s.as_ref().to_string();

    if registers::is_main(&s) { Register::Main(string) }
    else if registers::is_addressing(&s) { Register::Addressing(string) }
    else if registers::is_helper(&s) { Register::Helper(string) }
    else if registers::is_port(&s) { Register::Port(string) }
    else if registers::is_special(&s) { Register::Special(string) }
    else { panic!("invalid register") }
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