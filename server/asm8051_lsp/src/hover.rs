#![allow(unused_imports, dead_code, unused_variables, unused_mut)]
//#region imports
use crate::{client_configuration::ClientConfiguration, client_configuration::{Locale, Kits}, docs};
use asm8051_parser::lexer::tokens::{Token, Keyword, ControlCharacter, PositionedToken, Register, HelperRegister, Number, Directive, Delimiter, Trivia};
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use asm8051_shared::{*, Documentation};
//#endregion



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
        3 => syntax_three_operands(
            key,
            operands[0].borrow(),
            operands[1].borrow(),
            operands[2].borrow(),
            docs.prefix,
        ),
        _ => "".to_string(),
    }
}

fn syntax_one_operand(key: String, operands: &Vec<ValidOperand>, prefix: String) -> String {
    let mut result = format!("{}{} [{}]\n\n", prefix, key, t!("hover.operand"));

    for operand in operands {
        result.push_str(format!("{}{} [{}]\n", prefix, key, operand.operand().label()).as_str());
        result.push_str(
            format!("{}{} {}\n\n", prefix, key, operand.operand().example(None)).as_str(),
        );
    }

    result
}

fn syntax_two_operands(
    key: String,
    operands0: &Vec<ValidOperand>,
    operands1: &Vec<ValidOperand>,
    prefix: String,
) -> String {
    let mut result = format!(
        "{}{} [{}], [{}]\n\n",
        prefix,
        key,
        t!("hover.operand0"),
        t!("hover.operand1")
    );

    for operand0 in operands0.clone() {
        for operand1 in operands1.clone() {
            if operand1.when_first_is() != PossibleOperand::Any
                && operand1.when_first_is() != operand0.operand()
            {
                continue;
            }
            result.push_str(
                format!(
                    "{}{} [{}], [{}]\n",
                    prefix,
                    key,
                    operand0.operand().label(),
                    operand1.operand().label()
                )
                .as_str(),
            );
            result.push_str(
                format!(
                    "{}{} {}, {}\n\n",
                    prefix,
                    key,
                    operand0.operand().example(None),
                    operand1.operand().example(None)
                )
                .as_str(),
            );
        }
    }

    result
}

fn syntax_three_operands(
    key: String,
    operands0: &Vec<ValidOperand>,
    operands1: &Vec<ValidOperand>,
    operands2: &Vec<ValidOperand>,
    prefix: String,
) -> String {
    let mut result = format!(
        "{}{} [{}], [{}], [{}]\n\n",
        prefix,
        key,
        t!("hover.operand0"),
        t!("hover.operand1"),
        t!("hover.operand2")
    );

    for operand0 in operands0.clone() {
        for operand1 in operands1.clone() {
            if operand1.when_first_is() != PossibleOperand::Any
                && operand1.when_first_is() != operand0.operand()
            {
                continue;
            }
            for operand2 in operands2.clone() {
                if operand2.when_first_is() != PossibleOperand::Any
                    && operand2.when_first_is() != operand0.operand()
                {
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
                    )
                    .as_str(),
                );

                result.push_str(
                    format!(
                        "{}{} {}, {}, {}\n\n",
                        prefix,
                        key,
                        operand0.operand().example(None),
                        operand1.operand().example(None),
                        operand2.operand().example(None)
                    )
                    .as_str(),
                );
            }
        }
    }

    result
}

pub(crate) fn generate_affected_flags(client_configuration: &Vec<Flag>) -> String {
    let mut result = String::new();

    for flag in client_configuration {
        result.push_str("- **");
        result.push_str(flag.flag().label().as_str());
        result.push_str("**: ");

        if !flag.when_set.is_empty() {
            result.push_str(t!("hover.setWhen").as_str());
            result.push_str(" ");
            result.push_str(flag.when_set.as_str());
        }

        if !flag.when_set.is_empty() && !flag.when_unset.is_empty() {
            result.push_str(", ");
        }

        if !flag.when_unset.is_empty() {
            result.push_str(t!("hover.unsetWhen").as_str());
            result.push_str(" ");
            result.push_str(flag.when_set.as_str());
        }

        result.push_str("\n");
    }

    result
}

