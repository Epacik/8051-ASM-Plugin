//#region imports
use crate::flags::Locale;
use crate::client_configuration::ClientConfiguration;
use lazy_static::lazy_static;
use tower_lsp::lsp_types::{ MarkedString, Position, TextDocumentItem, LanguageString};
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashMap;
//#endregion

pub(crate) fn all_documentation(locale: Locale) -> Option<HashMap<String, Documentation>> {
    let docs = DOCUMENTATION.get(locale.borrow());

    match docs {
        Some(some_docs) => Option::Some(some_docs.clone()),
        None => Option::None,
    }
}

//#region syntax generation
pub(crate) fn syntax(key_docs: (String, Documentation)) -> String {
    let (key, docs) = key_docs;

    if docs.dont_generate_syntax {
        return String::from("");
    }

    let operands = docs.valid_operands.clone();
    
    for op in docs.valid_operands {
        if op.is_empty() {
            return String::from("");
        }
    }

    match operands.len() {
        0 => key,
        1 => syntax_one_operand(key, operands[0].borrow(), docs.prefix),
        2 => syntax_two_operands(key, operands[0].borrow(), operands[1].borrow(), docs.prefix),
        3 => syntax_three_operands(key, operands[0].borrow(), operands[1].borrow(), operands[2].borrow(), docs.prefix),
        _ => "".to_string(),
    }
}

fn syntax_one_operand(key: String, operands: &Vec<ValidOperand>, prefix: String) -> String {
    let mut result = format!("{}{} [operand]\n\n",prefix, key);

    for operand in operands {
        result.push_str(format!("{}{} [{}]\n", prefix, key, operand.operand().label()).as_str());
        result.push_str(format!("{}{} {}\n\n", prefix, key, operand.operand().example(None)).as_str());
    }    

    result
}

fn syntax_two_operands(key: String, operands0: &Vec<ValidOperand>, operands1: &Vec<ValidOperand>, prefix: String) -> String {
    let mut result = format!("{}{} [operand0], [operand1]\n\n", prefix,  key);

    for operand0 in operands0.clone() {
        for operand1 in operands1.clone() {
            if operand1.when_first_is() != PossibleOperand::ANY && operand1.when_first_is() != operand0.operand() {
                continue;
            }
            result.push_str(format!("{}{} [{}], [{}]\n", prefix,  key, operand0.operand().label(), operand1.operand().label()).as_str());
            result.push_str(format!("{}{} {}, {}\n\n", prefix,  key, operand0.operand().example(None), operand1.operand().example(None)).as_str());
        }
    }    

    result
}

fn syntax_three_operands(key: String, operands0: &Vec<ValidOperand>, operands1: &Vec<ValidOperand>, operands2: &Vec<ValidOperand>, prefix: String) -> String {
    let mut result = format!("{}{} [operand0], [operand1], [operand2]\n\n", prefix, key);

    for operand0 in operands0.clone() {
        for operand1 in operands1.clone() {
            if operand1.when_first_is() != PossibleOperand::ANY && operand1.when_first_is() != operand0.operand() {
                continue;
            }
            for operand2 in operands2.clone() {
                if operand2.when_first_is() != PossibleOperand::ANY && operand2.when_first_is() != operand0.operand() {
                    continue;
                }
                result.push_str(
                    format!(
                        "{}{} [{}], [{}], [{}]\n",
                        prefix,
                        key,
                        operand0.operand().label(), 
                        operand1.operand().label(), 
                        operand2.operand().label()
                    ).as_str());

                result.push_str(
                    format!(
                        "{}{} {}, {}, {}\n\n", 
                        prefix,
                        key, 
                        operand0.operand().example(None), 
                        operand1.operand().example(None),
                        operand2.operand().example(None)
                    ).as_str());
            }
        }
    }    

    result
}

pub(crate) fn generate_affected_flags(flags: Vec<Flag>) -> String {
    let mut result = String::new();

    for flag in flags {
        result.push_str("- **");
        result.push_str(flag.flag().label().as_str());
        result.push_str("**: ");

        if !flag.when_set.is_empty() {
            result.push_str("set when ");
            result.push_str(flag.when_set.as_str());
        }

        if !flag.when_set.is_empty() && !flag.when_unset.is_empty() {
            result.push_str(", ");
        }

        if !flag.when_unset.is_empty() {
            result.push_str("unset when ");
            result.push_str(flag.when_set.as_str());
        }

        result.push_str("\n");
    }

    result
}

