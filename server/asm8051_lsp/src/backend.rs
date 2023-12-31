use crate::docs;
use crate::flags::Kits;
//#region imports hell
use crate::{
    client_configuration::ClientConfiguration, diagnostics, flags::Locale, hover, LANG_ID,
};

use crate::i18n::change_language;

use asm8051_parser::lexer::tokens::{
    ControlCharacter,
    Register,
    Instruction,
    Trivia,
    Token,
    Keyword,
    MainRegister,
    SpecialRegister,
    Delimiter
};
use asm8051_shared::{PossibleOperand, ValidOperand};

use dashmap::DashMap;
use tower_lsp::jsonrpc::*;
use tower_lsp::lsp_types::*;
use tower_lsp::*;

use serde_json::Value;
use std::{borrow::Borrow, option::Option, string::String, sync::Arc};
use tokio::sync::Mutex;
//#endregion imports

/// Connection with a client and additional configutation
#[derive(Debug)]
pub(crate) struct Backend {
    /// connection with LSP client
    pub(crate) client: Client,

    /// documents opened in editor indexed by uri to that document
    pub(crate) documents: DashMap<String, TextDocumentItem>,

    /// things that client supports
    pub(crate) client_capabilities: Arc<Mutex<ClientCapabilities>>,

    /// configuration received from client
    pub(crate) client_configuration: Arc<Mutex<ClientConfiguration>>,

    pub(crate) client_locale: Arc<Mutex<Locale>>,

    pub(crate) semantic_token_map: DashMap<SemanticTokenType, u32>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // updating capabilities of client
        try_update_mutex_value(self.client_capabilities.borrow(), params.capabilities).await;

