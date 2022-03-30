extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
pub mod documentation;

use crate::flags::Locale;
use crate::ClientConfiguration;
use documentation::Documentation;
use lazy_static::lazy_static;
use lspower::lsp::{ MarkedString, Position, TextDocumentItem, LanguageString};
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashMap;

use self::documentation::{ValidOperand, PossibleOperand};


pub(crate) fn all_documentation(locale: Locale) -> Option<HashMap<String, Documentation>> {
    let docs = DOCUMENTATION.get(locale.borrow());

    match docs {
        Some(some_docs) => Option::Some(some_docs.clone()),
        None => Option::None,
    }
}


pub(crate) fn syntax(key_docs: (String, Documentation)) -> String {

    let operands = key_docs.1.valid_operands.clone();

    match key_docs.1.valid_operands.len() {
        0 => key_docs.0,
        1 => syntax_for_one_operand(key_docs.0, operands[0].borrow()),
        2 => syntax_for_two_operands(key_docs.0, operands[0].borrow(), operands[1].borrow()),
        3 => syntax_for_three_operands(key_docs.0, operands[0].borrow(), operands[1].borrow(), operands[2].borrow()),
        _ => "".to_string(),
    }
}


fn syntax_for_one_operand(key: String, operands: &Vec<ValidOperand>) -> String {
    let mut result = format!("{} [operand]\n\n", key);

    for operand in operands {
        result.push_str(format!("{} [{}]\n", key, operand.operand().label()).as_str());
        result.push_str(format!("{} {}\n\n", key, operand.operand().example(None)).as_str());
    }    

    result
}

fn syntax_for_two_operands(key: String, operands0: &Vec<ValidOperand>, operands1: &Vec<ValidOperand>) -> String {
    let mut result = format!("{} [operand0], [operand1]\n\n", key);

    for operand0 in operands0.clone() {
        for operand1 in operands1.clone() {
            if operand1.when_first_is() != PossibleOperand::ANY && operand1.when_first_is() != operand0.operand() {
                continue;
            }
            result.push_str(format!("{} [{}], [{}]\n", key, operand0.operand().label(), operand1.operand().label()).as_str());
            result.push_str(format!("{} {}, {}\n\n", key, operand0.operand().example(None), operand1.operand().example(None)).as_str());
        }
    }    

    result
}

fn syntax_for_three_operands(key: String, operands0: &Vec<ValidOperand>, operands1: &Vec<ValidOperand>, operands2: &Vec<ValidOperand>) -> String {
    let mut result = format!("{} [operand0], [operand1], [operand2]\n\n", key);

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
                        "{} [{}], [{}], [{}]\n",
                        key,
                        operand0.operand().label(), 
                        operand1.operand().label(), 
                        operand2.operand().label()
                    ).as_str());

                result.push_str(
                    format!(
                        "{} {}, {}, {}\n\n", 
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

pub(crate) fn generate_affected_flags(flags: Vec<documentation::Flag>) -> String {
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
                result.push_str(documentation::PossibleOperand::from_bits_truncate(operand).label().as_str());
                result.push_str("\n");
            }

            result.push_str("\n\n");
        }
    }

    result
}