pub(crate) fn generate_valid_operands(operands: Vec<Vec<ValidOperand>>) -> String {
    
    if operands.len() == 0 {
        return String::new();
    }
    let mut result = String::new();

    if operands.len() == 1 {
        for operand in operands[0].clone() {
            result.push_str(" - ");
            result.push_str(operand.operand().label().as_str());
            result.push_str("\n");
        }
    }
    else {
        let mut filtered : Vec<Vec<i32>> = Vec::new();
        for i in 0..operands.len() {
            let inner = &operands[i]; 
            filtered.push(Vec::new());
            for operand in inner {
                if !filtered[i].contains(&operand.operand) {
                    filtered[i].push(operand.operand);
                }
            }
        }

        for i in 0..filtered.len() {

            result.push_str("**Operand");
            result.push_str(i.to_string().as_str());
            result.push_str("**: \n");

            for operand in filtered[i].clone() {
                result.push_str(" - ");
                result.push_str(PossibleOperand::from_bits_truncate(operand).label().as_str());
                result.push_str("\n");
            }

            result.push_str("\n\n");
        }
    }

    result
}
//#endregion syntax generation

#[allow(dead_code)]
/// Finds what user is hovering their cursor over and then tries to match documentation for specified locale
pub(crate) fn documentation(
    position: Position,
    document: &TextDocumentItem,
    _configuration: &ClientConfiguration,
    locale: Locale
) -> Vec<MarkedString> {

    let symbol = get_symbol(document, position);

    match symbol {
        Symbol::None => Vec::new(),
        Symbol::Number(number) => documentation_number(number, locale),
        Symbol::Label(label, pos) => documentation_label(label, pos, document.borrow()),
        Symbol::PredefinedSymbol(mnemonic) => documentation_predefined(mnemonic, locale),
        Symbol::Constant(_, _) => Vec::new(),
        Symbol::Macro(_, _) => Vec::new(),
    }
}

fn documentation_label(label: String, pos: u32, document: &TextDocumentItem) -> Vec<MarkedString> {
    let lines = document.text.lines();
    let lines = lines.into_iter().collect::<Vec<&str>>();
    let prev_text = lines[(pos - 1) as usize];
    if pos == 0 || !prev_text.trim().starts_with(";") {
        return Vec::new();
    }

    let mut comment_start: u32 = pos;
    for i in (0..pos).rev() {
        if !lines[i as usize].trim().starts_with(";") {
            comment_start = i + 1;
            break;
        }
    }

    if comment_start == pos {
        return Vec::new();
    }

    let lines = &lines[(comment_start as usize)..(pos as usize)];
    
    let mut documentation_vector: Vec<MarkedString> = Vec::new();

    let tmp = format!("**{}**", label.as_str());
    documentation_vector.push(MarkedString::String(tmp));

    let mut tmp = String::new();
    for line in lines {
        tmp.push_str(&line.trim()[1..]);
        tmp.push_str("\n\n");
    }
    documentation_vector.push(MarkedString::String(tmp));

    documentation_vector
}

