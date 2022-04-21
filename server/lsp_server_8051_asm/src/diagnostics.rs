use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, TextDocumentItem};
use rand::Rng;

///IGNORE THIS FOR NOW
/// For now it's just returning random amount of meaningless informations
pub(crate) fn get_diagnostics(text_document: &TextDocumentItem) -> Vec<Diagnostic> {
    let mut rng = rand::thread_rng();
    let number_of_problems: u8 = 0;//rng.gen();

    let mut diagnostics : Vec<Diagnostic> = Vec::new();

    for i in 0..number_of_problems {

        let start: u16 = rng.gen();
        let end: u32 = rng.gen_range(start as u32 .. start as u32 * 2);

        diagnostics.push(Diagnostic{
            range: Range { start: Position{ line: i as u32, character: start as u32 }, end: Position{ line: i as u32, character: end } },
            severity: Option::from(DiagnosticSeverity::INFORMATION),
            code: None,
            code_description: None,
            source: None,
            message: format!("Error number {} in {}", i, text_document.uri.as_str()),
            related_information: None,
            tags: None,
            data: None
        })
    }

    diagnostics
}