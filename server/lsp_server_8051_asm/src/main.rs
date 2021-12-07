#![deny(warnings)]

mod hover_documentation;
mod types;
mod diagnostics;

use std::borrow::Borrow;
use std::sync::{Mutex};
use std::collections::HashMap;
use lspower::jsonrpc::Result;
use lspower::{LanguageServer, LspService, Server};
use lspower::lsp::{CompletionItem, CompletionOptions, CompletionParams, CompletionResponse, ConfigurationItem, DidChangeConfigurationParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, Hover, HoverContents, HoverParams, HoverProviderCapability, InitializedParams, InitializeParams, InitializeResult, LanguageString, MarkedString, MessageType, Registration, TextDocumentItem, TextDocumentSyncCapability, TextDocumentSyncKind};
use types::{Backend, ClientConfiguration};


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
        let capabilities = self.client_capabilities.lock().unwrap().as_ref().unwrap().clone();

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

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        print!("completion!");
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string())
        ])))
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        print!("hover!");

        let mut md :Vec<MarkedString> = Vec::new();
        md.push(MarkedString::String(r#"Multiplies the unsigned values of the accumulator and the register B.

The result is a 16 bit unsigned number, lower half of which will be stored in the Accumulator, and the higher half in the B register.

---

Affected flags:
- **Carry** cleared
- **Overflow** set if result is greater than 255, cleared otherwise

---

Syntax:"#.to_string()));
        md.push(MarkedString::LanguageString(LanguageString { language: "asm8051".to_string(), value: "MUL AB".to_string() }));

        Ok(Some(Hover {
            contents: HoverContents::Array(md),
            range: None
        }))
    }

    async fn did_open(&self, _params: DidOpenTextDocumentParams) {
        println!("file open");
        let file_uri = _params.text_document.uri.as_str();
        let file = _params.text_document.clone();

        self.documents.lock().unwrap().insert(String::from(file_uri), file);
        self.validate_document(_params.text_document).await;
    }

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

    async fn validate_document(&self, document: TextDocumentItem) {
        self.get_settings(document.borrow()).await;
        print!("{}", document.uri);
        print!("{}\n\n", document.text);
        self.client.publish_diagnostics(document.clone().uri, diagnostics::get_diagnostics(document.borrow()), None).await;
    }

    async fn get_settings(&self, document: &TextDocumentItem){
        let has_configuration_capabilities = self.client_capabilities.lock().unwrap().as_ref().unwrap().workspace.as_ref().unwrap().configuration;
        if has_configuration_capabilities.is_none() || !has_configuration_capabilities.unwrap() {
            return;
        }

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

        for config_value in _config_vector {
            if !config_value.is_object() || config_value.as_object().is_none() {
                continue;
            }

            let config_object = config_value.as_object();

            let _config = ClientConfiguration::from_json_object(config_object);

        }
    }
}