fn documentation_number(number: String, locale: Locale) -> Vec<MarkedString> {
    let labels = match locale {
        Locale::POLISH => ("Binarnie", "Dziesiętnie", "Szesnastkowo"),
        _              => ("Binary",   "Decimal",     "Hexadecimal"),
    };

    let parse_result: Result<i32, std::num::ParseIntError>;

    if number.ends_with("b") || number.ends_with("B") {
        let n = &number[1..(number.len() - 1)];
        parse_result = i32::from_str_radix(n, 2);
    }
    else if number.ends_with("h") || number.ends_with("H") {
        let n = &number[1..(number.len() - 1)];
        parse_result = i32::from_str_radix(n, 16);
    }
    else {
        let n = &number[1..];
        parse_result = i32::from_str_radix(n, 10);
    }

    let value = match parse_result {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let string = MarkedString::String(
        format!("{}: #{:b}b\n\n{}: #{}\n\n{}: #{:X}h",
         labels.0, value,
         labels.1, value,
         labels.2, value));
    vec![string]

}

fn documentation_predefined(mnemonic: String, locale: Locale) -> Vec<MarkedString> {
    
    let mut documentation_option = get_documentation(locale, mnemonic.clone());

    if documentation_option.is_none() && locale != Locale::ENGLISH {
        documentation_option = get_documentation(Locale::ENGLISH, mnemonic.clone());
    }

    if documentation_option.is_none() {
        return <Vec<MarkedString>>::new();
    }

    let documentation = match documentation_option {
        None => Documentation::default(),
        Some(d) => d,
    };

    let mut documentation_vector: Vec<MarkedString> = Vec::new();

    let tmp = format!("**{}**", mnemonic.as_str());
    documentation_vector.push(MarkedString::String(tmp));

    // if documentation.detail != "" {
    //     tmp = String::from("**");
    //     tmp.push_str(documentation.detail.as_str());
    //     tmp.push_str("**");
    //     documentation_vector.push(MarkedString::String(tmp));
    // }

    if documentation.description != "" {
        let tmp = String::from(documentation.description.clone());
        documentation_vector.push(MarkedString::String(tmp));
    }

    let tmp = syntax((mnemonic, documentation.clone()));
    if tmp != "" {
        documentation_vector.push(MarkedString::LanguageString(LanguageString {
            language: crate::LANG_ID.to_string(),
            value: tmp.to_string(),
        }));
    }

    let tmp = generate_valid_operands(documentation.valid_operands.clone());

    if tmp != "" {
        let header = String::from(match locale {
            Locale::POLISH => "Poprawne operandy:\n\n",
            Locale { .. } => "Valid operands:\n\n",
        });

        documentation_vector.push(MarkedString::String(format!("{}{}", header, tmp)));
    }

    let tmp = generate_affected_flags(documentation.affected_flags.clone());
    if tmp != "" {
        let header = String::from(match locale {
            Locale::POLISH => "Zmodyfikowane flagi:\n\n",
            Locale { .. } => "Affected flags:\n\n",
        });
        
        documentation_vector.push(MarkedString::String(format!("{}{}", header, tmp)));
    }

    documentation_vector
}

/// Get the symbol/mnemonic/word over which user is hovering
fn get_symbol(document: &TextDocumentItem, position: Position) -> Symbol {
    // split text document into individual lines
    let mut lines = document.borrow().text.lines();

    // go to the line over which user is hovering
    let line_option = lines.nth(position.line as usize);

    if line_option.is_none() {
        return Symbol::None;
    }
    let chars = line_option.unwrap().chars().collect::<Vec<char>>();
    let chars_length = chars.len();

    if position.character >= (chars_length as u32) {
        return Symbol::None;
    }

    // Check if we're not in the comment, and return if we are
    for i in 0..=position.character {
        if chars[i as usize] == ';' {
            return Symbol::None;
        }
    }

    let mut symbol_start_position = 0;
    let mut symbol_end_position = chars_length as u32;

    // find beginning of the symbol user is hovering over
    for i in (0..=position.character).rev() {
        if !is_valid_character(chars[i as usize]) {
            symbol_start_position = i + 1;
            break;
        }
    }
    
    // find end of the symbol user is hovering over
    for i in position.character..(chars_length as u32) {
        if !is_valid_character(chars[i as usize]) {
            symbol_end_position = i;
            break;
        }
    }

    if symbol_start_position >= symbol_end_position {
        return Symbol::None;
    }

    // using locations of beginning and end of the symbol create a String containing it
    let chars = chars[(symbol_start_position as usize)..(symbol_end_position as usize)].borrow();

    let symbol_text = String::from_iter(chars);
    if DOCUMENTATION[&Locale::ENGLISH].contains_key(&symbol_text) {
        return Symbol::PredefinedSymbol(symbol_text);
    }
    else if symbol_text.starts_with("#") {
        return Symbol::Number(symbol_text);
    }
    
    let sym = is_symbol_constant(&symbol_text, document.borrow());
    if sym.0 {
        return Symbol::Constant(symbol_text, sym.1);
    }
    
    let sym = is_symbol_macro(&symbol_text, document.borrow());
    if sym.0 {
        return Symbol::Macro(symbol_text, sym.1);
    }

    let sym = is_symbol_label(&symbol_text, document.borrow());
    if sym.0 {
        return Symbol::Label(symbol_text, sym.1);
    }
    

    Symbol::None

}

fn is_symbol_macro(symbol_text: &str, document: &TextDocumentItem) -> (bool, u32) {
    let lines = document.text.lines();
    let mut line_number: u32 = 0;
    for line in lines {
        if line.starts_with(symbol_text) && 
           str_contains_any(
                line.borrow(), 
                &[ "MACRO" ], 
                false) {
            return (true, line_number);
        }
        line_number = line_number + 1;
    }

    (false, 0)
}

fn is_symbol_label(symbol_text: &str, document: &TextDocumentItem) -> (bool, u32) {
    let lines = document.text.lines();
    let mut line_number: u32 = 0;
    for line in lines {
        if line.starts_with(symbol_text) && 
           !str_contains_any(
                line.borrow(), 
                &[ "EQU", "SET", "DB", "DW", "REG", "BIT", "MACRO" ], 
                false) {
            return (true, line_number);
        }
        line_number = line_number + 1;
    }

    (false, 0)
}

fn is_symbol_constant(symbol_text: &str, document: &TextDocumentItem) -> (bool, u32) {
    let lines = document.text.lines();
    let mut line_number: u32 = 0;
    for line in lines {
        if line.starts_with(symbol_text) && 
           str_contains_any(
                line.borrow(), 
                &[ "EQU", "SET", "DB", "DW", "REG", "BIT" ], 
                false) {
            return (true, line_number);
        }
        line_number = line_number + 1;
    }

    (false, 0)
}

fn str_contains_any(string: &str, contains: &[&str], case_sensitive: bool) -> bool {
    let t = match case_sensitive {
        false => string.to_lowercase(),
        true  => string.to_string(),
    };
    for cont in contains {
        let c = match case_sensitive {
            false => cont.to_lowercase(),
            true  => cont.to_string(),
        };

        if t.contains(&c) {
            return true;
        }
    }
    
    false
}

fn is_valid_character(character: char) -> bool {
    let text = character.to_string();
    IS_VALID_CHARACTER_REGEX.is_match(text.as_str())
}

fn get_documentation(_locale: Locale, _mnemonic: String) -> Option<Documentation> {
    let docs = match DOCUMENTATION.get(_locale.borrow()){
        Some(doc) => doc,
        None => return None,
    };

    let doc = match docs.get(&String::from(_mnemonic)){
        Some(d) => d,
        None => return None,
    };

    Option::Some(doc.clone())
}

lazy_static! {
    /// I don't want to create regex each time I want to use it
    static ref IS_VALID_CHARACTER_REGEX: Regex = Regex::new(r"[a-zA-Z0-9_.#@]").unwrap();//[\p{L}\p{N}_.#@]

    static ref DOCUMENTATION: HashMap<Locale, HashMap<String, Documentation>> = HashMap::from([
        (Locale::ENGLISH, load_documentation::load_documentation!(english)),
        (Locale::POLISH, load_documentation::load_documentation!(polish)),
    ]);
}
use serde::{Deserialize, Serialize};
use bitflags::bitflags;

pub enum Symbol {
    None,
    Number(String),
    Label(String, u32),
    PredefinedSymbol(String),
    Constant(String, u32),
    Macro(String, u32),
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Documentation {
    pub detail: std::string::String,
    pub description: std::string::String,
    pub valid_operands: std::vec::Vec<std::vec::Vec<ValidOperand>>,
    pub affected_flags: std::vec::Vec<Flag>,
    pub dont_generate_syntax: bool,
    pub dont_duplicate_in_all_docs: bool,
    pub full_key: std::string::String,
    pub category: String,
    pub prefix: String,
    pub prefix_required: bool,
}

impl Documentation {
    #[allow(dead_code)]
    pub fn new (
        detail: &str, 
        description: &str, 
        valid_operands: Vec<Vec<ValidOperand>>, 
        affected_flags: Vec<Flag>,
        dont_generate_syntax: bool,
        dont_duplicate_in_all_docs: bool,
        full_key: &str,
        category: &str,
        prefix: &str,
        prefix_required: bool) -> Documentation {

        Documentation { 
            detail: String::from(detail), 
            description: String::from(description), 
            valid_operands, 
            affected_flags, 
            dont_generate_syntax, 
            category: String::from(category) ,
            dont_duplicate_in_all_docs,
            full_key: String::from(full_key),
            prefix: String::from(prefix),
            prefix_required
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Flag {
    pub flag: i32,
    pub when_set: std::string::String,
    pub when_unset: std::string::String,
}

#[allow(dead_code)]
impl Flag {
    pub fn flag(&self) -> FlagType {
        FlagType::from_bits_truncate(self.flag)
    }

    pub fn new(flag: FlagType) -> Flag {
        Flag {
            flag: flag.bits,
            when_set: String::from(""),
            when_unset: String::from(""),
        }
    }

    pub fn new_with_messages(flag: FlagType, when_set: &str, when_unset: &str) -> Flag {
        Flag {
            flag: flag.bits,
            when_set: String::from(when_set),
            when_unset: String::from(when_unset),
        }
    }

    pub fn from_i32(flag: i32) -> Flag {
        Flag {
            flag,
            when_set: String::from(""),
            when_unset: String::from(""),
        }
    }

    pub fn from_i32_with_messages(flag: i32, when_set: &str, when_unset: &str) -> Flag {
        Flag {
            flag,
            when_set: String::from(when_set),
            when_unset: String::from(when_unset),
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct ValidOperand {
    pub operand: i32,
    pub when_first_is: i32,
}

#[allow(dead_code)]
impl ValidOperand {
    pub fn operand(&self) -> PossibleOperand {
        PossibleOperand::from_bits_truncate(self.operand)
    }

    pub fn when_first_is(&self) -> PossibleOperand {
        PossibleOperand::from_bits_truncate(self.when_first_is)
    }

    pub fn new(operand: PossibleOperand, when_first_is: Option<PossibleOperand>) -> ValidOperand {
        ValidOperand {
            operand: operand.bits,
            when_first_is: when_first_is.unwrap_or(PossibleOperand::ANY).bits,
        }
    } 

    pub fn from_i32(operand: i32, when_first_is: Option<i32>) -> ValidOperand {
        let when_first_is = when_first_is.unwrap_or(0);
        let max = PossibleOperand::all().bits();
        let min = PossibleOperand::empty().bits();

        if operand > max || operand < min { panic!("operand was out of range") }
        if when_first_is > max || when_first_is < min { panic!("when_first_is was out of range") }

        ValidOperand { operand, when_first_is }
    }

    pub fn equals(&self, other: &ValidOperand) -> bool {
        self.operand == other.operand && self.when_first_is == other.when_first_is
    }

} 

bitflags! {
    #[derive(serde::Deserialize, Default)]
    pub struct FlagType: i32 {
        const PARITY                = 0;
        const USER_DEFINED          = 1;
        const OVERFLOW              = 2;
        const REGISTER_BANK_SELECT0 = 3;
        const REGISTER_BANK_SELECT1 = 4;
        const FLAG0                 = 5;
        const AUXILIARY_CARRY       = 6;
        const CARRY                 = 7;
    }

    #[derive(serde::Deserialize, Default)]
    pub struct PossibleOperand: i32 {
        const ANY                              = 0;
        const CODE_ADDRESS                     = 1;
        const LABEL                            = 2;
        const DATA                             = 3;
        const DATA16                           = 4;
        const INTERNAL_RAM_ADDRESS             = 5;
        const ADDRESS_IN_R0_OR_R1              = 6;
        const REGISTERS_RX                     = 7;
        const CARRY_FLAG                       = 8;
        const BIT_ADDRESS                      = 9;
        const NEGATED_BIT_ADDRESS              = 10;
        const RELATIVE_ADDRESS                 = 11;
        const ACCUMULATOR                      = 12;
        const ACCUMULATOR_AND_B                = 13;
        const ADDRESS_IN_ACCUMULATOR_PLUS_DPTR = 14;
        const DPTR                             = 15;
        const ADDRESS_IN_DPTR                  = 16;
        const ADDRESS_IN_ACCUMULATOR_PLUS_PC   = 17;

        const ABSOLUTE_ADDRESS                 = 18;

        const HEX_NUMBER                       = 100;
        const BINARY_NUMBER                    = 101;
        const DECIMAL_NUMBER                   = 102;
        const ASCII_CHARACTERS                 = 103;

        
    }
}

#[allow(dead_code)]
impl FlagType {
    pub fn label(&self) -> String {
        match self.bits {
            0 => "Patity".to_string(),
            1 => "User defined".to_string(),
            2 => "Overflow".to_string(),
            3 => "Register Bank Select 0".to_string(),
            4 => "Register Bank Select 1".to_string(),
            5 => "Flag 0".to_string(),
            6 => "Auxiliary Carry".to_string(),
            7 => crate::localize!("flag-carry"),
            _ => "Unknown".to_string()
        }
    }
}

impl PossibleOperand {
    pub fn label(&self) -> String {
        (match self.bits {
            0 => "any",
            1 => "code address",
            2 => "label",
            3 => "byte",
            4 => "two bytes",
            5 => "internal RAM address",
            6 => "@R0 or @R1",
            7 => "R0 trough R7",
            8 => "carry flag",
            9 => "bit address",
            10 => "address of negated bit",
            11 => "relative address",
            12 => "the Accumulator",
            13 => "the Accumulator an B register",
            14 => "address in the Accumulator + DPTR",
            15 => "DPTR",
            16 => "address in DPTR",
            17 => "address in the Accumulator + PC",
            18 => "Absolute address",

            100 => "Hexadecimal number",
            101 => "Binary number",
            102 => "Decimal number",
            103 => "Ascii characters",
            _ => "Unknown"
        }).to_string()
    }

    pub fn example(&self, i: Option<i32>) -> String {
        let r_address = format!("@R{}", i.unwrap_or(0));
        let r = format!("R{}", i.unwrap_or(0));
        (match self.bits {
            1 => "23H",
            2 => "LABEL",
            3 => "#32H",
            4 => "#5C6H",
            5 => "23H",
            6 => r_address.as_str(),
            7 => r.as_str(),
            8 => "C",
            9 => "23H",
            10 => "/23H",
            11 => "23H",
            12 => "A",
            13 => "AB",
            14 => "@A+DPTR",
            15 => "DPTR",
            16 => "@DPTR",
            17 => "@A+PC",
            18 => "100h",

            100 => "56h",
            101 => "010101011b",
            102 => "63",
            103 => "'Lorem ipsum'",
            _ => ""
        }).to_string()
    }
}

// #region tests
#[cfg(test)]
mod tests {
    mod all_documentation {
        use test_case::test_case;
        use crate::{flags::Locale, hover::all_documentation};

        #[test_case(Locale::POLISH,  true  ; "some for POLISH locale")]
        #[test_case(Locale::ENGLISH, true  ; "some for ENGLISH locale")]
        #[test_case(Locale::DEFAULT, false ; "none for DEFAULT locale")]
        fn is(locale: Locale, is_some: bool) {
            let docs = all_documentation(locale);
            assert_eq!(docs.is_some(), is_some);
        }

        #[test_case(Locale::POLISH  ; "polish locale")]
        #[test_case(Locale::ENGLISH ; "english locale")]
        fn is_not_empty_for(locale: Locale) {
            let docs = all_documentation(locale).unwrap();
            assert!(docs.len() > 0);
        }
    }

    mod generated_syntax {
        mod with_syntax_function {
            use test_case::test_case;

            use crate::hover::{Documentation, ValidOperand, Flag, syntax};
            
            fn empty_valid_operands() -> Vec<Vec<ValidOperand>> {
                Vec::<Vec::<ValidOperand>>::new()
            }
            fn empty_affected_flags() -> Vec<Flag> {
                Vec::<Flag>::new()
            }
    
            #[test_case(
                "TEST",
                Documentation::new(
                    "", 
                    "some description", 
                    Vec::<Vec::<ValidOperand>>::new(), 
                    Vec::<Flag>::new(), 
                    true, 
                    false,
                    "TEST",
                    "category", 
                    "", 
                    false) ; "dont_generate_syntax is set")]
            #[test_case("", Documentation::new("", "", empty_valid_operands(), empty_affected_flags(), false, false, "", "", "", false) ; "no data is provided") ]
            #[test_case("TEST", Documentation::new(
                "some detail",
                "some description",
                vec!(Vec::<ValidOperand>::new()),
                empty_affected_flags(),
                false,
                false,
                "TEST",
                "category", 
                "", 
                false) ; "any of the inner vectors of valid operands is empty")]
            #[test_case("TEST", Documentation::new(
                "some detail",
                "some description",
                vec!(
                    vec!(ValidOperand::from_i32(1, None)),
                    vec!(ValidOperand::from_i32(1, None)), 
                    vec!(ValidOperand::from_i32(1, None)), 
                    vec!(ValidOperand::from_i32(1, None))
                ),
                empty_affected_flags(),
                false,
                false,
                "TEST",
                "category", 
                "", 
                false) ; "lenght of valid_operands vector is greater than 3")]
            fn is_empty_when(mnemonic: &str, doc: Documentation) {
                let mnemonic = String::from(mnemonic);
                let syntax = syntax((mnemonic, doc));
                assert!(syntax.is_empty());
            }

            // #[test_case(
            //     Documentation::new(
            //         "some detail",
            //         "some quick desctiption",
            //         vec!(vec!(ValidOperand::from_i32(1, None), ValidOperand::from_i32(2, None))),
            //         vec!(Fla)
            //     ))]
            // fn is_not_empty_when_some_data_is_provided_and_dont_generate_syntax_is_not_set(doc: Documentation) {
            //     let syntax = syntax((String::from("TEST"), doc));
            // }
        }
        
    }

}
#[cfg(test)]
mod test_all_documentation {
    use crate::{flags::Locale, hover::all_documentation};