pub(crate) fn generate_valid_operands(operands: &Vec<Vec<ValidOperand>>) -> String {
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
    } else {
        let mut filtered: Vec<Vec<PossibleOperand>> = Vec::new();
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
            result.push_str("**");
            result.push_str(t!("hover.Operand__cap").as_str());
            result.push_str(i.to_string().as_str());
            result.push_str("**: \n");

            for operand in filtered[i].clone() {
                result.push_str(" - ");
                result.push_str(
                    operand
                        .label()
                        .as_str(),
                );
                result.push_str("\n");
            }

            result.push_str("\n\n");
        }
    }

    result
}

pub(crate) fn generate_addressing_modes(addressing_modes: &[AddressingMode]) -> String {
    let mut result = String::new();

    for mode in addressing_modes {
        result.push_str("- ");
        result.push_str(mode.label().as_str());
        result.push_str("\n");
    }

    result
}

pub fn generate_possible_registers(registers: &[PossibleRegister]) -> String {
    let mut result = String::new();

    for register in registers {
        result.push_str("- ");
        result.push_str(register.label().as_str());
        result.push_str("\n");
    }

    result
}


//#endregion syntax generation

#[allow(dead_code)]
/// Finds what user is hovering their cursor over and then tries to match documentation for specified locale
pub(crate) fn documentation(
    position: Position,
    tokens: &Vec::<PositionedToken>,
    locale: Locale,
    kit: Kits,
) -> Vec<MarkedString> {

    let (token, modifier) = match get_symbol(&tokens, position) {
        Some(sym) => sym,
        None => return Vec::new(),
    };

    let mut ast_lines: HashMap<usize, Vec<PositionedToken>> = HashMap::new();

    for token in tokens {
        let line = token.position.line;

        if !ast_lines.contains_key(&line) {
            ast_lines.insert(line, vec![]);
        }

        let mut tokens = ast_lines.get_mut(&line).unwrap();

        tokens.push(token.clone());
    }
    
    match token.token {
        Token::Keyword(kw) => documentation_keyword(kw, modifier, &locale),
        Token::Number(num) => documentation_number(num),
        Token::Label(lb) => documentation_other(lb, token.position, &ast_lines, &locale, kit),
        Token::Other(ot) => documentation_other(ot, token.position, &ast_lines, &locale, kit),
        Token::String(st) => documentation_string(st, &locale),
        _ => Vec::new(),
    }

}

fn documentation_string(st: String, locale: &Locale) -> Vec<MarkedString> {
    vec![
        MarkedString::String(format!("\"{st}\"")),
    ]
}

pub fn documentation_other(
    label: String,
    pos: asm8051_parser::lexer::Position, 
    ast: &HashMap<usize, Vec<PositionedToken>>, 
    locale: &Locale,
    kit: Kits) 
    -> Vec<MarkedString> {
    let (is_macro, line) =  is_symbol_macro(label.as_str(), &ast);
    if is_macro {
        return documentation_label(&label, line, &ast);
    }

    let (is_const, line) = is_symbol_constant(label.as_str(), &ast);
    
    if is_const {
        return documentation_label(&label, line, &ast);
    }

    let (is_label, line) = is_symbol_label(label.as_str(), &ast);

    if is_label {
        return documentation_label(&label, line, &ast);
    }

    let documentation = match docs::get_documentation(locale, &label) {
        Some(docs) => docs,
        None => match docs::get_documentation(&Locale::ENGLISH, &label) {
            Some(docs) => docs,
            None => return <Vec<MarkedString>>::new(),
        },
    };

    if kit != Kits::DSM51 && documentation.category == Kits::DSM51.category_name() {
        return <Vec<MarkedString>>::new();
    }

    documentation_for_found(label, documentation)
}