#[allow(dead_code)]
/// Finds what user is hovering their cursor over and then tries to match documentation for specified locale
pub(crate) fn documentation(
    position: Position,
    document: &TextDocumentItem,
    configuration: &ClientConfiguration,
) -> Vec<MarkedString> {
    //let _doc = load_documentation!();

    let mnemonic = get_symbol(document, position);

    if mnemonic == "" {
        return Vec::new();
    }

    let locale: Locale;

    if configuration.locale() == Locale::DEFAULT {
        locale = configuration.ui_locale();
    } else {
        locale = configuration.locale();
    }

    let mut documentation_option = get_documentation_data(locale, mnemonic.clone());

    if documentation_option.is_none() && locale != Locale::ENGLISH {
        documentation_option = get_documentation_data(Locale::ENGLISH, mnemonic.clone());
    }

    if documentation_option.is_none() {
        return <Vec<MarkedString>>::new();
    }

    let documentation = match documentation_option {
        None => Documentation::default(),
        Some(d) => d,
    };

    let mut documentation_vector: Vec<MarkedString> = Vec::new();
    let mut tmp: String;

    tmp = String::from("**");
    tmp.push_str(mnemonic.as_str());
    tmp.push_str("**");
    documentation_vector.push(MarkedString::String(tmp));

    // if documentation.detail != "" {
    //     tmp = String::from("**");
    //     tmp.push_str(documentation.detail.as_str());
    //     tmp.push_str("**");
    //     documentation_vector.push(MarkedString::String(tmp));
    // }

    if documentation.description != "" {
        tmp = String::from(documentation.description.clone());
        documentation_vector.push(MarkedString::String(tmp));
    }

    tmp = syntax((mnemonic, documentation.clone()));
    if tmp != "" {
        documentation_vector.push(MarkedString::LanguageString(LanguageString {
            language: "asm8051".to_string(),
            value: tmp.to_string(),
        }));
    }

    tmp = generate_valid_operands(documentation.valid_operands.clone());

    if tmp != "" {
        let header = String::from(match locale {
            Locale::POLISH => "Poprawne operandy:\n\n",
            Locale { .. } => "Valid operands:\n\n",
        });

        documentation_vector.push(MarkedString::String(format!("{}{}", header, tmp)));
    }

    tmp = generate_affected_flags(documentation.affected_flags.clone());
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
fn get_symbol(document: &TextDocumentItem, position: Position) -> String {
    // split text document into individual lines
    let mut lines = document.borrow().text.lines();

    // go to the line over which user is hovering
    let mut line_option: Option<&str> = Option::None;
    for _i in 0..position.line + 1 {
        line_option = lines.next();
    }

    if line_option.is_none() {
        return String::from("");
    }

    // get individual Unicode graphemes as Vec<&str>
    let graphemes =
        UnicodeSegmentation::graphemes(line_option.unwrap(), true)
        .collect::<Vec<&str>>();
    let _chars = line_option.unwrap().chars().collect::<Vec<char>>();
    let chars_length = _chars.len();

    let mut symbol_start_position = 0;
    let mut symbol_end_position = graphemes.len() as u32;

    if position.character != 0 {
        // find beginning of the symbol user is hovering over
        for i in (0..=position.character).rev() {
            let index = i as usize;
            if index < chars_length && !is_valid_character(_chars[index]) {
                symbol_start_position = i + 1;
                break;
            }
        }
    }
    if position.character + 1 < graphemes.len() as u32 {
        // find end of the symbol user is hovering over
        for i in position.character..=(graphemes.len() - 1).try_into().unwrap() {
            if !is_valid_character(_chars[i as usize]) {
                symbol_end_position = i;
                break;
            }
        }
    }

    // using locations of beginning and end of the symbol create a String containing it
    let mut sym: String = String::from("");
    for i in symbol_start_position..symbol_end_position {
        sym.push_str(_chars[i as usize].to_string().as_str());
    }

    sym
}

fn is_valid_character(character: char) -> bool {
    IS_VALID_CHARACTER_REGEX.is_match(character.to_string().as_str())
}

fn get_documentation_data(_locale: Locale, _mnemonic: String) -> Option<Documentation> {
    let docs = DOCUMENTATION.get(_locale.borrow());
    if docs.is_none() {
        return Option::None;
    }

    let doc = docs.unwrap().get(&String::from(_mnemonic));

    if doc.is_none() {
        return Option::None;
    }

    Option::Some(doc.unwrap().clone())
}

lazy_static! {
    /// I don't want to create regex each time I want to use it
    static ref IS_VALID_CHARACTER_REGEX: Regex = Regex::new(r"[a-zA-Z0-9_.#@]").unwrap();//[\p{L}\p{N}_.#@]

    static ref DOCUMENTATION: HashMap<Locale, HashMap<String, Documentation>> = HashMap::from([
        (Locale::ENGLISH, load_documentation::load_documentation!(english)),
        (Locale::POLISH, load_documentation::load_documentation!(polish)),
    ]);
}