    #[test]
    fn while_locale_is_polish() {
        let docs = all_documentation(Locale::POLISH);
        assert!(docs.is_some(), "Docs are Option::None");

        let docs = docs.unwrap();
        assert!(!docs.is_empty(), "Docs are a HashMap, but that HashMap is empty");
    }

    #[test]
    fn while_locale_is_english() {
        let docs = all_documentation(Locale::ENGLISH);
        assert!(docs.is_some(), "Docs are Option::None");

        let docs = docs.unwrap();
        assert!(!docs.is_empty(), "Docs are a HashMap, but that HashMap is empty");
    }

    #[test]
    fn while_locale_is_default() {
        let docs = all_documentation(Locale::DEFAULT);
        assert!(docs.is_none(), "Docs are not Option::None");
    }

}

#[cfg(test)]
mod test_generating_syntax {
    //use std::borrow::Borrow;

    use test_case::test_case;

    use super::{syntax_one_operand, ValidOperand};

    fn test_mnemonic() -> String {
        String::from("TEST")
    }

    #[test_case(Vec::<ValidOperand>::new() ; "with zero valid operands")]
    #[test_case(vec!(ValidOperand::from_i32(12, None)) ; "with one valid operands")]
    #[test_case(vec!(ValidOperand::from_i32(12, None), ValidOperand::from_i32(3, None)) ; "with two valid operands")]
    #[test_case(vec!(ValidOperand::from_i32(12, None), ValidOperand::from_i32(3, None), ValidOperand::from_i32(5, None)) ;
        "with three valid operands")]
    fn for_one_operand(operands: Vec::<ValidOperand>){
        let syntax = syntax_one_operand(test_mnemonic(), &operands, String::from(""));

        let syntax_header = format!("{} [operand]", test_mnemonic());
        assert!(syntax.starts_with(&syntax_header));

        for operand in operands {
            let section_for_operand = format!("{} [{}]\n{} {}",
            test_mnemonic(), operand.operand().label(), test_mnemonic(), operand.operand().example(Some(1)));
             assert!(syntax.contains(&section_for_operand));
        }
    }
    
    // fn for_two_operands(operands0: Vec::<ValidOperand>, operands1: Vec::<ValidOperand>) {
    //     let syntax = crate::hover::syntax_two_operands(test_mnemonic(), &operands0, &operands1, String::from(""));
        
    //     let syntax_header = format!("{} [operand]", test_mnemonic());
    //     assert!(syntax.starts_with(&syntax_header));

    //     for operand in operands0 {
    //         for operand1 in operands1 {

    //         }
    //     }
    // }
}
//#endregion tests