fn is_symbol_macro(label: &str, ast: &HashMap<usize, Vec<PositionedToken>>) -> (bool, i32) {

    let label_token = Token::Label(String::from(label));

    for (line, tokens) in ast {
        if tokens.len() == 0 {
            continue;
        }

        if label_token != tokens[0].token {
            continue;
        }

        if tokens_contains_any(&tokens, &[ &Token::Keyword(Keyword::Directive(Directive::MACRO)) ]) {
            return (true, (*line as i32));
        }

    }

    (false, 0)
}

fn is_symbol_label(label: &str, ast: &HashMap<usize, Vec<PositionedToken>>) -> (bool, i32) {

    let label_token = Token::Label(String::from(label));

    for (line, tokens) in ast {
        if tokens.len() == 0 {
            continue;
        }

        if label_token != tokens[0].token {
            continue;
        }

        if !tokens_contains_any(&tokens, &[
             &Token::Keyword(Keyword::Directive(Directive::EQU)),
             &Token::Keyword(Keyword::Directive(Directive::SET)),
             &Token::Keyword(Keyword::Directive(Directive::DB)),
             &Token::Keyword(Keyword::Directive(Directive::DW)),
             &Token::Keyword(Keyword::Directive(Directive::REG)),
             &Token::Keyword(Keyword::Directive(Directive::BIT)),
             &Token::Keyword(Keyword::Directive(Directive::MACRO)),
        ]) {
            return (true, (*line as i32));
        }
    }

    (false, 0)
}

fn is_symbol_constant(label: &str, ast: &HashMap<usize, Vec<PositionedToken>>) -> (bool, i32) {
    let label = if label.starts_with("#") {
        let chars = label.chars().collect::<Vec<char>>();
        let chars = chars[1..].borrow();
        String::from_iter(chars)
    } else {
        String::from(label)
    };

    let label_token = Token::Label(label);

    for (line, tokens) in ast {
        if tokens.len() == 0 {
            continue;
        }

        if label_token != tokens[0].token {
            continue;
        }

        if tokens_contains_any(&tokens, &[
             &Token::Keyword(Keyword::Directive(Directive::EQU)),
             &Token::Keyword(Keyword::Directive(Directive::SET)),
             &Token::Keyword(Keyword::Directive(Directive::DB)),
             &Token::Keyword(Keyword::Directive(Directive::DW)),
             &Token::Keyword(Keyword::Directive(Directive::REG)),
             &Token::Keyword(Keyword::Directive(Directive::BIT)),
        ]) {
            return (true, (*line as i32));
        }
    }

    (false, 0)
}

fn tokens_contains_any(tokens: &Vec<PositionedToken>, contains: &[&Token]) -> bool {
    for t in tokens {
        for c in contains {
            if &&t.token == c {
                return true;
            }
        }
    }

    false
}

fn str_contains_any(string: &str, contains: &[&str], case_sensitive: bool) -> bool {
    let t = match case_sensitive {
        false => string.to_lowercase(),
        true => string.to_string(),
    };
    for cont in contains {
        let c = match case_sensitive {
            false => cont.to_lowercase(),
            true => cont.to_string(),
        };

        if t.contains(&c) {
            return true;
        }
    }

    false
}

fn documentation_label(label: &String, line: i32, ast: &HashMap<usize, Vec<PositionedToken>>) -> Vec<MarkedString> {
    if line <= 1 {
        return Vec::new();
    }

    let mut index = (line - 1) as usize;
    let prev_line = match ast.get(&index) {
        Some(l) => l,
        None => return Vec::new(),
    };

    if !is_comment_line(&prev_line) {
        return Vec::new();
    }

    let mut string_lines: Vec<String> = Vec::new();

    while let Some(line) = ast.get(&index) {
        if !is_comment_line(&line) {
            break;
        }

        string_lines.push(get_comment_string(&line));
        if index == 0{
            break;
        }
        index -= 1;
    }

    string_lines.reverse();

    let mut documentation_vector: Vec<MarkedString> = Vec::new();

    let tmp = format!("**{}**", label.as_str());
    documentation_vector.push(MarkedString::String(tmp));

    let mut tmp = String::new();
    for line in string_lines {
        tmp.push_str(&line.trim());
        tmp.push_str("\n\n");
    }
    documentation_vector.push(MarkedString::String(clean_markdown(&tmp)));

    documentation_vector
}

