//#region imports hell
use crate::{diagnostics, hover_documentation, client_configuration::ClientConfiguration};
use lazy_static::__Deref;
use tower_lsp::{
    Client, LanguageServer,
    jsonrpc::{ Error, ErrorCode, Result },
    lsp_types::{ClientCapabilities, CompletionItem, CompletionOptions, CompletionParams, CompletionResponse,
        ConfigurationItem, DidChangeConfigurationParams, DidCloseTextDocumentParams,
        DidOpenTextDocumentParams, ExecuteCommandOptions, ExecuteCommandParams, Hover, HoverContents,
        HoverParams, HoverProviderCapability, InitializeParams, InitializeResult, InitializedParams,
        MessageType, Registration, TextDocumentItem, TextDocumentSyncCapability, TextDocumentSyncKind,
        Url,}
};

use serde_json::Value;
use tokio::sync::Mutex;
use std::{
    borrow::Borrow,
    collections::HashMap,
    option::Option,
    string::String,
    sync::Arc,
};
//#endregion imports

/// Connection with a client and additional configutation
#[derive(Debug)]
pub(crate) struct Backend {
    /// connection with LSP client
    pub(crate) client: Client,

    /// documents opened in editor indexed by uri to that document
    pub(crate) documents: Arc<Mutex<HashMap<String, TextDocumentItem>>>,

    /// things that client supports
    pub(crate) client_capabilities: Arc<Mutex<ClientCapabilities>>,

    /// configuration received from client
    pub(crate) client_configuration: Arc<Mutex<ClientConfiguration>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {

    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // updating capabilities of client
        try_update_mutex_value(self.client_capabilities.borrow(), params.capabilities).await;

        // time to set some capabilities, so the client knows what server can do
        let mut result = InitializeResult::default();

        // capability to execute commands
        result.capabilities.execute_command_provider = Some(ExecuteCommandOptions {
            commands: vec!["test.command".to_string()],
            work_done_progress_options: Default::default(),
        });

        // capability to synchronise documents
        result.capabilities.text_document_sync = Some(TextDocumentSyncCapability::from(
            TextDocumentSyncKind::INCREMENTAL,
        ));

        // capability to autocomplete
        result.capabilities.completion_provider = Some(CompletionOptions {
            resolve_provider: Option::from(true),
            trigger_characters: None,
            all_commit_characters: None,
            work_done_progress_options: Default::default(),
        });

        // capability to show documentation on hover
        result.capabilities.hover_provider = Option::from(HoverProviderCapability::Simple(true));

        Ok(result)
    }

    async fn initialized(&self, _params: InitializedParams) {
        println!("{}", crate::localize!("server-initialized"));

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
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        println!("server shutdown!");
        Ok(())
    }

    async fn did_change_configuration(&self, _params: DidChangeConfigurationParams) {
        println!("{}", crate::localize!("configuration-changed"));

        self.validate_all_documents().await;
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        println!("completion!");
        let _locale = self.client_configuration.lock().await.display_locale();
        let documentation = hover_documentation::all_documentation(_locale);

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
        println!("hover!");

        // get clients configuration
        let config = self.client_configuration.lock().await.clone();
        
        
        //load text of a document user id hovering over
        let uri = _params
            .text_document_position_params
            .text_document
            .uri
            .as_ref();

        let document: TextDocumentItem; 
        
        { // I don't want to hog that mutex for too long
            let documents = self.documents.lock().await;
            let doc = documents.get(uri);
            if doc.is_none() {
                return Err(Error {
                    code: ErrorCode::ServerError(002),
                    message: "An error occurred while reading document".to_string(),
                    data: None,
                });
            }

            document = doc.unwrap().clone();
        }
        
       

        //get documentation for whatever user is hovering over
        let doc = hover_documentation::documentation(
            _params.text_document_position_params.position,
            document.borrow(),
            config.borrow(),
        );

        Ok(Some(Hover {
            contents: HoverContents::Array(doc),
            range: None,
        }))
    }

