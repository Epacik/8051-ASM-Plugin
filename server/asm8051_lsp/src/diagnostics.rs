use asm8051_parser::lexer::tokens::PositionedToken;
use tower_lsp::lsp_types::{Diagnostic, TextDocumentItem, Range, Position, DiagnosticSeverity, NumberOrString};

use crate::{flags::{Kits, Locale}, docs};


fn map_position(pos: &asm8051_parser::lexer::Position) -> Range {
    let line = pos.line as u32;
    let start = pos.columns.start as u32;
    let end = pos.columns.end as u32;

    Range { 
        start: Position { line, character: start + 1 }, 
        end:   Position { line, character: end + 1 } 
    }
}

fn map_severity(sev: &asm8051_parser::issues::IssueType) -> DiagnosticSeverity {
    match sev {
        asm8051_parser::issues::IssueType::Error => DiagnosticSeverity::ERROR,
        asm8051_parser::issues::IssueType::Warning => DiagnosticSeverity::WARNING,
        asm8051_parser::issues::IssueType::Info => DiagnosticSeverity::INFORMATION,
        asm8051_parser::issues::IssueType::Hint => DiagnosticSeverity::HINT,
    }
}

pub(crate) fn get_diagnostics(_text_document: &TextDocumentItem, kit: Kits) -> Vec<Diagnostic> {

    let (tokens, errors) = asm8051_parser::lexer::lexical_analysis(&_text_document.text);

    let tokens = match tokens {
        Some(t) => t,
        None => Vec::<PositionedToken>::new(),
    };

    let labels = asm8051_parser::lexer::get_label_definitions(&tokens);

    let mnemonics = docs::all_documentation(&Locale::ENGLISH)
        .unwrap()
        .iter()
        .filter(|(key, value)| kit == Kits::DSM51 || value.category != Kits::DSM51.category_name());

    let mut diagnostics : Vec<Diagnostic> = Vec::new();
    for error in errors {
        let message = String::from(error.info().message_key());
        diagnostics.push(Diagnostic{
                range: map_position(&error.position()),
                severity: Some(map_severity(&error.info().default_type())),
                code: Some(NumberOrString::Number(error.info().code() as i32)),
                code_description: None,
                source: Some(String::from("asm8051")),
                message,
                related_information: None,
                tags: None,
                data: None
            });
    }

    diagnostics
}
