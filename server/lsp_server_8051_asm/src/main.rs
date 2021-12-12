#![deny(warnings)]

mod hover_documentation;
mod types;
mod diagnostics;

use std::borrow::Borrow;
use std::sync::{LockResult, Mutex, MutexGuard};
use std::collections::HashMap;
use lspower::jsonrpc::{Error, ErrorCode, Result};
use lspower::{LanguageServer, LspService, Server};
use lspower::lsp::{CompletionItem, CompletionOptions, CompletionParams, CompletionResponse, ConfigurationItem, DidChangeConfigurationParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, Hover, HoverContents, HoverParams, HoverProviderCapability, InitializedParams, InitializeParams, InitializeResult, LanguageString, MessageType, Registration, TextDocumentItem, TextDocumentSyncCapability, TextDocumentSyncKind};
use types::{Backend, ClientConfiguration};
use crate::types::Locale;


#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8050").await.unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    let (read, write) = tokio::io::split(stream);

    let (service, messages) = LspService::new(|client| Backend {
        client,
        documents: Mutex::new(HashMap::new()),
        locale: Mutex::new(None),
        client_capabilities: Mutex::new(None),
        client_configuration: Mutex::new(ClientConfiguration::default())
    });

    Server::new(read, write)
        .interleave(messages)
        .serve(service)
        .await;
}

#[lspower::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {

        // updating mutex with capabilities of client
        let mut capabilities = self.client_capabilities.lock().unwrap();
        *capabilities = Option::from(params.capabilities);

        // updating mutex with locale of client
        let mut locale = self.locale.lock().unwrap();
        *locale = params.locale;

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
        let has_configuration_capability = self.client_capabilities.lock().unwrap().as_ref().unwrap().workspace.as_ref().unwrap().configuration.unwrap();

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
        let config_lock = self.client_configuration.lock();
        if config_lock.is_err() {
            return Err(Error {
                code: ErrorCode::ServerError(001),
                message: "An error occurred while reading client configuration".to_string(),
                data: None
            });
        }

        //get list of opened documents
        let documents_lock = self.documents.lock();
        if documents_lock.is_err() {
            return Err(Error {
                code: ErrorCode::ServerError(002),
                message: "An error occurred while reading document".to_string(),
                data: None
            });
        }

        //load text of a document user id hovering over
        let uri = _params.text_document_position_params.text_document.uri.as_ref();
        let document = documents_lock.as_ref().unwrap().get(uri);
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
            config_lock.unwrap().borrow()
        );

        Ok(Some(Hover {
            contents: HoverContents::Array(doc),
            range: None
        }))
    }

    /// Add opened file to a local hashmap of opened files and validate it
    async fn did_open(&self, _params: DidOpenTextDocumentParams) {
        println!("file open");
        let file_uri = _params.text_document.uri.as_str();
        let file = _params.text_document.clone();

        self.documents.lock().unwrap().insert(String::from(file_uri), file);
        self.validate_document(_params.text_document).await;
    }

    /// remove file from local HashMap of opened files
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
            self.validate_document(value).await;
        }
    }

    /// This will be later used to sending diagnostics informations to the client
    async fn validate_document(&self, document: TextDocumentItem) {
        self.get_settings(document.borrow()).await;
        self.client.publish_diagnostics(document.clone().uri, diagnostics::get_diagnostics(document.borrow()), None).await;
    }

    /// Get new configuration from the client if it's capable of it
    async fn get_settings(&self, document: &TextDocumentItem){
        // can the client provide it's configuration
        let has_configuration_capabilities = self.client_capabilities.lock().unwrap().as_ref().unwrap().workspace.as_ref().unwrap().configuration;
        if has_configuration_capabilities.is_none() || !has_configuration_capabilities.unwrap() {
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
            if !config_value.is_object() || config_value.as_object().is_none() {
                continue;
            }

            // parse configuration
            let config_object = config_value.as_object();
            let cnf = ClientConfiguration::from_json_object(config_object);
            if cnf.is_some() {
                newconfig = cnf.unwrap();
            }
        }
        //get a hold of current configuration and locale of the client
        let clientconfig_lock: LockResult<MutexGuard<ClientConfiguration>> = self.client_configuration.lock();
        let ui_locale_lock = self.locale.lock();
        if clientconfig_lock.is_err() || ui_locale_lock.is_err() {
            return;
        }

        // update local copy of clients configuration
        let mut clientconfig = clientconfig_lock.unwrap();
        let new_config = ClientConfiguration {
            max_number_of_problems: newconfig.max_number_of_problems,
            kit: newconfig.kit,
            locale: newconfig.locale,
            ui_locale: match ui_locale_lock.unwrap().as_ref().unwrap().as_str() {
                "polski" => Locale::POLISH,
                &_ => Locale::ENGLISH,
            }
        };
        *clientconfig = new_config;
    }
}

