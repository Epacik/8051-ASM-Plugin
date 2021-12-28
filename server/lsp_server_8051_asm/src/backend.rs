use std::collections::HashMap;
use std::sync::{Mutex};
use crate::{diagnostics, hover_documentation};
use std::borrow::{Borrow};
use lspower::jsonrpc::{Error, ErrorCode, Result};
use lspower::{Client, LanguageServer};
use lspower::lsp::{ClientCapabilities, CompletionItem, CompletionOptions, CompletionParams, CompletionResponse, ConfigurationItem, DidChangeConfigurationParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, Hover, HoverContents, HoverParams, HoverProviderCapability, InitializedParams, InitializeParams, InitializeResult, MessageType, Registration, TextDocumentItem, TextDocumentSyncCapability, TextDocumentSyncKind};
use std::option::Option;
use std::string::String;
use crate::client_configuration::ClientConfiguration;


 

/// Connection with a client and additional configutation
#[derive(Debug)]
pub(crate) struct Backend {
    /// connection with LSP client
    pub(crate) client: Client,

    //I didn't feel like changing these mutexes yet lol
    
    /// documents opened in editor
    pub(crate) documents: Mutex<HashMap<String, TextDocumentItem>>,

    /// Locale of user interface
    pub(crate) locale: Mutex<Option<String>>,

    /// things that client supports
    pub(crate) client_capabilities: Mutex<Option<ClientCapabilities>>,

    /// configuration received from client
    pub(crate) client_configuration: Mutex<ClientConfiguration>,
}

#[lspower::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {

        // updating capabilities of client
        self.set_client_capabilities(Option::from(params.capabilities));

        // updating with locale of client
        self.set_locale(params.locale);

        // time to set some capabilities, so the client knows what server can do
        let mut result = InitializeResult::default();
        result.capabilities.text_document_sync = Some(TextDocumentSyncCapability::from(TextDocumentSyncKind::INCREMENTAL));
        result.capabilities.completion_provider = Some(CompletionOptions{
            resolve_provider: Option::from(true),
            trigger_characters: None,
            all_commit_characters: None,
            work_done_progress_options: Default::default()
        });
        result.capabilities.hover_provider = Option::from(HoverProviderCapability::Simple(true));

        Ok(result)
    }

    async fn initialized(&self, _params: InitializedParams) {
        print!("server initialized!");
        // get clients capabilities
        let capabilities = self.client_capabilities.lock().unwrap().as_ref().unwrap().clone();

        // add custom event in case user changes configuration in editor that supports it
        if capabilities.workspace.is_some() && capabilities.workspace.as_ref().unwrap().configuration.is_some() && capabilities.workspace.as_ref().unwrap().configuration.unwrap() {
            let _register_result = self.client.register_capability(vec![ Registration {
                id: "workspace/didChangeConfiguration".to_string(),
                method: "workspace/didChangeConfiguration".to_string(),
                register_options: None
            } ]).await;
        }

        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        print!("server shutdown!");
        Ok(())
    }

    async fn did_change_configuration(&self, _params: DidChangeConfigurationParams) {
        print!("Configuration has changed!");
        //let _result = self.client.configuration(vec![ConfigurationItem { scope_uri: Some(Url::try_from(".").unwrap()), section: Some("asm8051".to_string()) }]).await;
        let has_configuration_capability = self.client_capabilities.lock().unwrap().as_ref().unwrap().workspace.as_ref().unwrap().configuration.unwrap_or(false);

        if !has_configuration_capability {
            return;
        }

        self.validate_all_documents().await;
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        print!("completion!");
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string())
        ])))
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        print!("hover!");

        // get clients configuration
        let config = self.client_configuration.lock().unwrap();


        //get list of opened documents
        let documents = self.documents.lock().unwrap();

        //load text of a document user id hovering over
        let uri = _params.text_document_position_params.text_document.uri.as_ref();
        let document = documents.get(uri);
        if document.is_none() {
            return Err(Error {
                code: ErrorCode::ServerError(002),
                message: "An error occurred while reading document".to_string(),
                data: None
            });
        }

        //get documentation for whatever user is hovering over
        let doc = hover_documentation::get_documentation(
            _params.text_document_position_params.position,
            document.unwrap().borrow(),
            config.borrow()
        );

        Ok(Some(Hover {
            contents: HoverContents::Array(doc),
            range: None
        }))
    }


    /// Add opened file to a local hashmap of opened files and validate it
    #[allow(unused_mut)]
    async fn did_open(&self, _params: DidOpenTextDocumentParams) {
        println!("file open");
        let file_uri = _params.text_document.uri.as_str();
        let file = _params.text_document.clone();

        self.documents.lock().unwrap().insert(String::from(file_uri), file);

        self.validate_document(_params.text_document.borrow()).await;
    }

    /// remove file from local HashMap of opened files
    #[allow(unused_mut)]
    async fn did_close(&self, _params: DidCloseTextDocumentParams) {
        println!("file closed");
        let file_url = _params.text_document.uri.as_str();

        self.documents.lock().unwrap().remove(file_url);
    }
}

impl Backend{
    async fn validate_all_documents(&self) {
        let documents = self.documents.lock().unwrap().clone();

        for (_key, value) in documents {
            self.validate_document(value.borrow()).await;
        }
    }

    /// This will be later used to sending diagnostics informations to the client
    async fn validate_document(&self, document: &TextDocumentItem) {
        self.get_settings(document.borrow()).await;
        self.client.publish_diagnostics(document.clone().uri, diagnostics::get_diagnostics(document.borrow()), None).await;
    }

    /// Get new configuration from the client if it's capable of it
    async fn get_settings(&self, document: &TextDocumentItem){
        // can the client provide it's configuration
        let has_configuration_capabilities = self.client_capabilities.lock().unwrap().as_ref().unwrap()
            .workspace.as_ref().unwrap()
            .configuration.unwrap_or(false);
        if !has_configuration_capabilities{
            return;
        }

        // download raw config
        let config_result = self.client.configuration(vec![
            ConfigurationItem{
                scope_uri: Option::Some(document.uri.clone()),
                section: Option::Some(String::from("asm8051"))
            }
        ]).await;

        if config_result.is_err() {
            return;
        }


        let _config_vector = config_result.unwrap();
        let mut newconfig = ClientConfiguration::default();

        for config_value in _config_vector {

            let cnf: core::result::Result<ClientConfiguration, serde_json::error::Error> = serde_json::from_value(config_value.clone());
            if cnf.is_ok() {
                newconfig = cnf.unwrap();
            }
            else {
                print!("{}", cnf.unwrap_err());
            }
        }

        let ui_locale = self.locale.lock().unwrap().as_ref()
            .unwrap_or(&String::from("english")).to_string();


        // update local copy of clients configuration
        self.set_client_configuration( ClientConfiguration{
            max_number_of_problems: newconfig.max_number_of_problems,
            kit: newconfig.kit,
            locale: newconfig.locale,
            ui_locale: ui_locale,
        });
    }

    fn set_client_capabilities(&self, new_capabilities: Option<ClientCapabilities>){
        let mut capabilities = self.client_capabilities.lock().unwrap();
        *capabilities = new_capabilities;
    }

    fn set_locale(&self, new_locale: Option<String>){
        let mut locale = self.locale.lock().unwrap();
        *locale = new_locale;
    }

    fn set_client_configuration(&self, new_config: ClientConfiguration ){
        let mut config = self.client_configuration.lock().unwrap();
        *config = new_config;
    }

}