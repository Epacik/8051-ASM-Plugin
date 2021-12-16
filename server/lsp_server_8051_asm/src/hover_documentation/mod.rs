extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
mod arithmetic;

use std::borrow::Borrow;
use lspower::lsp::{LanguageString, MarkedString, Position, TextDocumentItem};
use crate::{ClientConfiguration};
use crate::types::{Documentation};
use lazy_static::lazy_static;
use regex::Regex;
use crate::flags::Locale;


#[allow(dead_code)]
/// Finds what user is hovering their cursor over and then tries to match documentation for specified locale
pub(crate) fn get_documentation(position: Position, document: &TextDocumentItem, configuration: &ClientConfiguration) -> Vec<MarkedString> {

    //let _doc = load_documentation!();

    let symbol = get_symbol(document, position);
    
    let locale: Locale;

    if configuration.locale() == Locale::DEFAULT {
        locale = configuration.ui_locale();
    }
    else {
        locale = configuration.locale();
    }

    let documentation = match locale {
        Locale::POLISH => get_pol_documentation(symbol),
        Locale { .. }  => get_eng_documentation(symbol),
    };


    let mut documentation_vector: Vec<MarkedString> = Vec::new();

    let mut tmp: String;
    
    if documentation.title != "" {
        tmp = String::from("### ");
        tmp.push_str(documentation.title);
        documentation_vector.push(MarkedString::String(tmp));
    }

    if documentation.detail != "" {
        tmp = String::from("**");
        tmp.push_str(documentation.detail);
        tmp.push_str("**");
        documentation_vector.push(MarkedString::String(tmp));
    }

    if documentation.description != "" {
        tmp = String::from(documentation.description);
        documentation_vector.push(MarkedString::String(tmp));
    }

    if documentation.syntax != "" {
        tmp = String::from(documentation.syntax);
        documentation_vector.push(MarkedString::LanguageString(LanguageString{ language: "asm8051".to_string(), value: tmp.to_string() }));
    }
    
    if documentation.valid_operands != "" {
        tmp = String::from(match locale { Locale::POLISH => "Poprawne operandy:\n\n", Locale { .. } => "Valid operands:\n\n" });
        tmp.push_str(documentation.valid_operands);
        documentation_vector.push(MarkedString::String(tmp));
    }

    if documentation.affected_flags != "" {
        tmp = String::from(match locale { Locale::POLISH => "Zmodyfikowane flagi:\n\n", Locale { .. } => "Affected flags:\n\n" });
        tmp.push_str(documentation.affected_flags);
        documentation_vector.push(MarkedString::String(tmp));
    }

    documentation_vector
}

fn get_pol_documentation(symbol: String) -> Documentation {
    match symbol.to_uppercase().as_str() {
        // "ADD"  => arithmetic::Pol::ADD,
        // "ADDC" => arithmetic::Pol::ADDC,
        // "SUBB" => arithmetic::Pol::SUBB,

        &_     => Documentation { title: "", detail: "", description: "", syntax: "", affected_flags: "", valid_operands: "" }
    }
}

fn get_eng_documentation(symbol: String) -> Documentation {
    match symbol.to_uppercase().as_str() {
        // "ADD"  => arithmetic::Eng::ADD,
        // "ADDC" => arithmetic::Eng::ADDC,
        // "SUBB" => arithmetic::Eng::SUBB,

        &_     => Documentation { title: "", detail: "", description: "", syntax: "", affected_flags: "", valid_operands: "" }
    }
}

/// Get the symbol/mnemonic/word over which user is hovering
fn get_symbol(document: &TextDocumentItem, position: Position) -> String {
    // split text document into individual lines
    let mut lines = document.borrow().text.lines();

    // go to the line over which user is hovering
    let mut line_option: Option<&str> = Option::None;
    for _i in 0 .. position.line + 1 {
        line_option = lines.next();
    }

    if line_option.is_none() {
        return String::from("");
    }

    // get individual Unicode characters as Vec<&str>
    let graphemes = UnicodeSegmentation::graphemes(line_option.unwrap(), true).collect::<Vec<&str>>();

    let mut symbol_start_position = 0;
    let mut symbol_end_position = graphemes.len() as u32;

    if position.character != 0 {

        // find beginning of the symbol user is hovering over
        for i in (0..=position.character).rev() {
            if !is_valid_character(graphemes[i as usize]) {
                symbol_start_position = i + 1;
                break;
            }
        }
    }
    if position.character + 1 < graphemes.len() as u32 {

        // find end of the symbol user is hovering over
        for i in position.character ..= (graphemes.len() - 1).try_into().unwrap() {
            if !is_valid_character(graphemes[i as usize]) {
                symbol_end_position = i;
                break;
            }
        }
    }

    // using locations of beginning and end of the symbol create a String containing it
    let mut sym :String = String::from("");
    for i in symbol_start_position..symbol_end_position {
        sym.push_str(graphemes[i as usize]);
    }

    sym
}

lazy_static! {
    /// I don't want to create regex each time I want to use it
    static ref IS_VALID_CHARACTER_REGEX: Regex = Regex::new(r"[a-zA-Z0-9_.#@]").unwrap();//[\p{L}\p{N}_.#@]
}

fn is_valid_character(character: &str) -> bool {
    IS_VALID_CHARACTER_REGEX.is_match(character)
}