        // time to set some capabilities, so the client knows what server can do
        let result = InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                // capability to execute commands
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["test.command".to_string()],
                    work_done_progress_options: Default::default(),
                }),
                // how documents are synced
                text_document_sync: Some(TextDocumentSyncCapability::from(
                    TextDocumentSyncKind::FULL,
                )),

                completion_provider: Some(CompletionOptions {
                    resolve_provider: Option::from(false),
                    trigger_characters: {
                        let capital = (65u8..=90u8).into_iter()
                            .map(|i| (i as char).to_string())
                            .collect::<Vec<String>>();
                        let lower = (97u8..=122u8).into_iter()
                            .map(|i| (i as char).to_string())
                            .collect::<Vec<String>>();
                        let other = vec![
                                ",",
                                " ",
                                "\t",
                                "\n",
                                "\r",
                                "\x0B",
                                "\x0C",
                                "\u{0085}",
                                "\u{2028}",
                                "\u{2029}"]
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>();

                        let all = capital
                            .iter()
                            .chain(lower.iter())
                            .chain(other.iter())
                            .map(|x| x.clone())
                            .collect::<Vec<String>>();

                        Some(all)
                    },
                    all_commit_characters: None,
                    work_done_progress_options: Default::default(),
                    completion_item: Some(CompletionOptionsCompletionItem { label_details_support: Some(true) })
                }),
                hover_provider: Some(HoverProviderCapability::Options(HoverOptions {
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: Some(true),
                    },
                })),
                //references_provider: Some(OneOf::Left(true)),
                // semantic_tokens_provider: Some(
                //     SemanticTokensServerCapabilities::SemanticTokensOptions(
                //         {
                //             SemanticTokensOptions { 
                //                 work_done_progress_options: WorkDoneProgressOptions { work_done_progress: Some(false) }, 
                //                 range: Some(false), 
                //                 full: Some(SemanticTokensFullOptions::Bool(true)),
                //                 legend: SemanticTokensLegend { 
                //                     token_types:{
                //                         let mut vec = (&self.semantic_token_map)
                //                             .clone()
                //                             .into_iter()
                //                             .collect::<Vec<(SemanticTokenType, u32)>>();
                //                         vec.sort_by(|a, b| a.1.cmp(&b.1));
                                        
                //                         vec.iter()
                //                         .map(|x| x.0.clone())
                //                         .collect()
                //                     },
                //                     token_modifiers: vec![]
                //                 }, 
                //             }
                //         })),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()

            
        };

        // capability to show documentation on hover

        self.update_configuration().await;

        let lang = params.locale.unwrap_or_default();
        let lang_localization = match lang.as_str() {
            "pl" => "pl",
            _ => "en",
        };

        change_language(lang_localization);
        
        let locale = match lang.as_str() {
            "pl" => Locale::POLISH,
            _ => Locale::ENGLISH,
        };
        try_update_mutex_value(self.client_locale.borrow(), locale).await;

        change_language(locale.lang_name().as_str());

        Ok(result)
    }

    async fn initialized(&self, _params: InitializedParams) {
        // add custom event in case user changes configuration in editor that supports it
        if has_configuration_capability(self.client_capabilities.lock().await.clone()) {
            let _register_result = self
                .client
                .register_capability(vec![Registration {
                    id: "workspace/didChangeConfiguration".to_string(),
                    method: "workspace/didChangeConfiguration".to_string(),
                    register_options: None,
                }])
                .await;
        }
        
        self.update_configuration().await;
        self.validate_all_documents().await;

        self.client
            .log_message(MessageType::INFO, t!("status.initialized"))
            .await;

    }

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::INFO, t!("status.shutdown"))
            .await;
        Ok(())
    }

    async fn did_change_configuration(&self, _params: DidChangeConfigurationParams) {

        self.client
            .log_message(MessageType::INFO, t!("status.config_changed"))
            .await;

        self.update_configuration().await;
        self.validate_all_documents().await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        
        // TODO: REFACTOR THIS
        self.client
            .log_message(MessageType::INFO, t!("status.completion"))
            .await;
        let kit = self.client_configuration.lock().await.kit();

        let locale = self.client_locale().await;
        let documentation = docs::all_documentation(&locale);

        if documentation.is_none() {
            return Ok(None);
        }

        let documentation = documentation.unwrap();
        
        let doc_id = params.text_document_position.text_document.uri.as_ref();
        let document = self.documents.get(doc_id);

        let document = match document {
            Some(doc) => doc,
            None => return Ok(None),
        };

        let document = document.value().text.clone();

        let (tokens, issues) = asm8051_parser::lexer::lexical_analysis(document);

        if issues.len() > 0 || tokens.is_none()
        {
            return Ok(None);
        }

        let tokens = tokens.unwrap();
        let _labels = asm8051_parser::lexer::get_label_definitions(&tokens);
        let current_line = asm8051_parser::lexer::get_line(
            &tokens, 
            params.text_document_position.position.line as usize);

        let char_num = params.text_document_position.position.character as usize;


        if char_num == 1 {
            return Ok(None);
        }

        let instructions = asm8051_parser::lexer::keywords::get_instruction_strings();
        let directives = asm8051_parser::lexer::keywords::get_directive_strings();
        let keywords = instructions
            .iter()
            .chain(directives.iter())
            .collect::<Vec<&String>>();

        let details = documentation
            .iter()
            .filter(|(key, _)| keywords.contains(&key))
            .map(|(key, value)| 
                CompletionItem {
                    label: key.clone(),
                    detail: Some(value.detail.clone()),
                    documentation: {
                        let doc = hover::documentation(
                            params.text_document_position.position.clone(),
                            &tokens,
                            locale,
                            kit)
                            .iter()
                            .map(|x| match x {
                                MarkedString::String(s) => s.clone(),
                                MarkedString::LanguageString(ls) => ls.value.clone()
                            })
                            .collect::<Vec<String>>()
                            .join("\n\n");

                        Some(
                            lsp_types::Documentation::MarkupContent(
                                MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value: doc,
                                }
                            )
                        )
                    },
                    kind: {
                        if instructions.contains(&key) {
                            Some(CompletionItemKind::FUNCTION)
                        }
                        else if directives.contains(&key) {
                            Some(CompletionItemKind::EVENT)
                        }
                        else {
                            None
                        }
                    },
                    ..CompletionItem::default()
                })
            .collect::<Vec<CompletionItem>>();

        if current_line.len() == 1 {
            return Ok(Some(CompletionResponse::Array(details)));
        }

        let first_item = &current_line[1];
        let column = (char_num - 1) as usize;

        if first_item.position.contains_column(column) {
            return Ok(Some(CompletionResponse::Array(details)));
        }

        if current_line.len() < 3 {
            return Ok(None);
        }

        if let Token::Keyword(kw) = first_item.token.clone() {
            let kw = if let Keyword::Instruction(ins) = kw {
                Some(ins.to_string())
            }
            else if let Keyword::Directive(dir) = kw {
                Some(dir.to_string())
            }
            else {
                None
            };

            let kw = match kw {
                Some(k) => k,
                None => return Ok(None),
            };

            let docs = match documentation.get(&kw) {
                Some(d) => d,
                None => return Ok(None),
            };

            // a line should look something like:
            // 0           1            2           3         4      5                 6
            // [whitespace][instruction][whitespace][argument]
            // [whitespace][instruction][whitespace][argument]
            // [whitespace][instruction][whitespace][argument][comma][(opt) whitespace][argument]
            // [whitespace][instruction][whitespace][argument][comma][(opt) whitespace][argument][comma][(opt) whitespace][argument]
            // 0 (l 1)     1 (l 2)      2 (l 3)     3 (l 4)   4(l 5) 5                 6

            let current_args = {
                let mut args: Vec<Vec<&asm8051_parser::lexer::tokens::PositionedToken>> = Vec::new();

                let mut index = 1_usize;
                for pt in &current_line[2..] {
                    if args.len() <= index {
                        args.push(vec![]);
                    }

                    if let Token::Trivia(tv) = pt.token.clone() {
                        if let Trivia::NewLine(_) = tv {
                            break;
                        }
                    }

                    
                    if let Token::ControlCharacter(cc) = pt.token.clone() {
                        if let ControlCharacter::ArgumentSeparator = cc {
                            index = index + 1;
                            //continue;
                        }
                        if let ControlCharacter::Delimiter(del) = cc {
                            if let Delimiter::CommentStart = del {
                                break;
                            }
                        }
                    }
                    
                    args[index - 1].push(pt);
                }

                let mut result : Vec<Vec<&asm8051_parser::lexer::tokens::PositionedToken>> = Vec::new();

                for a in args {
                    if a.len() > 0 {
                        result.push(a);
                    }
                }

                result
            };

            let first_current_operand = &current_args[0];
            let first_possible_op = {
                let mut address = false;
                let mut helper_r = false;
                let mut at = false;
                let mut a = false;
                let mut b = false;
                let mut dph = false;
                let mut dpl = false;
                let mut c = false;
                let mut dptr = false;

                for o in first_current_operand {
                    match o.token.clone() {
                        Token::Keyword(kw) => match kw {
                            Keyword::Register(reg) => match reg {
                                Register::Main(m) => match m {
                                    MainRegister::A => a = true,
                                    MainRegister::B => b = true,
                                    MainRegister::AB => {},
                                },
                                Register::Helper(_) => helper_r = true,
                                Register::Special(sp) => match sp {
                                    SpecialRegister::DPL => dpl = true,
                                    SpecialRegister::DPH => dph = true,
                                    SpecialRegister::DPTR => dptr = true,
                                    _ => {}
                                },
                                _ => {}
                            },
                            Keyword::FlagOrBit(fob) => if fob.as_str() == "C" {
                                c = true;
                            },
                            _ => {}
                        },
                        Token::Label(_) => address = true,
                        Token::Address(_) => address = true,
                        Token::ControlCharacter(cc) => 
                            if let ControlCharacter::AddressingModifier = cc { at = true; },
                        _ => {}
                    }

                }

                if address {
                    PossibleOperand::CodeAddress
                }
                else if at && helper_r {
                    PossibleOperand::AddressInR0OrR1 
                }
                else if helper_r{
                    PossibleOperand::HelperRegisters
                }
                else if a {
                    PossibleOperand::Accumulator
                }
                else if b {
                    PossibleOperand::RegisterB
                }
                else if c {
                    PossibleOperand::CarryFlag
                }
                else if at && dptr {
                    PossibleOperand::AddressInDptr 
                }
                else if dptr {
                    PossibleOperand::Dptr
                }
                else if dph {
                    PossibleOperand::Dph
                }
                else if dpl {
                    PossibleOperand::Dpl
                }
                else {
                    PossibleOperand::Any
                }
            };

            let len = std::cmp::min(docs.valid_operands.len(), current_args.len());
            
            if docs.valid_operands.len() == 0 || len == 0 {
                return Ok(None);
            }

            for i in 0..len {
                let args = &current_args[i];
                let has_column = {
                    let mut hc = false;
                    for arg in args {
                        if arg.position.contains_column(column){
                            hc = true;
                        }
                    }
                    hc
                };

                if !has_column {
                    continue;
                }

                let ops = if i == 0 || first_possible_op == PossibleOperand::Any {
                    docs.valid_operands[i]
                        .iter()
                        .filter(|_| true)
                        .collect::<Vec<&ValidOperand>>()
                }
                else {
                    docs.valid_operands[i]
                        .iter()
                        .filter(|x| 
                            x.when_first_is == PossibleOperand::Any || 
                            x.when_first_is == first_possible_op)
                        .collect::<Vec<&ValidOperand>>()
                };

                let ci = ops
                    .iter()
                    .map(|x| {
                    match x.operand {
                        PossibleOperand::Label |
                        PossibleOperand::AbsoluteAddress |
                        PossibleOperand::RelativeAddress |
                        PossibleOperand::CodeAddress => {
                            Some(_labels
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.clone(),
                                        kind: Some(CompletionItemKind::CONSTANT),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        PossibleOperand::AddressInR0OrR1 => {
                            Some(vec!["@R0", "@R1"]
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.to_string(),
                                        kind: Some(CompletionItemKind::REFERENCE),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        PossibleOperand::HelperRegisters => {
                            Some(vec!["R0", "R1", "R2", "R3", "R4", "R5", "R6", "R7"]
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.to_string(),
                                        kind: Some(CompletionItemKind::ENUM),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        PossibleOperand::CarryFlag => {
                            Some(vec!["C"]
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.to_string(),
                                        kind: Some(CompletionItemKind::INTERFACE),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        PossibleOperand::Accumulator => {
                            Some(vec!["A"]
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.to_string(),
                                        kind: Some(CompletionItemKind::ENUM),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        PossibleOperand::AccumulatorAndB => {
                            Some(vec!["AB"]
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.to_string(),
                                        kind: Some(CompletionItemKind::ENUM),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        PossibleOperand::AddressInAccumulatorPlusDptr => {
                            Some(vec!["@A+DPTR"]
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.to_string(),
                                        kind: Some(CompletionItemKind::REFERENCE),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        PossibleOperand::Dptr => {
                            Some(vec!["DPTR"]
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.to_string(),
                                        kind: Some(CompletionItemKind::ENUM),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        PossibleOperand::AddressInDptr => {
                            Some(vec!["@DPTR"]
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.to_string(),
                                        kind: Some(CompletionItemKind::REFERENCE),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        PossibleOperand::AddressInAccumulatorPlusPC => {
                            Some(vec!["@A+PC"]
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.to_string(),
                                        kind: Some(CompletionItemKind::REFERENCE),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        PossibleOperand::RegisterB => {
                            Some(vec!["B"]
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.to_string(),
                                        kind: Some(CompletionItemKind::ENUM),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        PossibleOperand::Dpl => {
                            Some(vec!["DPL"]
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.to_string(),
                                        kind: Some(CompletionItemKind::ENUM),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        PossibleOperand::Dph => {
                            Some(vec!["DPH"]
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.to_string(),
                                        kind: Some(CompletionItemKind::ENUM),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        PossibleOperand::AsciiCharacters => {
                            Some(vec!["\"\""]
                                .iter()
                                .map(|y| {
                                    CompletionItem {
                                        label: y.to_string(),
                                        kind: Some(CompletionItemKind::ENUM),
                                        ..CompletionItem::default()
                                    }
                                })
                                .collect::<Vec<CompletionItem>>())
                        },
                        _ => None,
                    }
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .flatten()
                .collect::<Vec<CompletionItem>>();

                return Ok(Some(CompletionResponse::Array(ci)));
            }

        }


        Ok(None)
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        self.client.log_message(MessageType::INFO, t!("status.hover")).await;
        let kit = self.client_configuration.lock().await.kit();

        // get clients configuration
        //let config = self.client_configuration.lock().await.clone();

        //load text of a document user id hovering over
        let uri = _params
            .text_document_position_params
            .text_document
            .uri
            .as_ref();

        let document = self.documents.get(uri);

        if document.is_none() {
            return Err(Error {
                code: ErrorCode::ServerError(2),
                message: t!("error.document_read"),
                data: None,
            });
        }

        let document = document.unwrap();

        let (tokens, _) = asm8051_parser::lexer::lexical_analysis(&document.borrow().text);
        let ast = match tokens {
            Some(s) => s,
            None => return Ok(None),
        };

        //get documentation for whatever user is hovering over
        let doc = hover::documentation(
            _params.text_document_position_params.position,
            &ast,
            self.client_locale().await,
            kit
        );

        Ok(Some(Hover {
            contents: HoverContents::Array(doc),
            range: None,
        }))
    }

    /// Add opened file to a local hashmap of opened files and validate it
    #[allow(unused_mut)]
    async fn did_open(&self, _params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file open")
            .await;

        if _params.text_document.language_id != String::from(LANG_ID) {
            return;
        }

        let file_uri = _params.text_document.uri.as_str();
        let file = _params.text_document.clone();

        self.documents.insert(String::from(file_uri), file);

        self.validate_document(_params.text_document.borrow()).await;
    }

    /// remove file from local HashMap of opened files
    #[allow(unused_mut)]
    async fn did_close(&self, _params: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file closed")
            .await;
        let file_uri = _params.text_document.uri.as_str();

        if self.documents.contains_key(file_uri) {
            self.documents.remove(file_uri);
        }
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file changed!")
            .await;
        let file_uri = params.text_document.uri.to_string();

        if !self.documents.contains_key(file_uri.as_str()) {
            return;
        }

        let document = TextDocumentItem {
            uri: params.text_document.uri,
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
            language_id: String::from(LANG_ID),
        };

        self.documents.insert(file_uri, document.clone());

        self.validate_document(document.borrow()).await;
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {

        let uri = params.text_document.uri.to_string();
        let document = match self.documents.get(&uri) {
            Some(d) => d.value().clone(),
            None => return Err(Error {
                code: ErrorCode::ServerError(2),
                message: t!("error.document_read"),
                data: None,
            }),
        };

        if document.language_id != LANG_ID {
            return Err(Error {
                code: ErrorCode::ServerError(3),
                message: t!("error.invalid-lang_id"),
                data: None,
            });
        }

        let src = document.text;
        let (tokens_opt, _errors) = asm8051_parser::lexer::lexical_analysis(src);

        let tokens = match tokens_opt {
            Some(t) => t,
            None => return Err(Error {
                code: ErrorCode::ServerError(4),
                message: t!("error.lexical-analysis_failed"),
                data: None,
            }),
        };

        let _stringified_tokens = tokens.iter().map(|x| x.to_string()).collect::<Vec<_>>();

        if tokens.len() == 0 {
            return Ok(None);
        }

        let partial_map = tokens.iter()
            .map(|token| {
                let token_type: SemanticTokenType = match token.token.clone() {
                    Token::Keyword(kw) => match kw {
                        Keyword::Instruction(ins) => match ins {
                            Instruction::ACALL | Instruction::CALL | Instruction::LCALL => SemanticTokenType::FUNCTION,
                            _ => SemanticTokenType::KEYWORD,
                        },
                        Keyword::Register(_)  => SemanticTokenType::REGEXP,
                        Keyword::Directive(_) => SemanticTokenType::NAMESPACE,
                        Keyword::FlagOrBit(_) => SemanticTokenType::TYPE,
                    },
                    Token::Label(_) => SemanticTokenType::STRUCT,
                    Token::Address(_) => SemanticTokenType::ENUM_MEMBER,
                    Token::String(_) => SemanticTokenType::STRING,
                    Token::Number(_) => SemanticTokenType::NUMBER,
                    Token::ControlCharacter(cc) => match cc 
                    {
                        ControlCharacter::Arithmetic(_) => SemanticTokenType::OPERATOR,
                        ControlCharacter::AddressingModifier => SemanticTokenType::MODIFIER,
                        ControlCharacter::ArgumentSeparator => SemanticTokenType::OPERATOR,
                        ControlCharacter::AddressingSeparator => SemanticTokenType::MODIFIER,
                        ControlCharacter::ImmediateModifier => SemanticTokenType::NUMBER,
                        ControlCharacter::Parenthesis(_) => SemanticTokenType::OPERATOR,
                        ControlCharacter::Delimiter(d) => match d {
                            Delimiter::CommentStart => SemanticTokenType::COMMENT,
                            Delimiter::LabelEnd => SemanticTokenType::METHOD,
                            Delimiter::SingleQuote => SemanticTokenType::STRING,
                            Delimiter::DoubleQuote => SemanticTokenType::STRING,
                        },
                    } ,
                    Token::Trivia(tr) => match tr {
                        Trivia::Comment(_) => SemanticTokenType::COMMENT,
                        _ => SemanticTokenType::new("NONE"),
                    },
                    Token::Other(_) => SemanticTokenType::PROPERTY,
                    Token::Unknown(_) => SemanticTokenType::new("NONE"),
                };

                let line = (token.position.line) as u32;
                let column = (token.position.columns.start) as u32;
                let length = (token.position.columns.end - token.position.columns.start + 1) as u32;

                (token_type, line, column, length)
            })
            .filter(|x| x.0 != SemanticTokenType::new("NONE"))
            // .filter(|x| x.3 > 0)
            .collect::<Vec<_>>();

        if partial_map.len() == 0 {
            return Ok(None);
        }

        let to_semantic_token = |t: (SemanticTokenType, u32, u32, u32)| -> SemanticToken {
            let (token_type, delta_line, delta_start, length) = t;
            let token_type = self.map_token_type(token_type);

            SemanticToken { 
                delta_line, 
                delta_start, 
                length, 
                token_type, 
                token_modifiers_bitset: 0
            }
        };

        let mut semantic_tokens: Vec<SemanticToken> = vec![ to_semantic_token(partial_map[0].clone()) ];

        for i in 1..(partial_map.len()) {
            let prev = &partial_map[i - 1];
            let curr = &partial_map[i];

            semantic_tokens.push(to_semantic_token((
                    curr.0.clone(),
                    curr.1 - prev.1,
                    if curr.2 < prev.2 { curr.2 } else { curr.2 - prev.2 },
                    curr.3
                )));
        }

        Ok(Some(
            SemanticTokensResult::Tokens(
                SemanticTokens {
                    result_id: None,
                    data: semantic_tokens,
                }
            )))
    }

    

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client
            .log_message(MessageType::INFO, format!("received {}", params.command))
            .await;
        Ok(Option::None)
    }
}

impl Backend {
    async fn validate_all_documents(&self) {
        if self.documents.is_empty() {
            return;
        }

        for res in self.documents.iter() {
            self.validate_document(res.pair().1).await;
        }
    }

    /// This will be later used to sending diagnostics informations to the client
    async fn validate_document(&self, document: &TextDocumentItem) {
        let (kit, number_of_problems) = {
            let config = self.client_configuration.lock().await;
            (config.kit(), config.max_number_of_problems)
        };
        //self.get_client_configuration(document.borrow()).await;
        self.client
            .publish_diagnostics(
                document.clone().uri,
                diagnostics::get_diagnostics(document, kit, number_of_problems),
                None,
            )
            .await;
    }

    async fn update_configuration(&self) {
        let config = self.ask_for_configuration().await;
        try_update_mutex_value(self.client_configuration.borrow(), config).await;
    }

    /// Get new configuration from the client if it's capable of it
    async fn ask_for_configuration(&self) -> ClientConfiguration {
        // async fn ask_for_configuration(&self, uri: Url) -> ClientConfiguration {
        // can the client provide it's configuration

        if !has_configuration_capability(self.client_capabilities.lock().await.clone()) {
            return ClientConfiguration::default();
        }

        // ask client for a raw config
        let config_result = self
            .client
            .configuration(vec![ConfigurationItem {
                // scope_uri: Option::Some(uri),
                scope_uri: Option::None,
                section: Option::Some(String::from(LANG_ID)),
            }])
            .await;

        if config_result.is_err() {
            return ClientConfiguration::default();
        }

        let _config_vector = config_result.unwrap();
        let mut newconfig = ClientConfiguration::default();

        for config_value in _config_vector {
            let cnf: core::result::Result<ClientConfiguration, serde_json::error::Error> =
                serde_json::from_value(config_value.clone());
            if cnf.is_ok() {
                newconfig = cnf.unwrap();
            } else {
                self.client
                    .log_message(MessageType::INFO, cnf.unwrap_err())
                    .await;
            }
        }

        let new_configuration = ClientConfiguration {
            max_number_of_problems: newconfig.max_number_of_problems,
            kit: newconfig.kit,
        };

        new_configuration
    }

    pub async fn get_all_documentation(&self) -> Result<Option<Value>> {
        let locale = self.client_locale().await;
        let kit = self.ask_for_configuration().await.kit();

        let docs_option = docs::all_documentation(&locale);
        if docs_option.is_none() {
            return Ok(Option::None);
        }
        let docs: std::collections::HashMap<String, asm8051_shared::Documentation> = docs_option.unwrap();

        let mut already_added = Vec::<String>::new();
        let mut map = serde_json::Map::new();

        for (key, docs) in docs {

            if kit != Kits::DSM51 && docs.category.as_str() == Kits::DSM51.category_name() {
                continue;
            }

            let stack_space_needed = match docs.stack_space_needed {
                Some(space) => Some(format!(
                    "#### {}: {}",
                    t!("hover_StackSpaceNeeded"),
                    space
                )),
                None => None,
            };

            let obj = serde_json::json!({
                "detail": docs.detail,
                "description": docs.description,
                "syntax": hover::syntax((key.clone(), docs.clone())),
                "affected_flags": hover::generate_affected_flags(&docs.affected_flags),
                "valid_operands": hover::generate_valid_operands(&docs.valid_operands),
                "category": docs.category,
                "label": match docs.label { Some(value) => value, None => key.clone(), },
                "addressing_modes": hover::generate_addressing_modes(&docs.addressing_modes),
                "stack_space_needed": stack_space_needed,
                "used_registers": hover::generate_possible_registers(&docs.used_registers),
                "changed_registers": hover::generate_possible_registers(&docs.changed_registers),
            });

            if !docs.dont_duplicate_in_all_docs {
                map.insert(key, obj);
            } else if docs.dont_duplicate_in_all_docs && !already_added.contains(&docs.full_key)
            {
                map.insert(docs.full_key.clone(), obj);
                already_added.push(docs.full_key);
            }
        }

        Ok(Option::Some(serde_json::Value::Object(map)))
    }

    pub fn new(client: tower_lsp::Client) -> Backend {

        let token_vec = vec![
            SemanticTokenType::NUMBER,   
            SemanticTokenType::COMMENT,  
            SemanticTokenType::STRING,   
            SemanticTokenType::MACRO,    
            SemanticTokenType::OPERATOR, 
            SemanticTokenType::VARIABLE, 
            SemanticTokenType::MODIFIER, 
            SemanticTokenType::KEYWORD,  
            SemanticTokenType::METHOD,   
            SemanticTokenType::FUNCTION,
            SemanticTokenType::PROPERTY,
            SemanticTokenType::NAMESPACE,
            SemanticTokenType::ENUM_MEMBER,
            SemanticTokenType::VARIABLE,
            SemanticTokenType::TYPE,
            SemanticTokenType::REGEXP,
            SemanticTokenType::STRUCT,

        ];

        let token_map = DashMap::<SemanticTokenType, u32>::new();

        for i in 0..(token_vec.len()) {
            let token = (&token_vec[i]).clone();
            token_map.insert(token, i as u32);
        }
        

        Backend {
            client,
            documents: DashMap::new(),
            client_capabilities: Arc::new(Mutex::new(ClientCapabilities::default())),
            client_configuration: Arc::new(Mutex::new(ClientConfiguration::default())),
            client_locale: Arc::new(Mutex::new(Locale::ENGLISH)),
            semantic_token_map: token_map,
        }
    }

    async fn client_locale(&self) -> Locale {
        self.client_locale.lock().await.clone()
    }

    fn map_token_type(&self, token_type: SemanticTokenType) -> u32 {
        match self.semantic_token_map.get(&token_type) {
            Some(u) => u.clone(),
            None => panic!("Invalid SemanticTokenType: {}", token_type.as_str()),
        }
    }

}

fn has_configuration_capability(capabilities: ClientCapabilities) -> bool {
    if capabilities.workspace.is_none() {
        return false;
    }

    capabilities
        .workspace
        .as_ref()
        .unwrap()
        .configuration
        .unwrap_or(false)
}

async fn try_update_mutex_value<T>(current: &Mutex<T>, new: T) -> bool {
    let mut current_lock = current.lock().await;
    *current_lock = new;
    true
}