fn get_comment_string(line: &[PositionedToken]) -> String {
    if line.len() == 0 {
        return String::new();
    }

    panic_on_mismatched_lines(&line);

    let mut string = String::new();
    for item in line {
        let content = match &item.token {
            Token::Trivia(tr) => match tr {
                Trivia::Comment(cm) => cm.as_str(),
                _ => "",
            },
            _ => "",
        };

        string.push_str(content);
    }

    string
}

fn is_comment_line(line: &[PositionedToken]) -> bool {

    if line.len() == 0 {
        return false;
    }

    panic_on_mismatched_lines(line);
    
    match &line[0].token {
        Token::ControlCharacter(cc) => match cc {
                ControlCharacter::Delimiter(d) => 
                    match d {
                        Delimiter::CommentStart => return true,
                        _ => return false,
                    },
                _ => return false,
            },
        _ => return false,
    };
}

fn panic_on_mismatched_lines(line: &[PositionedToken]) {
    if line.len() == 0 {
        return;
    }

    let line_index = line[0].position.line;

    for item in line {
        if item.position.line != line_index {
            panic!("You must provide tokens with matching position.line field");
        }
    }
}

fn clean_markdown(tmp: &str) -> String {
    //TODO: remove command links
    String::from(tmp)
}

fn documentation_number(number: Number) -> Vec<MarkedString> {

    let header = t!("hover.numberBase_header");
    let label_binary = t!("hover.numberBase_label_binary");
    let label_octal = t!("hover.numberBase_label_octal");
    let label_decimal = t!("hover.numberBase_label_decimal");
    let label_hex = t!("hover.numberBase_label_hexadecimal");

    let parsed = match number {
        Number::Binary(bin) => i32::from_str_radix(bin.as_str(), 2),
        Number::Octal(oct) => i32::from_str_radix(oct.as_str(), 8),
        Number::Decimal(dec) => i32::from_str_radix(dec.as_str(), 10),
        Number::Hexadecimal(hex) => i32::from_str_radix(hex.as_str(), 16),
    };

    let value = match parsed {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let string = MarkedString::String(format!(
        r#"#### {header}

{label_binary}: #{value:b}B

{label_octal}: #{value:o}O

{label_decimal}: #{value}

{label_hex}: #{value:X}H"#));

    vec![string]
}

fn documentation_keyword(mnemonic: Keyword, modifier: AddressingModifier, locale: &Locale) -> Vec<MarkedString> {

    // get string representation...
    let string_repr = match mnemonic {
        Keyword::Instruction(ins) => ins.to_string(),
        Keyword::Directive(dir) => dir.to_string(),
        Keyword::FlagOrBit(fob) => fob,
        Keyword::Register(reg) => match reg {
            Register::Main(mr) => mr.to_string(),
            Register::Special(sr) => sr.to_string(),
            Register::Port(pr) => pr.to_string(),
            Register::Helper(hr) => match hr {
                HelperRegister::R0 | HelperRegister::R1 if modifier == AddressingModifier::Yes => format!("@{}", hr.to_string()),
                 _ => hr.to_string()
            },
        },
    };

    let documentation = match docs::get_documentation(locale, &string_repr) {
        Some(docs) => docs,
        None => match docs::get_documentation(&Locale::ENGLISH, &string_repr) {
            Some(docs) => docs,
            None => return <Vec<MarkedString>>::new(),
        },
    };

    documentation_for_found(string_repr, documentation)
}

fn documentation_for_found(string_repr: String, documentation: Documentation) -> Vec<MarkedString> {
    let mut documentation_vector: Vec<MarkedString> = Vec::new();

    let tmp = format!("**{}**", string_repr.as_str());
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

    // let tmp = syntax((mnemonic.clone(), documentation.clone()));
    // if tmp != "" {
    //     documentation_vector.push(MarkedString::LanguageString(LanguageString {
    //         language: crate::LANG_ID.to_string(),
    //         value: tmp.to_string(),
    //     }));
    // }

    if let Some(space) = documentation.stack_space_needed {
        documentation_vector.push(MarkedString::String(format!(
            "#### {}: {}",
            t!("hover.StackSpaceNeeded"),
            space
        )));
    }

    let tmp = generate_addressing_modes(&documentation.addressing_modes);

    if tmp != "" {
        documentation_vector.push(MarkedString::String(format!(
            "{}:\n\n{}",
            t!("hover.addressingModes"),
            tmp
        )));
    }

    let tmp = generate_valid_operands(&documentation.valid_operands);

    if tmp != "" {
        documentation_vector.push(MarkedString::String(format!(
            "{}:\n\n{}",
            t!("hover.validOperands"),
            tmp
        )));
    }

    let tmp = generate_affected_flags(&documentation.affected_flags);
    if tmp != "" {
        documentation_vector.push(MarkedString::String(format!(
            "{}:\n\n{}",
            t!("hover.affectedFlags"),
            tmp
        )));
    }

    let tmp = generate_possible_registers(&documentation.used_registers);
    if tmp != "" {
        documentation_vector.push(MarkedString::String(format!(
            "{}:\n\n{}",
            t!("hover.usedRegisters"),
            tmp
        )));
    }

    let tmp = generate_possible_registers(&documentation.changed_registers);
    if tmp != "" {
        documentation_vector.push(MarkedString::String(format!(
            "{}:\n\n{}",
            t!("hover.changedRegisters"),
            tmp
        )));
    }

    documentation_vector.push(MarkedString::String(format!(
        "[{}](command:asm8051.openDocs?%7B%22category%22:%22{}%22,%22item%22:%22{}%22%7D)",
        t!("hover.goToDocs"),
        documentation.category,
        string_repr
    )));

    documentation_vector
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum AddressingModifier {
    Yes, No
}

/// Get the symbol/mnemonic/word over which user is hovering
fn get_symbol(document: &Vec<PositionedToken>, position: Position) -> Option<(PositionedToken, AddressingModifier)> {
    // split text document into individual lines
    
    let current_line = document.iter()
        .filter(|x| x.position.line as u32 == position.line)
        .collect::<Vec<_>>();

    let _my_line_str = current_line.iter().map(|x| x.to_string()).collect::<Vec<_>>();

    let mut tok: Option<(&PositionedToken, AddressingModifier)> = Option::None;

    for i in 0..(current_line.len()) {
        let token = &current_line[i];
        let _token_str = token.to_string();

        let col = position.character as usize;
        let range = &token.position.columns;
        if range.start <= col && col <= range.end {
            if token.token == Token::ControlCharacter(ControlCharacter::AddressingModifier) {
                if i == current_line.len() - 1 {
                    return Option::None;
                }
                else {
                    tok = Some((&current_line[i + 1], AddressingModifier::Yes));
                    break 
                }
            }
            else if token.token == Token::ControlCharacter(ControlCharacter::AddressingSeparator) {
                if i == 0 {
                    return Option::None;
                }
                else {
                    tok = Some((&current_line[i - 1], AddressingModifier::No));
                    break;
                }
            }
            else if 
                token.token == Token::ControlCharacter(ControlCharacter::Delimiter(Delimiter::DoubleQuote)) ||
                token.token == Token::ControlCharacter(ControlCharacter::Delimiter(Delimiter::SingleQuote)) {

                if i > 0 {
                    let index = i - 1;
                    if let Token::String(st) = &current_line[index].token {
                        tok = Some((&current_line[index], AddressingModifier::No));
                        break;
                    }
                }
                else if i < current_line.len() - 1 {
                    let index = i + 1;
                    if let Token::String(st) = &current_line[index].token {
                        tok = Some((&current_line[index], AddressingModifier::No));
                        break;
                    }
                }
            }
            else {
                tok = match token.token.clone() {
                    Token::Address(_) if i >= 2 && &current_line[i - 1].token == &Token::ControlCharacter(ControlCharacter::AddressingSeparator) 
                      => Some((&current_line[i - 2], AddressingModifier::No)),
                    Token::Keyword(kw) => match kw {
                        Keyword::Register(rg) => match rg {
                            Register::Helper(hl) => match hl {
                                HelperRegister::R0 | HelperRegister::R1 if i >= 1 && &current_line[i - 1].token == &Token::ControlCharacter(ControlCharacter::AddressingModifier)
                                  => Some((token, AddressingModifier::Yes)),
                                _ => Some((token, AddressingModifier::No)),
                            },
                            _ => Some((token, AddressingModifier::No)),
                        },
                        _ => Some((token, AddressingModifier::No)),
                    },

                    _ => Some((token, AddressingModifier::No)),
                };
            }
        }
    }

    match tok {
        Some((token, modifier)) => Some((token.clone(), modifier)),
        None => None,
    }
}

fn is_valid_character(character: char) -> bool {
    let text = character.to_string();
    IS_VALID_CHARACTER_REGEX.is_match(text.as_str())
}

lazy_static! {
    /// I don't want to create regex each time I want to use it
    pub static ref IS_VALID_CHARACTER_REGEX: Regex = Regex::new(r"[a-zA-Z0-9_.#@]").unwrap();//[\p{L}\p{N}_.#@]
}

// // #region tests
// #[cfg(test)]
// mod tests {
//     mod all_documentation {
//         use crate::{client_configuration::Locale, hover::all_documentation};
//         use test_case::test_case;

//         #[test_case(Locale::POLISH,  true  ; "some for POLISH locale")]
//         #[test_case(Locale::ENGLISH, true  ; "some for ENGLISH locale")]
//         #[test_case(Locale::DEFAULT, false ; "none for DEFAULT locale")]
//         fn is(locale: Locale, is_some: bool) {
//             let docs = all_documentation(locale);
//             assert_eq!(docs.is_some(), is_some);
//         }

//         #[test_case(Locale::POLISH  ; "polish locale")]
//         #[test_case(Locale::ENGLISH ; "english locale")]
//         fn is_not_empty_for(locale: Locale) {
//             let docs = all_documentation(locale).unwrap();
//             assert!(docs.len() > 0);
//         }
//     }

//     mod generated_syntax {
//         mod with_syntax_function {
//             use test_case::test_case;

//             use crate::hover::{syntax, Documentation, Flag, ValidOperand};

//             fn empty_valid_operands() -> Vec<Vec<ValidOperand>> {
//                 Vec::<Vec<ValidOperand>>::new()
//             }
//             fn empty_affected_flags() -> Vec<Flag> {
//                 Vec::<Flag>::new()
//             }

//             #[test_case(
//                 "TEST",
//                 Documentation::new(
//                     "", 
//                     "some description", 
//                     Vec::<Vec::<ValidOperand>>::new(),
//                     Vec::<Flag>::new(),
//                     true,
//                     false,
//                     "TEST",
//                     "category", 
//                     "", 
//                     false,
//                     Option::None) ; "dont_generate_syntax is set")]
//             #[test_case("", Documentation::new("", "", empty_valid_operands(), empty_affected_flags(), false, false, "", "", "", false, Option::None) ; "no data is provided") ]
//             #[test_case("TEST", Documentation::new(
//                 "some detail",
//                 "some description",
//                 vec!(Vec::<ValidOperand>::new()),
//                 empty_affected_flags(),
//                 false,
//                 false,
//                 "TEST",
//                 "category", 
//                 "", 
//                 false,
//                 Option::None) ; "any of the inner vectors of valid operands is empty")]
//             #[test_case("TEST", Documentation::new(
//                 "some detail",
//                 "some description",
//                 vec!(
//                     vec!(ValidOperand::from_i32(1, None)),
//                     vec!(ValidOperand::from_i32(1, None)),
//                     vec!(ValidOperand::from_i32(1, None)),
//                     vec!(ValidOperand::from_i32(1, None))
//                 ),
//                 empty_affected_flags(),
//                 false,
//                 false,
//                 "TEST",
//                 "category", 
//                 "", 
//                 false,
//                 Option::None) ; "length of valid_operands vector is greater than 3")]
//             fn is_empty_when(mnemonic: &str, doc: Documentation) {
//                 let mnemonic = String::from(mnemonic);
//                 let syntax = syntax((mnemonic, doc));
//                 assert!(syntax.is_empty());
//             }

//             // #[test_case(
//             //     Documentation::new(
//             //         "some detail",
//             //         "some quick description",
//             //         vec!(vec!(ValidOperand::from_i32(1, None), ValidOperand::from_i32(2, None))),
//             //         vec!(Fla)
//             //     ))]
//             // fn is_not_empty_when_some_data_is_provided_and_dont_generate_syntax_is_not_set(doc: Documentation) {
//             //     let syntax = syntax((String::from("TEST"), doc));
//             // }
//         }
//     }
// }
// #[cfg(test)]
// mod test_all_documentation {
//     use crate::{client_configuration::Locale, hover::all_documentation};

//     #[test]
//     fn while_locale_is_polish() {
//         let docs = all_documentation(Locale::POLISH);
//         assert!(docs.is_some(), "Docs are Option::None");

//         let docs = docs.unwrap();
//         assert!(
//             !docs.is_empty(),
//             "Docs are a HashMap, but that HashMap is empty"
//         );
//     }

//     #[test]
//     fn while_locale_is_english() {
//         let docs = all_documentation(Locale::ENGLISH);
//         assert!(docs.is_some(), "Docs are Option::None");

//         let docs = docs.unwrap();
//         assert!(
//             !docs.is_empty(),
//             "Docs are a HashMap, but that HashMap is empty"
//         );
//     }

//     #[test]
//     fn while_locale_is_default() {
//         let docs = all_documentation(Locale::DEFAULT);
//         assert!(docs.is_none(), "Docs are not Option::None");
//     }
// }

// #[cfg(test)]
// mod test_generating_syntax {
//     //use std::borrow::Borrow;

//     use test_case::test_case;

//     use super::{syntax_one_operand, ValidOperand};

//     fn test_mnemonic() -> String {
//         String::from("TEST")
//     }

//     #[test_case(Vec::<ValidOperand>::new() ; "with zero valid operands")]
//     #[test_case(vec!(ValidOperand::from_i32(12, None)) ; "with one valid operands")]
//     #[test_case(vec!(ValidOperand::from_i32(12, None), ValidOperand::from_i32(3, None)) ; "with two valid operands")]
//     #[test_case(vec!(ValidOperand::from_i32(12, None), ValidOperand::from_i32(3, None), ValidOperand::from_i32(5, None)) ;
//         "with three valid operands")]
//     fn for_one_operand(operands: Vec<ValidOperand>) {
//         let syntax = syntax_one_operand(test_mnemonic(), &operands, String::from(""));

//         let syntax_header = format!("{} [operand]", test_mnemonic());
//         assert!(syntax.starts_with(&syntax_header));

//         for operand in operands {
//             let section_for_operand = format!(
//                 "{} [{}]\n{} {}",
//                 test_mnemonic(),
//                 operand.operand().label(),
//                 test_mnemonic(),
//                 operand.operand().example(Some(1))
//             );
//             assert!(syntax.contains(&section_for_operand));
//         }
//     }

//     // fn for_two_operands(operands0: Vec::<ValidOperand>, operands1: Vec::<ValidOperand>) {
//     //     let syntax = crate::hover::syntax_two_operands(test_mnemonic(), &operands0, &operands1, String::from(""));

//     //     let syntax_header = format!("{} [operand]", test_mnemonic());
//     //     assert!(syntax.starts_with(&syntax_header));

//     //     for operand in operands0 {
//     //         for operand1 in operands1 {

//     //         }
//     //     }
//     // }
// }
// //#endregion tests
