//#region imports hell
use crate::{
    client_configuration::ClientConfiguration, diagnostics, flags::Locale, hover, LANG_ID,
};

use crate::localize;
use crate::i18n::change_language;

use asm8051_parser::lexer::tokens::{ControlCharacter, Delimiter};
use asm8051_parser::lexer::tokens::{Instruction, Trivia, Token, Keyword};

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

                // completion_provider: Some(CompletionOptions {
                //     resolve_provider: Option::from(true),
                //     trigger_characters: None,
                //     all_commit_characters: None,
                //     work_done_progress_options: Default::default(),
                // }),
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

        self.client
            .log_message(MessageType::INFO, localize!("server-initialized"))
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::INFO, "server shutdown!")
            .await;
        Ok(())
    }

    async fn did_change_configuration(&self, _params: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::INFO, localize!("configuration-changed"))
            .await;

        self.update_configuration().await;
        self.validate_all_documents().await;
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        self.client
            .log_message(MessageType::INFO, "completion!")
            .await;

        let _locale = self.client_locale().await;
        let documentation = hover::all_documentation(_locale);

        if documentation.is_none() {
            return Ok(None);
        }

        let mut completion = Vec::<CompletionItem>::new();
        for kvp in documentation.unwrap() {
            completion.push(CompletionItem::new_simple(kvp.0, kvp.1.detail));
        }

        Ok(Some(CompletionResponse::Array(completion)))
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        self.client.log_message(MessageType::INFO, "hover!").await;

        // get clients configuration
        let config = self.client_configuration.lock().await.clone();

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
                message: localize!("error-document-read"),
                data: None,
            });
        }

        let document = document.unwrap();

        //get documentation for whatever user is hovering over
        let doc = hover::documentation(
            _params.text_document_position_params.position,
            document.borrow(),
            config.borrow(),
            self.client_locale().await,
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

        if self.documents.contains_key(file_uri.borrow()) {
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
                message: localize!("error-document-read"),
                data: None,
            }),
        };

        if document.language_id != LANG_ID {
            return Err(Error {
                code: ErrorCode::ServerError(3),
                message: localize!("error-invalid-lang-id"),
                data: None,
            });
        }

        let src = document.text;
        let (tokens_opt, _errors) = asm8051_parser::lexer::lexical_analisys(src);

        let tokens = match tokens_opt {
            Some(t) => t,
            None => return Err(Error {
                code: ErrorCode::ServerError(4),
                message: localize!("error-lexical-analysis-failed"),
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
            self.validate_document(res.pair().1.borrow()).await;
        }
    }

    /// This will be later used to sending diagnostics informations to the client
    async fn validate_document(&self, document: &TextDocumentItem) {
        //self.get_client_configuration(document.borrow()).await;
        self.client
            .publish_diagnostics(
                document.clone().uri,
                diagnostics::get_diagnostics(document.borrow()),
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
        let _locale = self.client_locale().await;
        let docs_option = hover::all_documentation(_locale);
        if docs_option.is_none() {
            return Ok(Option::None);
        }
        let docs = docs_option.unwrap();

        let mut already_added = Vec::<String>::new();
        let mut map = serde_json::Map::new();

        for (key, docs) in docs {
            let obj = serde_json::json!({
                "detail": docs.detail,
                "description": docs.description,
                "syntax": hover::syntax((key.clone(), docs.clone())),
                "affected_flags": hover::generate_affected_flags(&docs.affected_flags),
                "valid_operands": hover::generate_valid_operands(&docs.valid_operands),
                "category": docs.category,
                "label": match docs.label { Some(value) => value, None => key.clone(), },
                "addressing_modes": hover::generate_addressing_modes(&docs.addressing_modes),
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