    /// Add opened file to a local hashmap of opened files and validate it
    #[allow(unused_mut)]
    async fn did_open(&self, _params: DidOpenTextDocumentParams) {
        println!("file open");
        let file_uri = _params.text_document.uri.as_str();
        let file = _params.text_document.clone();

        self.documents
            .lock().await
            .insert(String::from(file_uri), file);

        self.validate_document(_params.text_document.borrow()).await;
    }

    /// remove file from local HashMap of opened files
    #[allow(unused_mut)]
    async fn did_close(&self, _params: DidCloseTextDocumentParams) {
        println!("file closed");
        let file_url = _params.text_document.uri.as_str();

        self.documents.lock().await.remove(file_url);
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client
            .log_message(MessageType::INFO, format!("received {}", params.command))
            .await;
        Ok(Option::None)
    }

    // async fn request_else(&self, method: &str, _params: Option<Value>) -> Result<Option<Value>> {
    //     println!("received request: {}", method);

    //     // self.client
    //     //     .log_message(MessageType::INFO, format!("received {}", method) )
    //     //     .await;
    //     match method {
    //         "documentation/getAll" => self.get_all_documentation().await,
    //         &_ => Err(lspower::jsonrpc::Error {
    //             code: ErrorCode::MethodNotFound,
    //             data: Option::None,
    //             message: String::from("Method not found"),
    //         }),
    //     }
    // }
}


impl Backend {
    async fn validate_all_documents(&self) {
        let documents_guard = self.documents.lock().await;
        let documents = documents_guard.deref();

        if documents.is_empty() {
            return;
        }
        
        let mut config: bool = true;

        for (_k, document) in documents {
            if config {
                try_update_mutex_value(
                    self.client_configuration.borrow(), 
                    self.ask_for_configuration(document.uri.clone()).await).await;

                config = false;
            }
            self.validate_document(document.borrow()).await;
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

    /// Get new configuration from the client if it's capable of it
    async fn ask_for_configuration(&self, uri: Url) -> ClientConfiguration {
        // can the client provide it's configuration
        
        if !has_configuration_capability(self.client_capabilities.lock().await.clone()) {
            return ClientConfiguration::default();
        }

        // ask client for a raw config
        let config_result = self
            .client
            .configuration(vec![ConfigurationItem {
                scope_uri: Option::Some(uri),
                section: Option::Some(String::from("asm8051")),
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
                println!("{}", cnf.unwrap_err());
            }
        }

        let ui_locale = self.client_configuration.lock().await.ui_locale.clone();
        

        let new_configuration = ClientConfiguration {
            max_number_of_problems: newconfig.max_number_of_problems,
            kit: newconfig.kit,
            locale: newconfig.locale,
            ui_locale: ui_locale,
        };

        let new_display_locale = new_configuration.display_locale().lang_name();

        crate::localizer()
            .select(&[new_display_locale.parse().unwrap()])
            .unwrap();

        new_configuration
    }

    
    pub async fn get_all_documentation(&self) -> Result<Option<Value>> {
        let _locale = self.client_configuration.lock().await.display_locale();
        let docs_option = hover_documentation::all_documentation(_locale);
        if docs_option.is_none() {
            return Ok(Option::None);
        }
        let _docs = docs_option.unwrap();
        //let dcs = serde_json::to_value(_docs).unwrap();

        let mut map = serde_json::Map::new();

        for pair in _docs {
            let obj = serde_json::json!({
                "detail": pair.1.detail,
                "description": pair.1.description,
                "syntax": hover_documentation::syntax(pair.clone()),
                "affected_flags": hover_documentation::generate_affected_flags(pair.clone().1.affected_flags),
                "valid_operands": hover_documentation::generate_valid_operands(pair.clone().1.valid_operands),
                "category": pair.1.category
            });
            map.insert(pair.0, obj);
        }

        Ok(Option::Some(serde_json::Value::Object(map)))
    }

    pub fn new(client: tower_lsp::Client) -> Backend {
        Backend {
            client,
            documents: Arc::new(Mutex::new(HashMap::new())),
            client_capabilities: Arc::new(Mutex::new(ClientCapabilities::default())),
            client_configuration: Arc::new(Mutex::new(ClientConfiguration::default())),
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